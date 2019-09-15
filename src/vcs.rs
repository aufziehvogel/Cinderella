use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use git2::Repository;

pub trait CodeSource {
    // TODO: Returned working copy here should be dynamic
    fn fetch(&self, target: &Path) -> Result<GitWorkingCopy, Box<dyn Error>>;
}

pub trait WorkingCopy {
    fn checkout_branch(&self, branch: &str);
}

pub struct GitSource {
    pub src: String,
}

pub struct GitWorkingCopy {
    pub path: PathBuf,
    repo: Repository,
}

impl CodeSource for GitSource {
    fn fetch(&self, target: &Path) -> Result<GitWorkingCopy, Box<dyn Error>> {
        let repo = Repository::clone(&self.src, target)?;

        let path = repo.workdir()
            .expect("Newly cloned repo is expected to have a workdir");

        Ok(GitWorkingCopy {
            path: path.to_path_buf(),
            repo: repo,
        })
    }
}

impl WorkingCopy for GitWorkingCopy {
    fn checkout_branch(&self, branch_name: &str) {
        let revname = format!("refs/remotes/origin/{}", branch_name);
        self.checkout_rev(&revname);
    }
}

impl GitWorkingCopy {
    fn checkout_rev(&self, rev: &str) {
        let obj = self.repo.revparse_single(rev).unwrap();

        self.repo.checkout_tree(
            &obj,
            None
        ).expect("Checkout of tree failed");
    }
}

impl Drop for GitWorkingCopy {
    fn drop(&mut self) {
        // TODO: Only write this error to a log file, but do not panic
        fs::remove_dir_all(&self.path)
            .expect("Could not delete work dir");
    }
}
