use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use git2::Repository;

use rand::Rng;
use rand::distributions::Alphanumeric;

// TODO: Move this to a sourcecontrol/git module later
trait CodeSource {
    fn fetch(&self, target: &Path) -> Result<WorkDir, Box<Error>>;
}

struct Git {
    src: String,
}

struct WorkDir {
    path: PathBuf,
}

impl CodeSource for Git {
    fn fetch(&self, target: &Path) -> Result<WorkDir, Box<Error>> {
        let repo = Repository::clone(&self.src, target)?;
        let path = repo.workdir()
            .expect("Newly cloned repo is expected to have a workdir");

        Ok(WorkDir {
            path: path.to_path_buf(),
        })
    }
}

impl Drop for WorkDir {
    fn drop(&mut self) {
        // TODO: Only write this error to a log file, but do not panic
        fs::remove_dir_all(&self.path)
            .expect("Could not delete work dir");
    }
}

fn random_dir() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>()
}

fn main() {
    println!("Hello, Cinderella!");

    let repo = Git {
        src: String::from("https://github.com/aufziehvogel/CInderella.git"),
    };

    // generate a temp unique work dir
    let mut tempdir = PathBuf::from("/tmp/cinderella");
    tempdir.push(random_dir());

    let workdir = repo.fetch(&tempdir).expect("could not clone repo");
    println!("Workdir is at {:?}", workdir.path);
}
