use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use rand::Rng;
use rand::distributions::Alphanumeric;

mod vcs;
mod pipeline;
mod execution;

use crate::vcs::CodeSource;
use crate::vcs::WorkingCopy;

pub struct RepoPointer {
    pub repo_url: String,
    pub branch: Option<String>,
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

fn cinderella_file(folder: &PathBuf) -> PathBuf {
    let mut cinderella_file = folder.clone();
    cinderella_file.push(".cinderella.toml");

    cinderella_file
}

pub fn run(repo_ptr: &RepoPointer) {
    let repo = vcs::GitSource {
        src: String::from(&repo_ptr.repo_url),
    };

    // generate a temp unique work dir
    let tempdir = random_dir("/tmp/cinderella");
    let workdir = repo.fetch(&tempdir).expect("could not clone repo");

    println!("Workdir is at {:?}", workdir.path);

    // setup variables for pipelines
    let mut variables = HashMap::new();

    // checkout the branch if a branch was provided
    if let Some(branch) = &repo_ptr.branch {
        println!("Switching to branch {}", branch);
        workdir.checkout_branch(&branch);

        variables.insert("branch".to_string(), branch.to_string());
    }

    // Switch to the exported work dir so that all commands
    // are executed there
    assert!(env::set_current_dir(&workdir.path).is_ok());

    let cinderella_file = cinderella_file(&workdir.path);
    if let Some(pipelines) = pipeline::load_pipeline(&cinderella_file) {
        execution::execute(&pipelines, &variables);
    } else {
        println!("No Cinderella configuration found");
    }
}
