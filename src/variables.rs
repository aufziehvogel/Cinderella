use std::collections::HashMap;
use std::path::PathBuf;

use toml;

use crate::config::Configs;
use crate::crypto;

pub fn load(workdir: &PathBuf, configs: &Configs) -> HashMap<String, String> {
    let mut variables = HashMap::new();

    variables.extend(load_internal(configs));
    variables.extend(load_secrets_from_file(workdir, configs));

    variables
}

fn load_internal(configs: &Configs) -> HashMap<String, String> {
    let mut variables = HashMap::new();

    if let Some(branch) = &configs.execution_config.branch {
        variables.insert("branch".to_string(), branch.to_string());
    }

    variables
}

fn load_secrets_from_file(workdir: &PathBuf, configs: &Configs)
    -> HashMap<String, String>
{
    let mut variables = HashMap::new();

    let secrets_file = configs.execution_config.secrets_file(workdir);

    // TODO: If secrets not defined output that not defined?
    if let Some(secrets) = &configs.cinderella_config.secrets {
        let password = &secrets.password;
        let decrypted = crypto::decrypt_file(&secrets_file, &password);

        if let Ok(secrets_content) = decrypted {
            variables.extend(load_secrets(&secrets_content));
        };
    }

    variables
}

fn load_secrets(toml_definition: &str) -> HashMap<String, String>
{
    toml::from_str(toml_definition).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_secrets() {
        let config = "USERNAME = \"my-user\"\nPASSWORD = \"my-pass\"";
        let variables = load_secrets(config);

        assert_eq!(variables["USERNAME"], "my-user");
        assert_eq!(variables["PASSWORD"], "my-pass");
    }
}
