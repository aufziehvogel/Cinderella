use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use git2::Repository;

pub trait CodeSource {
    fn fetch(&self, target: &Path) -> Result<WorkDir, Box<Error>>;
}

pub struct Git {
    pub src: String,
}

pub struct WorkDir {
    pub path: PathBuf,
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
