use std::path::PathBuf;
use std::io::prelude::Write;
use std::fs::{self, File};

pub enum BuildStatus {
    Success,
    Error(String),
}

static ICON_SUCCESS: &'static [u8] = include_bytes!("../assets/icon_build_success.png");
static ICON_ERROR: &'static [u8] = include_bytes!("../assets/icon_build_error.png");

pub fn generate_status_icon(project: &str, branch: &str, status: BuildStatus, dir: &PathBuf) {
    let mut path = dir.clone();
    path.push(project);

    // ignore the AlreadyExists error
    // other errors are also not important, because
    // we recognize them once the file cannot be written
    let _ = fs::create_dir(path.clone());

    path.push(format!("{}.png", branch));

    // TODO: Error handling
    let mut buffer = File::create(path).unwrap();
    match status {
        BuildStatus::Success => buffer.write(ICON_SUCCESS).expect("Could not write to file"),
        BuildStatus::Error(_) => buffer.write(ICON_ERROR).expect("Could not write to file"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::Read;

    #[test]
    fn generate_success_status_icon() {
        let dir = tempfile::tempdir().unwrap();
        let pathbuf = dir.path().to_path_buf();

        generate_status_icon("myproject", "master", BuildStatus::Success, &pathbuf);

        let mut buf = dir.path().to_path_buf();
        buf.push("myproject");
        buf.push("master.png");
        assert!(buf.exists());

        let mut f = File::open(buf).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        assert_eq!(buffer, ICON_SUCCESS);
    }

    #[test]
    fn generate_error_status_icon() {
        let dir = tempfile::tempdir().unwrap();
        let pathbuf = dir.path().to_path_buf();

        generate_status_icon("my-other-project", "some-branch", BuildStatus::Error("Error Reason".to_string()), &pathbuf);

        let mut buf = dir.path().to_path_buf();
        buf.push("my-other-project");
        buf.push("some-branch.png");
        assert!(buf.exists());

        let mut f = File::open(buf).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();

        assert_eq!(buffer, ICON_ERROR);
    }
}