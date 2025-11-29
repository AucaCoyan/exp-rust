use anyhow::Context;
use prometheus::{IntCounterVec, Registry, TextEncoder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, ffi::OsString, os::windows::ffi::OsStringExt, path::PathBuf};
use tokio::{
    signal,
    time::{self, Duration},
};
use windows::Win32::{
    Foundation::{CloseHandle, HWND, MAX_PATH},
    System::ProcessStatus::K32GetModuleBaseNameW,
    System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId},
};

const PERSISTENCE_FILE: &str = "metrics_state.json";
const SAVE_INTERVAL_SECS: u64 = 30; // Save to disk every 30 seconds
const PUSH_INTERVAL_SECS: u64 = 15; // Push to Pushgateway every 15 seconds
const DEFAULT_PUSHGATEWAY_HOST: &str = "localhost";
const DEFAULT_PUSHGATEWAY_PORT: u16 = 9091;
const DEFAULT_JOB_NAME: &str = "windows_active_window";

/// Gets the executable name
///
/// Samples: "chrome.exe", "code.exe"
/// From the PID, opens the process with read permissions
/// and returns the parent *.exe
fn get_executable_name(hwnd: HWND) -> Option<String> {
    unsafe {
        if hwnd.is_invalid() {
            return None;
        }

        // Step 1: Get the process ID
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        if process_id == 0 {
            return None;
        }

        // Step 2: Open the process with query and read permissions
        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        )
        .ok()?;

        // Step 3: Get the module base name (executable name)
        let mut buffer = vec![0u16; MAX_PATH as usize];
        let length = K32GetModuleBaseNameW(process_handle, None, &mut buffer);

        let _ = CloseHandle(process_handle);

        if length == 0 {
            return None;
        }

        // Convert UTF-16 to String and lowercase it for consistency
        let executable = OsString::from_wide(&buffer[..length as usize])
            .to_string_lossy()
            .to_lowercase();

        Some(executable)
    }
}

/// Gets the executable name of the currently active foreground window
fn get_active_window_executable() -> Option<String> {
    unsafe {
        let hwnd = GetForegroundWindow();
        get_executable_name(hwnd)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct MetricsState {
    /// Maps executable name to total seconds
    executables: HashMap<String, u64>,
}

/// Save metrics state to disk
async fn save_metrics_state(registry: &Registry, path: &PathBuf) -> anyhow::Result<()> {
    let mut state = MetricsState {
        executables: HashMap::new(),
    };

    // Collect all counter values by executable name from the registry
    let metric_families = registry.gather();
    for family in metric_families {
        if family.get_name() == "windows_active_seconds_total" {
            for metric in family.get_metric() {
                // Find the executable label
                for label in metric.get_label() {
                    if label.get_name() == "executable" {
                        let exec_name = label.get_value().to_string();
                        let value = metric.get_counter().get_value() as u64;
                        state.executables.insert(exec_name, value);
                        break;
                    }
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&state).context("Failed to serialize metrics state")?;
    tokio::fs::write(path, json)
        .await
        .context("Failed to write metrics state file")?;

    Ok(())
}

/// Load metrics state from disk and restore counter values
async fn load_metrics_state(counter: &IntCounterVec, path: &PathBuf) -> anyhow::Result<()> {
    if !path.exists() {
        return Ok(()); // No previous state, start fresh
    }

    let content = tokio::fs::read_to_string(path)
        .await
        .context("Failed to read metrics state file")?;
    let state: MetricsState =
        serde_json::from_str(&content).context("Failed to parse metrics state file")?;

    // Restore counter values
    // Prometheus counters are cumulative, so we restore the previous total
    for (exec_name, saved_value) in state.executables {
        let current = counter.with_label_values(&[&exec_name]).get();
        let diff = saved_value.saturating_sub(current);
        if diff > 0 {
            // Increment by the difference to restore the cumulative value
            counter.with_label_values(&[&exec_name]).inc_by(diff);
        }
    }

    Ok(())
}

/// Push metrics to Prometheus Pushgateway
async fn push_metrics_to_gateway(
    registry: &Registry,
    pushgateway_url: &str,
    job_name: &str,
) -> anyhow::Result<()> {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    let metrics_body = encoder
        .encode_to_string(&metric_families)
        .context("Failed to encode metrics")?;

    let push_url = format!("{}/metrics/job/{}", pushgateway_url, job_name);

    let client = reqwest::Client::new();
    let response = client
        .put(&push_url)
        .body(metrics_body)
        .header("Content-Type", "text/plain; version=0.0.4")
        .send()
        .await
        .context("Failed to send metrics to Pushgateway")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Pushgateway returned error {}: {}", status, body);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Active Window Monitor started. Tracking executable names...");

    let state_path = PathBuf::from(PERSISTENCE_FILE);

    // ----- Prometheus setup -----
    let registry = Registry::new();
    let active_counter = IntCounterVec::new(
        prometheus::Opts::new(
            "windows_active_seconds_total",
            "Total seconds spent in each foreground window executable",
        ),
        &["executable"],
    )?;
    registry.register(Box::new(active_counter.clone()))?;

    // Load previous state from disk
    if let Err(e) = load_metrics_state(&active_counter, &state_path).await {
        eprintln!("Warning: Failed to load previous metrics state: {}", e);
        println!("Starting with fresh metrics...");
    } else {
        println!("Restored metrics state from disk");
    }

    // ----- Pushgateway configuration -----
    // Get configuration from environment variables or use defaults
    let pushgateway_host =
        env::var("PUSHGATEWAY_HOST").unwrap_or_else(|_| DEFAULT_PUSHGATEWAY_HOST.to_string());
    let pushgateway_port = env::var("PUSHGATEWAY_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PUSHGATEWAY_PORT);
    let job_name = env::var("JOB_NAME").unwrap_or_else(|_| DEFAULT_JOB_NAME.to_string());

    let pushgateway_url = format!("http://{}:{}", pushgateway_host, pushgateway_port);
    println!("Pushgateway URL: {}", pushgateway_url);
    println!("Job name: {}", job_name);
    println!("Configure via environment variables: PUSHGATEWAY_HOST, PUSHGATEWAY_PORT, JOB_NAME");

    // Spawn periodic save task
    let registry_for_save = registry.clone();
    let path_for_save = state_path.clone();
    let save_handle = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(SAVE_INTERVAL_SECS));
        loop {
            interval.tick().await;
            if let Err(e) = save_metrics_state(&registry_for_save, &path_for_save).await {
                eprintln!("Error saving metrics state: {e}");
            } else {
                println!("Metrics state saved to disk");
            }
        }
    });

    // Spawn periodic push task
    let registry_for_push = registry.clone();
    let push_url = pushgateway_url.clone();
    let job_name_for_push = job_name.clone();
    let push_handle = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(PUSH_INTERVAL_SECS));
        loop {
            interval.tick().await;
            if let Err(e) =
                push_metrics_to_gateway(&registry_for_push, &push_url, &job_name_for_push).await
            {
                eprintln!("Error pushing metrics to Pushgateway: {e}");
            } else {
                println!("Metrics pushed to Pushgateway");
            }
        }
    });

    // ----- Sampling loop -----
    let mut interval = time::interval(Duration::from_secs(1));

    tokio::select! {
        _ = async move {
            loop {
                interval.tick().await;
                if let Some(executable) = get_active_window_executable() {
                    active_counter.with_label_values(&[&executable]).inc();
                }
            }
        } => {
            // Do nothing
        },
        _ = signal::ctrl_c() => {
            println!("\nShutting down gracefully...");
        },
    }

    // Save final state and push final metrics before shutdown
    println!("Saving final metrics state...");
    if let Err(e) = save_metrics_state(&registry, &state_path).await {
        eprintln!("Error saving final metrics state: {e}");
    }

    println!("Pushing final metrics to Pushgateway...");
    if let Err(e) = push_metrics_to_gateway(&registry, &pushgateway_url, &job_name).await {
        eprintln!("Error pushing final metrics: {e}");
    }

    // Cancel background tasks
    save_handle.abort();
    push_handle.abort();

    println!("Shutdown complete");
    Ok(())
}
