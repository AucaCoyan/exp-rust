use assert_cmd::Command;
use std::{fs::File, io::Write};

/// `cargo run -- ./Cargo.toml ./src/main.rs`
///
/// only checks that it doesn't return an error when given 2 files
#[test]
fn takes_more_than_1_arg() {
    // TODO: create a file instead of reading one in the repo
    let mut cmd = Command::cargo_bin("clap-maybestdin").unwrap();
    dbg!(cmd.arg("/src/main.rs").arg("Cargo.toml").assert().success());
}

/// `cargo run -- tests.txt` returns "Ok("Hello World!")"
#[test]
fn pipe_file_to_stdin() {
    let mut file = File::create("tests.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    let mut cmd = Command::cargo_bin("clap-maybestdin").unwrap();
    cmd.arg("-")
        .pipe_stdin("tests.txt")
        .unwrap()
        .assert()
        .stdout(b"Ok(\"Hello, world!\")\n" as &[u8]);
}

/// `cargo run -- tests.txt` returns "Ok("Hello World!")"
#[test]
fn pipe_folder_to_stdin() {
    // TODO: create a file instead of reading one in the repo
    let mut file = File::create("tests.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    let mut cmd = Command::cargo_bin("clap-maybestdin").unwrap();
    cmd.arg("src/");
}
