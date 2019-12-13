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
