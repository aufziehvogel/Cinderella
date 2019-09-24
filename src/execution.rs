use std::collections::HashMap;
use std::process::Command;
use std::io::Write;

use evalexpr;

use crate::parser;
use crate::pipeline;

pub enum ExecutionResult {
    NoExecution,
    Success,
    // failed command and its output
    BuildError(String, String, Option<i32>),
    ExecutionError(String, String),
}

pub fn execute<W: Write>(
    pipelines: &Vec<pipeline::Pipeline>,
    variables: &HashMap<String, String>,
    stdout: &mut W) -> ExecutionResult
{
    // TODO: Refactor this whole function to get a cleaner design
    let mut executed_at_least_one = false;

    for pipeline in pipelines {
        let execute = match &pipeline.when {
            Some(when) => {
                execute_test(&when, &variables)
            }
            None => true,
        };

        if execute {
            executed_at_least_one = true;
            let res = execute_pipeline(pipeline, &variables, stdout);

            match res {
                ExecutionResult::BuildError(_, _, _) | ExecutionResult::ExecutionError(_, _) => {
                    return res;
                },
                _ => (),
            }
        }
    }

    if executed_at_least_one {
        ExecutionResult::Success
    } else {
        ExecutionResult::NoExecution
    }
}

fn execute_pipeline<W: Write>(
    pipeline: &pipeline::Pipeline,
    variables: &HashMap<String, String>,
    stdout: &mut W) -> ExecutionResult
{
    writeln!(stdout, "Executing pipeline \"{}\"\n", pipeline.name).unwrap();

    for cmd in &pipeline.commands {
        writeln!(stdout, "Step: {}", cmd).unwrap();

        let cmd = replace_variables(&cmd, &variables);
        // TODO: Raise error if some variables remain unsubstituted?

        let parts = parser::parse_command(&cmd);
        let output = Command::new(&parts[0])
            .args(&parts[1..])
            .output();
        let output = match output {
            Ok(output) => output,
            Err(e) => return ExecutionResult::ExecutionError(cmd, e.to_string()),
        };

        stdout.write_all(&output.stdout).unwrap();

        let outtext = String::from_utf8(output.stdout.iter().map(|&c| c as u8).collect()).unwrap();
        if !output.status.success() {
            return ExecutionResult::BuildError(
                String::from(format!("Pipeline failed in step: {}", cmd)),
                outtext,
                output.status.code()
            );
        }
    }

    ExecutionResult::Success
}

fn execute_test(test: &str, variables: &HashMap<String, String>) -> bool {
    // not possible to use evalexpr Context, because evalexpr only handles
    // standard variable names without special characters (percentage
    // symbol cannot be used)
    let test = replace_variables(test, variables);

    match evalexpr::eval_boolean(&test) {
        Ok(true) => true,
        _ => false,
    }
}

fn replace_variables(command: &str, variables: &HashMap<String, String>)
        -> String
{
    let mut res = String::from(command);

    for (original, replacement) in variables {
        // replace "%VARNAME" with replacement value
        let varname = format!("%{}", original.to_uppercase());
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
            commands: vec!["echo '%MYVAR'".to_string()],
            when: None,
        };
        let mut variables = HashMap::new();
        variables.insert(String::from("myvar"), String::from("some value"));

        let result = execute_stringout(pipeline, variables);

        assert!(result.contains("some value"));
    }

    #[test]
    fn test_conditional_pipeline_false() {
        let pipeline = Pipeline {
            name: String::from("my-test"),
            commands: vec!["echo 'Building non-master'".to_string()],
            when: Some(String::from("\"%BRANCH\" != \"master\"")),
        };
        let mut variables = HashMap::new();
        variables.insert(String::from("branch"), String::from("master"));

        let result = execute_stringout(pipeline, variables);

        println!("{}", result);
        assert!(!result.contains("non-master"));
    }

    #[test]
    fn test_conditional_pipeline_true() {
        let pipeline = Pipeline {
            name: String::from("my-test"),
            commands: vec!["echo 'Building master'".to_string()],
            when: Some(String::from("\"%BRANCH\" == \"master\"")),
        };
        let mut variables = HashMap::new();
        variables.insert(String::from("branch"), String::from("master"));

        let result = execute_stringout(pipeline, variables);

        println!("{}", result);
        assert!(result.contains("Building master"));
    }
}
