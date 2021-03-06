use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::env;

use evalexpr;
use duct::cmd;

use crate::parser;
use crate::pipeline;

pub enum ExecutionResult {
    NoExecution,
    Success(Vec<StepResult>),
    Error(Vec<StepResult>),
}

pub enum StepResult {
    Success(String, String),
    Error(String, String, Option<i32>),
}

struct Command {
    command: String,
    args: Vec<String>,
}

impl Command {
    fn command_string(&self) -> String {
        let mut parts = self.args.clone();
        parts.insert(0, String::from(&self.command));

        let command = parts.iter().map(|arg| {
            if arg.contains(" ") {
                format!("\"{}\"", arg)
            } else {
                String::from(arg)
            }
        }).collect::<Vec<String>>().join(" ");

        command
    }

    fn execute(&self) -> StepResult {
        let reader = cmd(&self.command, &self.args).stderr_to_stdout()
            .reader().unwrap();
        let f = BufReader::new(&reader);

        let mut outtext = String::new();

        for line in f.lines() {
            match line {
                Ok(line) => {
                    println!("{}", line);

                    // TODO: Newline style should be system dependent
                    outtext.push_str(&line);
                    outtext.push_str("\n");
                },
                _ => {
                    reader.kill().expect("Could not kill reader");
                    return StepResult::Error(
                        self.command_string(),
                        outtext,
                        // TODO: How can we get the correct code here?
                        None
                    );
                },
            }
        }

        // guaranteed to be Ok(Some(_)) after EOF
        let output = reader.try_wait().unwrap().unwrap();
        match output.status.success() {
            true => StepResult::Success(self.command_string(), outtext),
            false => {
                StepResult::Error(
                    self.command_string(),
                    outtext,
                    output.status.code()
                )
            },
        }
    }
}

pub fn execute(
    pipelines: &Vec<pipeline::Pipeline>,
    variables: &HashMap<String, String>) -> ExecutionResult
{
    let mut done_steps = Vec::new();

    for pipeline in pipelines {
        let execute = match &pipeline.when {
            Some(when) => {
                execute_test(&when, &variables)
            }
            None => true,
        };

        if execute {
            let res = execute_pipeline(pipeline, &variables);

            match res {
                ExecutionResult::Success(steps) => done_steps.extend(steps),
                ExecutionResult::Error(_) => return res,
                ExecutionResult::NoExecution => (),
            }
        }
    }

    if done_steps.len() > 0 {
        ExecutionResult::Success(done_steps)
    } else {
        ExecutionResult::NoExecution
    }
}

fn execute_pipeline(
    pipeline: &pipeline::Pipeline,
    variables: &HashMap<String, String>) -> ExecutionResult
{
    let res = pipeline.commands.iter()
        .try_fold(Vec::<StepResult>::new(), |mut step_results, cmd| {
            let result = execute_step(&cmd, variables);

            match result {
                StepResult::Success(_, _) => {
                    step_results.push(result);
                    Ok(step_results)
                },
                StepResult::Error(_, _, _) => {
                    step_results.push(result);
                    Err(step_results)
                }
            }
        });

    match res {
        Ok(step_results) => ExecutionResult::Success(step_results),
        Err(step_results) => ExecutionResult::Error(step_results),
    }
}

fn execute_step(cmd: &str, variables: &HashMap<String, String>) -> StepResult {
    let cmd = replace_variables(&cmd, &variables);
    let parts = parser::parse_command(&cmd);

    let cmd = Command {
        command: String::from(&parts[0]),
        args: parts[1..].to_vec(),
    };

    cmd.execute()
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

    let res = replace_envvars(&res);

    res
}

fn replace_envvars(command: &str) -> String
{
    let mut res = String::from(command);

    for (key, value) in env::vars() {
        let varname = format!("${}", key.to_uppercase());
        res = res.replace(&varname, &value);
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
        let res = execute(&vec![pipeline], &variables);

        let mut out = String::new();
        match res {
            ExecutionResult::Success(steps)
                | ExecutionResult::Error(steps) =>
            {
                for step in steps {
                    let text = match step {
                        StepResult::Success(_command, out) => out,
                        StepResult::Error(_command, out, _code) => out,
                    };
                    out.push_str(&text);
                }
            },
            _ => (),
        }

        out
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

        assert!(result.contains("this is my test"));
    }

    #[test]
    fn test_execute_error_statement() {
        let pipeline = Pipeline {
            name: String::from("error-test"),
            commands: vec!["bash -c \"exit 1\"".to_string()],
            when: None,
        };
        let variables = HashMap::new();

        let result = execute(&vec![pipeline], &variables);

        match result {
            ExecutionResult::Error(steps) => {
                if let StepResult::Error(cmd, _out, _code) = &steps[0] {
                    assert_eq!(cmd, "bash -c \"exit 1\"");
                } else {
                    assert!(false);
                }
            },
            // fail if something different from error is returned
            _ => assert!(false),
        }
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

        assert!(result.contains("Building master"));
    }
}
