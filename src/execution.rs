use std::process::Command;

use crate::pipeline;

pub fn execute(pipelines: &Vec<pipeline::Pipeline>) {
    for pipeline in pipelines {
        execute_pipeline(pipeline);
    }
}

pub fn execute_pipeline(pipeline: &pipeline::Pipeline) {
    println!("Executing Pipeline \"{}\"", pipeline.name);

    for cmd in &pipeline.commands {
        println!("Step: {}", cmd);

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
