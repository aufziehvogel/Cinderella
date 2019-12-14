use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use rand::Rng;
use rand::distributions::Alphanumeric;

mod config;
mod vcs;
mod parser;
mod pipeline;
mod execution;
mod mail;
mod crypto;
mod variables;

pub use crate::config::ExecutionConfig;

use crate::config::{CinderellaConfig, Configs};
use crate::execution::{ExecutionResult, StepResult};
use crate::vcs::CodeSource;
use crate::vcs::WorkingCopy;

fn random_dir(base_path: &str) -> PathBuf {
    let mut tempdir = PathBuf::from(base_path);

    let random_dirname = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>();
    tempdir.push(random_dirname);

    tempdir
}

fn appconfig_file() -> PathBuf {
    let mut application_path = env::current_exe().unwrap();
    application_path.pop();
    application_path.push("config.toml");

    application_path
}

pub fn run(exec_config: &ExecutionConfig) {
    let cinderella_config = CinderellaConfig::from_file(appconfig_file());
    let configs = Configs {
        cinderella_config: &cinderella_config,
        execution_config: exec_config,
    };

    let repo = vcs::GitSource {
        src: exec_config.repo_url.clone(),
    };

    // generate a temp unique work dir
    let tempdir = random_dir("/tmp/cinderella");
    let workdir = repo.fetch(&tempdir).expect("could not clone repo");

    println!("Workdir is at {:?}", workdir.path);

    // checkout the branch if a branch was provided
    if let Some(branch) = &exec_config.branch {
        println!("Switching to branch {}", branch);
        workdir.checkout_branch(&branch);
    } else if let Some(tag) = &exec_config.tag {
        println!("Switching to tag {}", tag);
        workdir.checkout_tag(&tag);
    }

    // Switch to the exported work dir so that all commands
    // are executed there
    assert!(env::set_current_dir(&workdir.path).is_ok());

    let cinderella_file = exec_config.cinderella_file(&workdir.path);
    if let Some(pipelines) = pipeline::load_pipeline(&cinderella_file) {
        // TODO: Check if execution was successful. If not and if email is
        // configured, send a mail
        let variables = variables::load(&workdir.path, &configs);
        let res = execution::execute(&pipelines, &variables);

        match res {
            ExecutionResult::Error(steps) => {
                let mut output = String::new();

                for step in steps {
                    match step {
                        StepResult::Success(command, out)
                            | StepResult::Error(command, out, _) =>
                        {
                            output.push_str(&command);
                            // TODO: newline should be system-dependent
                            output.push_str("\n");
                            output.push_str(&out);
                        },
                    }
                }

                let mailer = mail::build_mailer(&cinderella_config.email);
                mailer.send_mail(
                    &exec_config.name(),
                    &format!("Build failed:\n\n{}", output));
            },
            _ => (),
        }
    } else {
        println!("No Cinderella configuration found");
    }
}

pub fn encrypt(plainpath: &Path, cipherpath: &Path, password: &str) {
    let plaintext = fs::read_to_string(plainpath)
        .expect("Unable to read file");

    let cipher = crypto::encrypt_string(&plaintext, password);
    fs::write(cipherpath, cipher).expect("Unable to write file");
}

pub fn decrypt(cipherpath: &Path, plainpath: &Path, password: &str) {
    match crypto::decrypt_file(&cipherpath, password) {
        Ok(plaintext) => {
            fs::write(plainpath, plaintext).expect("Unable to write file");
        },
        _ => println!("Cannot decrypt, probably wrong password"),
    }
}
