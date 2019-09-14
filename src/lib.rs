use std::path::PathBuf;

use rand::Rng;
use rand::distributions::Alphanumeric;

mod vcs;
mod pipeline;
mod execution;

use crate::vcs::CodeSource;

fn random_dir() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>()
}

pub fn run(repo_url: &str) {
    let repo = vcs::Git {
        src: String::from(repo_url),
    };

    // generate a temp unique work dir
    let mut tempdir = PathBuf::from("/tmp/cinderella");
    tempdir.push(random_dir());

    let workdir = repo.fetch(&tempdir).expect("could not clone repo");

    println!("Workdir is at {:?}", workdir.path);

    let mut cinderella_file = workdir.path.clone();
    cinderella_file.push(".cinderella.toml");

    if let Some(pipelines) = pipeline::load_pipeline(".cinderella.toml") {
        execution::execute(&pipelines);
    } else {
        println!("No Cinderella configuration found");
    }
}
