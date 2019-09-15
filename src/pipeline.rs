use std::fs;
use std::path::PathBuf;
use toml::Value;

#[derive(Debug)]
pub struct Pipeline {
    pub name: String,
    pub commands: Vec<String>,
    pub when: Option<String>,
}

pub fn load_pipeline(path: &PathBuf) -> Option<Vec<Pipeline>> {
    if let Ok(contents) = fs::read_to_string(path) {
        let data = contents.parse::<Value>().unwrap();

        let res: Vec<Pipeline> = data.as_table().unwrap().iter()
            .filter_map(|(key, value)| {
                match value {
                    Value::Table(table) => {
                        // TODO: Better error handling needed
                        Some(Pipeline {
                            name: key.to_string(),
                            commands: value["commands"].as_array().unwrap().iter()
                                .map(|cmd| String::from(cmd.as_str().unwrap()))
                                .collect(),
                            when: table.get("when").map(|v| String::from(v.as_str().unwrap())),
                        })
                    },
                    _ => None
                }
            }).collect();

        Some(res)
    } else {
        // TODO: Should we differentiate more? Like:
        // - file does not exist: None
        // - file does exist, but cannot be read: Error
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() {
        let config = "[my-test]\ncommands = [\"echo Hallo\"]";
        let mut tmpfile = NamedTempFile::new().unwrap();
        let f = tmpfile.as_file_mut();
        f.write_all(config.as_bytes()).expect("Unable to write to file");

        let pipelines = load_pipeline(&tmpfile.path().to_path_buf());
        let pipelines = pipelines.unwrap();

        assert_eq!(pipelines.len(), 1);
        assert_eq!(pipelines[0].name, "my-test");
        assert_eq!(pipelines[0].commands.len(), 1);
        assert_eq!(pipelines[0].commands[0], "echo Hallo");
    }

    #[test]
    fn test_none_on_missing_config() {
        let mut path = PathBuf::new();
        path.push("/tmp/some/invalid/path/cinderella.toml");

        let pipelines = load_pipeline(&path);

        assert!(pipelines.is_none());
    }
}
