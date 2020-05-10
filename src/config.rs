use std::fs;
use std::path::PathBuf;
use std::vec::Vec;

use serde::Deserialize;
use toml;

pub struct Configs<'a> {
    pub cinderella_config: &'a CinderellaConfig,
    pub execution_config: &'a ExecutionConfig,
}

#[derive(Deserialize, Debug)]
pub struct CinderellaConfig {
    pub email: Option<Email>,
    pub secrets: Option<Secrets>,
    pub dashboard: Option<Dashboard>,
}

#[derive(Deserialize, Debug)]
pub struct Email {
    pub server: String,
    pub user: String,
    pub password: String,
    pub from: String,
    pub to: String,
}

#[derive(Deserialize, Debug)]
pub struct Secrets {
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Dashboard {
    pub folder: String,
}

impl CinderellaConfig {
    pub fn from_file(path: PathBuf) -> CinderellaConfig {
        match fs::read_to_string(path) {
            Ok(contents) => {
                toml::from_str(&contents).expect("Configuration invalid")
            },
            _ => CinderellaConfig {
                email: None,
                secrets: None,
                dashboard: None,
            }
        }
    }
}

pub struct ExecutionConfig {
    pub repo_url: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub cinderella_filepath: Option<String>,
}

impl ExecutionConfig {
    // TODO: This approach only works for URLs, not for local paths.
    // TODO: Move the name() function to the CodeSource
    pub fn name(&self) -> String {
        let components: Vec<&str> = self.repo_url.split('/').collect();

        // TODO: Make more Rusty
        // TODO: Always use the canonical path for getting project name? e.g.
        // when user defines "." as path, still use the folder name
        if components.last().is_some() && components.last().unwrap().to_string() != "" {
            return components.last().unwrap().to_string();
        } else if components.len() >= 2 {
            return components[components.len() - 2].to_string();
        } else {
            return "".to_string();
        }
    }

    pub fn cinderella_file(&self, folder: &PathBuf) -> PathBuf {
        let filepath = match &self.cinderella_filepath {
            Some(filepath) => PathBuf::from(filepath),
            None => {
                let mut cinderella_file = folder.clone();
                cinderella_file.push(".cinderella.toml");
                cinderella_file
            },
        };

        filepath
    }

    pub fn secrets_file(&self, folder: &PathBuf) -> PathBuf {
        let mut secrets_file = folder.clone();
        secrets_file.push(".cinderella");
        secrets_file.push("secrets");
        secrets_file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_project_name_from_path() {
        let config = ExecutionConfig {
            repo_url: "/path/to/repo".to_string(),
            branch: Some("master".to_string()),
            tag: None,
            cinderella_filepath: None,
        };
        assert_eq!(config.name(), "repo");

        // even if the path ends with a slash, we should extract the right
        // project name
        let config = ExecutionConfig {
            repo_url: "/path/to/repo.git/".to_string(),
            branch: Some("master".to_string()),
            tag: None,
            cinderella_filepath: None,
        };
        assert_eq!(config.name(), "repo.git");
    }

    #[test]
    fn test_load_valid_config() {
        let config = r#"
            [email]
            server = "localhost"
            user = "user"
            password = "s"
            to = "to@example.com"
            from = "from@example.com"

            [dashboard]
            folder = "/var/www/cinderella"
        "#;
        let mut tmpfile = NamedTempFile::new().unwrap();
        let f = tmpfile.as_file_mut();
        f.write_all(config.as_bytes()).expect("Unable to write to file");

        let config = CinderellaConfig::from_file(tmpfile.path().to_path_buf());

        let email = config.email.unwrap();
        assert_eq!(email.server, "localhost");
        assert_eq!(email.user, "user");
        assert_eq!(email.password, "s");
        assert_eq!(email.to, "to@example.com");
        assert_eq!(email.from, "from@example.com");

        let dashboard = config.dashboard.unwrap();
        assert_eq!(dashboard.folder, "/var/www/cinderella");
    }

    #[test]
    fn test_can_handle_missing_config() {
        let mut path = PathBuf::new();
        path.push("/tmp/some/invalid/path/config.toml");

        let config = CinderellaConfig::from_file(path);

        assert!(config.email.is_none());
    }

    #[test]
    fn test_secrets_file_path() {
        // this test exists to ensure that we recognize when the expected
        // path to the secrets file changes, so that we can mention this in
        // the change notes

        let exec_config = ExecutionConfig {
            repo_url: String::from("https://example.com/my-repo.git"),
            branch: Some(String::from("master")),
            tag: None,
            cinderella_filepath: None,
        };

        let base_path = PathBuf::from("/tmp/work-dir");
        let secrets_file = exec_config.secrets_file(&base_path);

        assert_eq!(
            secrets_file,
            PathBuf::from("/tmp/work-dir/.cinderella/secrets")
        );
    }
}
