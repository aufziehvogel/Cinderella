use std::collections::HashMap;
use std::process::Command;
use std::io::Write;

use crate::pipeline;

pub fn execute<W: Write>(
    pipelines: &Vec<pipeline::Pipeline>,
    variables: &HashMap<String, String>,
    stdout: &mut W)
{
    for pipeline in pipelines {
        let execute = match &pipeline.when {
            Some(when) => {
                let test = replace_variables(&when, &variables);
                execute_test(&test)
            }
            None => true,
        };

        if execute {
            execute_pipeline(pipeline, &variables, stdout);
        }
    }
}

fn execute_pipeline<W: Write>(
    pipeline: &pipeline::Pipeline,
    variables: &HashMap<String, String>,
    stdout: &mut W)
{
    writeln!(stdout, "Executing pipeline \"{}\"\n", pipeline.name).unwrap();

    for cmd in &pipeline.commands {
        writeln!(stdout, "Step: {}", cmd).unwrap();

        let cmd = replace_variables(&cmd, &variables);
        // TODO: Raise error if some variables remain unsubstituted?

        let parts = split_command(&cmd);
        let output = Command::new(parts[0])
            .args(&parts[1..])
            .output()
            .expect(&format!("failed to run {}", cmd));

        stdout.write_all(&output.stdout).unwrap();
        assert!(output.status.success());
    }
}

fn split_command<'a>(command: &'a str) -> Vec<&'a str> {
    // TODO: Successful argument parsing needs a lot more details,
    // e.g. for quoted arguments like myprogram "argument 1"
    // but for a first shot this works
    let parts: Vec<&str> = command.split(" ").collect();
    parts
}

fn execute_test(test: &str) -> bool {
    let args = split_command(&test);

    let status = Command::new("test")
        .args(&args)
        .status()
        .expect(&format!("failed to run test {}", test));

    status.success()
}

fn replace_variables(command: &str, variables: &HashMap<String, String>)
        -> String
{
    let mut res = String::from(command);

    for (original, replacement) in variables {
        // replace "{{ varname }}" with replacement value
        let varname = format!("{{{{ {} }}}}", original);
        res = res.replace(&varname, replacement);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::pipeline::Pipeline;

    fn execute_stringout(pipeline: Pipeline,
                         variables: HashMap<String, String>) -> String {
        let mut stdout = Vec::new();
        execute(&vec![pipeline], &variables, &mut stdout);
        String::from_utf8(stdout.iter().map(|&c| c as u8).collect()).unwrap()
    }

    #[test]
    fn test_execute_pipeline() {
        let pipeline = Pipeline {
            name: String::from("my-test"),
            commands: vec!["echo 'this is my test'".to_string()],
            when: None,
        };
        let variables = HashMap::new();

        let result = execute_stringout(pipeline, variables);

        assert!(result.contains("Executing pipeline \"my-test\""));
        assert!(result.contains("this is my test"));
    }

    #[test]
    fn test_pipeline_with_variables() {
        let pipeline = Pipeline {
            name: String::from("my-test"),
            commands: vec!["echo '{{ myvar }}'".to_string()],
            when: None,
        };
        let mut variables = HashMap::new();
        variables.insert(String::from("myvar"), String::from("some value"));

        let result = execute_stringout(pipeline, variables);

        assert!(result.contains("some value"));
    }

    #[test]
    fn test_conditional_pipeline() {
        let pipeline = Pipeline {
            name: String::from("my-test"),
            commands: vec!["echo 'Building non-master'".to_string()],
            when: Some(String::from("\"{{ branch }}\" != \"master\"")),
        };
        let mut variables = HashMap::new();
        variables.insert(String::from("branch"), String::from("master"));

        let result = execute_stringout(pipeline, variables);

        println!("{}", result);
        assert!(!result.contains("non-master"));
    }
}
