use std::fs::File;
use std::path::Path;

use serde::{Serialize, Deserialize};
use rmp_serde;

use sodiumoxide::crypto::secretbox::{self, Key, Nonce};
use sodiumoxide::crypto::pwhash::{self, Salt};

#[derive(Serialize, Deserialize)]
struct CryptoFile {
    pwsalt: Salt,
    nonce: Nonce,
    content: Vec<u8>,
}

struct SaltedKey {
    pwsalt: Salt,
    key: Key,
}

fn gen_salted_key(password: &str, salt: Salt) -> SaltedKey {
    // cf. https://docs.rs/sodiumoxide/0.2.5/sodiumoxide/crypto/pwhash/index.html#example-key-derivation
    let mut k = secretbox::Key([0; secretbox::KEYBYTES]);
    {
        let secretbox::Key(ref mut kb) = k;
        pwhash::derive_key(kb, password.as_bytes(), &salt,
                           pwhash::OPSLIMIT_INTERACTIVE,
                           pwhash::MEMLIMIT_INTERACTIVE).unwrap();

        SaltedKey {
            key: k,
            pwsalt: salt
        }
    }
}

fn gen_salted_key_random_salt(password: &str) -> SaltedKey {
    let salt = pwhash::gen_salt();
    gen_salted_key(password, salt)
}

pub fn encrypt_string(plaintext: &str, password: &str) -> Vec<u8> {
    let salted_key = gen_salted_key_random_salt(password);
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

pub fn decrypt_file(filepath: &Path, password: &str) -> Result<String, ()> {
    let r = File::open(filepath).unwrap();
    let f: CryptoFile = rmp_serde::from_read(r).unwrap();
    let salted_key = gen_salted_key(password, f.pwsalt);

    secretbox::open(&f.content, &f.nonce, &salted_key.key)
        .map(|bytes| String::from_utf8(bytes).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_encrypt_then_decrypt() {
        let plaintext = "my-secret-string";

        let ciphertext = encrypt_string(plaintext, "my-pass");

        // write the ciphertext to a file
        let mut tmpfile = NamedTempFile::new().unwrap();
        let f = tmpfile.as_file_mut();
        f.write_all(&ciphertext).expect("Unable to write to file");

        let decrypted = decrypt_file(tmpfile.path(), "my-pass").unwrap();

        assert_eq!(plaintext, decrypted);
    }
}
