use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

use assert_cmd::prelude::*;
use tempfile;
use assert_cmd;

#[test]
fn test_encrypt_decrypt() {
    let dir = tempfile::tempdir().unwrap();
    let cinderella_dir = dir.path().join(".cinderella");
    let secrets_file = cinderella_dir.join("secrets.toml");

    fs::create_dir(cinderella_dir).unwrap();
    let mut file = File::create(&secrets_file).unwrap();
    writeln!(file, "MY_SECRET = \"secret\"").unwrap();

    Command::cargo_bin("cinderella").unwrap()
        .current_dir(&dir)
        .arg("encrypt")
        .arg("-p")
        .arg("my-pass")
        .output()
        .expect("Encryption failed");

    fs::remove_file(&secrets_file).expect("Could not delete plaintext");

    Command::cargo_bin("cinderella").unwrap()
        .current_dir(&dir)
        .arg("decrypt")
        .arg("-p")
        .arg("my-pass")
        .output()
        .expect("Decryption failed");

    let res = fs::read_to_string(&secrets_file).unwrap();

    assert_eq!("MY_SECRET = \"secret\"", res.trim());
}

#[test]
fn test_environment_variables_get_replaced() {
    let dir = tempfile::tempdir().unwrap();
    let cinderella_file = dir.path().join(".cinderella.toml");

    let mut file = File::create(&cinderella_file).unwrap();
    writeln!(file, "[test]\ncommands = [\"echo $MY_ENV_VAR\"]").unwrap();

    // Create a git repo, TODO: Would be nicer if we could run cinderella
    // also on folders without having a git repo
    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .output()
        .expect("Creation of git repo failed");

    let output = Command::cargo_bin("cinderella").unwrap()
        .args(vec!["run", "-f", &cinderella_file.to_string_lossy(), "."])
        .current_dir(&dir)
        .env("MY_ENV_VAR", "test-env-var")
        .output()
        .expect("Execution failed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("test-env-var"));
}
