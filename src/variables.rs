use std::collections::HashMap;

pub fn load(branch: &Option<String>) -> HashMap<String, String> {
    let mut variables = HashMap::new();

    if let Some(branch) = &branch {
        variables.insert("branch".to_string(), branch.to_string());
    }

    variables
}
