use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};

fn get_active_window_title() -> String {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        
        if hwnd.is_invalid() {
            return String::from("No active window");
        }
        
        let mut buffer = vec![0u16; 512];
        let length = GetWindowTextW(hwnd, &mut buffer);
        
        if length == 0 {
            return String::from("(Untitled)");
        }
        
        // Convert UTF-16 to String
        String::from_utf16_lossy(&buffer[..length as usize])
    }
}

fn main() {
    println!("Active Window Monitor started. Press Ctrl+C to stop.");
    
    loop {
        let title = get_active_window_title();
        println!("Active Window: {}", title);
        
        thread::sleep(Duration::from_secs(1));
    }
}
