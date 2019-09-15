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
