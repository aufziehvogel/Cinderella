use std::fs::File;

use serde::{Serialize, Deserialize};
use rmp_serde;

use sodiumoxide::crypto::secretbox::{self, Key, Nonce};
use sodiumoxide::crypto::pwhash::{self, Salt};

#[derive(Serialize, Deserialize)]
pub struct CryptoFile {
    pwsalt: Salt,
    nonce: Nonce,
    content: Vec<u8>,
}

pub struct SaltedKey {
    pwsalt: Salt,
    key: Key,
}

pub fn gen_key_from_pw(password: &str) -> SaltedKey {
    let salt = pwhash::gen_salt();

    // cf. https://docs.rs/sodiumoxide/0.2.5/sodiumoxide/crypto/pwhash/index.html#example-key-derivation
    let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    {
        let secretbox::Key(ref mut kb) = k;
        pwhash::derive_key(kb, password.as_bytes(), &salt,
                           pwhash::OPSLIMIT_INTERACTIVE,
                           pwhash::MEMLIMIT_INTERACTIVE).unwrap();

        SaltedKey {
            key: Key::from_slice(kb).unwrap(),
            pwsalt: salt
        }
    }
}

pub fn encrypt_string(plaintext: &str, password: &str) -> Vec<u8> {
    let salted_key = gen_key_from_pw(password);
    let nonce = secretbox::gen_nonce();
    let ciphertext = secretbox::seal(plaintext.as_bytes(),
                                     &nonce, &salted_key.key);

    let f = CryptoFile {
        nonce,
        pwsalt: salted_key.pwsalt,
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
