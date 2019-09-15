use std::collections::HashMap;
use std::process::Command;

use crate::pipeline;

pub fn execute(pipelines: &Vec<pipeline::Pipeline>,
               variables: &HashMap<String, String>) {
    for pipeline in pipelines {
        execute_pipeline(pipeline, &variables);
    }
}

pub fn execute_pipeline(pipeline: &pipeline::Pipeline,
                        variables: &HashMap<String, String>) {
    println!("Executing Pipeline \"{}\"", pipeline.name);

    for cmd in &pipeline.commands {
        println!("Step: {}", cmd);

        let cmd = replace_variables(&cmd, &variables);
        // TODO: Raise error if some variables remain unsubstituted?

        // TODO: Successful argument parsing needs a lot more details,
        // e.g. for quoted arguments like myprogram "argument 1"
        // but for a first shot this works
        let parts: Vec<&str> = cmd.split(" ").collect();

        let status = Command::new(parts[0])
            .args(&parts[1..])
            .status()
            .expect(&format!("failed to run {}", cmd));

        assert!(status.success());
    }
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
