use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub email: Option<Email>,
}

#[derive(Deserialize, Debug)]
pub struct Email {
    pub server: String,
    pub user: String,
    pub password: String,
    pub from: String,
    pub to: String,
}

pub fn read_config(path: PathBuf) -> Config {
    // TODO: Use path relative to binary, not to CWD
    match fs::read_to_string(path) {
        Ok(contents) => {
            toml::from_str(&contents).expect("Configuration invalid")
        },
        _ => Config {
            email: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
        let config = r#"
            [email]
            server = "localhost"
            user = "user"
            password = "s"
            to = "to@example.com"
            from = "from@example.com"
        "#;
        let mut tmpfile = NamedTempFile::new().unwrap();
        let f = tmpfile.as_file_mut();
        f.write_all(config.as_bytes()).expect("Unable to write to file");

        let config = read_config(tmpfile.path().to_path_buf());

        let email = config.email.unwrap();
        assert_eq!(email.server, "localhost");
        assert_eq!(email.user, "user");
        assert_eq!(email.password, "s");
        assert_eq!(email.to, "to@example.com");
        assert_eq!(email.from, "from@example.com");
    }

    #[test]
    fn test_can_handle_missing_config() {
        let mut path = PathBuf::new();
        path.push("/tmp/some/invalid/path/config.toml");

        let config = read_config(path);

        assert!(config.email.is_none());
    }
}
