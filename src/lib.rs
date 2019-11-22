use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

use rand::Rng;
use rand::distributions::Alphanumeric;

mod config;
mod vcs;
mod parser;
mod pipeline;
mod execution;
mod mail;
mod crypto;

use crate::execution::ExecutionResult;
use crate::vcs::CodeSource;
use crate::vcs::WorkingCopy;

pub struct ExecutionConfig {
    pub repo_url: String,
    pub branch: Option<String>,
    pub cinderella_filepath: Option<String>,
}

impl ExecutionConfig {
    // TODO: This approach only works for URLs, not for local paths.
    fn name(&self) -> String {
        self.repo_url.split('/').last().unwrap().to_string()
    }

    fn cinderella_file(&self, folder: &PathBuf) -> PathBuf {
        let filepath = match &self.cinderella_filepath {
            Some(filepath) => PathBuf::from(filepath),
            None => {
                let mut cinderella_file = folder.clone();
                cinderella_file.push(".cinderella.toml");
                cinderella_file
            },
        };

        filepath
    }
}

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
    let config = config::read_config(appconfig_file());

    let repo = vcs::GitSource {
        src: exec_config.repo_url.clone(),
    };

    // generate a temp unique work dir
    let tempdir = random_dir("/tmp/cinderella");
    let workdir = repo.fetch(&tempdir).expect("could not clone repo");

    println!("Workdir is at {:?}", workdir.path);

    // setup variables for pipelines
    let mut variables = HashMap::new();

    // checkout the branch if a branch was provided
    if let Some(branch) = &exec_config.branch {
        println!("Switching to branch {}", branch);
        workdir.checkout_branch(&branch);

        variables.insert("branch".to_string(), branch.to_string());
    }

    // Switch to the exported work dir so that all commands
    // are executed there
    assert!(env::set_current_dir(&workdir.path).is_ok());

    let cinderella_file = exec_config.cinderella_file(&workdir.path);
    if let Some(pipelines) = pipeline::load_pipeline(&cinderella_file) {
        // TODO: Check if execution was successful. If not and if email is
        // configured, send a mail
        let res = execution::execute(&pipelines, &variables, &mut io::stdout());

        match res {
            ExecutionResult::BuildError(msg, output, code) => {
                eprintln!("Build failed: {}\n\n{}", msg, output);

                let code_msg = match code {
                    Some(code) => format!("Exited with status code: {}", code),
                    None => format!("Process terminated by signal")
                };
                let mailer = mail::build_mailer(&config.email);
                mailer.send_mail(
                    &exec_config.name(),
                    &format!("Build failed: {}\n{}\n\n{}", msg, code_msg, output));
            },
            ExecutionResult::ExecutionError(msg, output) => {
                eprintln!("Build failed: {}\n\n{}", msg, output);

                let mailer = mail::build_mailer(&config.email);
                mailer.send_mail(
                    &exec_config.name(),
                    &format!("Build failed: {}\n\n{}", msg, output));
            },
            _ => (),
        }
    } else {
        println!("No Cinderella configuration found");
    }
}

pub fn encrypt(filepath: String) {
    let plaintext = "some data";

    let cipher = crypto::encrypt_string(plaintext);
    fs::write(filepath, cipher).expect("Unable to write file");
}

pub fn decrypt(filepath: String) {
    let plaintext = crypto::decrypt_file(&filepath);
    println!("{}", plaintext);
}
