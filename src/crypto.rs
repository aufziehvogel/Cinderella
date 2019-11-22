use std::fs::File;

use serde::{Serialize, Deserialize};
use rmp_serde;

use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::xsalsa20poly1305::Nonce;

#[derive(Serialize, Deserialize)]
pub struct CryptoFile {
    nonce: Nonce,
    content: Vec<u8>,
}

pub fn encrypt_string(plaintext: &str) -> Vec<u8> {
    let key = secretbox::gen_key();
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(plaintext.as_bytes(), &nonce, &key);

    let f = CryptoFile {
        nonce,
        content: ciphertext,
    };

    rmp_serde::to_vec(&f).unwrap()
}

pub fn decrypt_file(filepath: &str) -> String {
    let r = File::open(filepath).unwrap();
    let f: CryptoFile = rmp_serde::from_read(r).unwrap();
    let key = secretbox::gen_key();

    let bytes = secretbox::open(&f.content, &f.nonce, &key).unwrap();
    String::from_utf8(bytes).unwrap()
}
