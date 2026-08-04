use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Builder to create and configure a Beru project sandbox for testing.
pub struct ProjectBuilder {
    name: String,
    files: Vec<(PathBuf, String)>,
}

impl ProjectBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
        }
    }

    /// Add a file with relative path and contents to the project.
    pub fn file<P: AsRef<Path>>(mut self, path: P, contents: &str) -> Self {
        self.files
            .push((path.as_ref().to_path_buf(), contents.to_string()));
        self
    }

    /// Build the project in a temporary directory and return a handle.
    pub fn build(self) -> Project {
        let tempdir = tempfile::tempdir().expect("failed to create tempdir");
        let proj_dir = tempdir.path().join(&self.name);
        fs::create_dir_all(&proj_dir).expect("failed to create project root");

        for (path, contents) in self.files {
            let full_path = proj_dir.join(path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).expect("failed to create parent dir");
            }
            fs::write(&full_path, contents).expect("failed to write file");
        }

        let beru_home = tempdir.path().join(".beru");
        fs::create_dir_all(&beru_home).expect("failed to create BERU_HOME");

        Project {
            _tempdir: tempdir,
            root: proj_dir,
            beru_home,
        }
    }
}

/// A constructed project Sandbox.
pub struct Project {
    _tempdir: TempDir,
    root: PathBuf,
    beru_home: PathBuf,
}

impl Project {
    /// Return the absolute path to the project root.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Create a Beru command pre-configured with the project's root as cwd,
    /// and the sandbox's isolated BERU_HOME.
    pub fn beru(&self, cmd: &str) -> Command {
        let mut command = Command::cargo_bin("beru").expect("beru binary not found");
        command.env("BERU_HOME", &self.beru_home);
        command.current_dir(&self.root);
        command.arg(cmd);
        command
    }
}

/// Helper function to start building a project.
pub fn project(name: &str) -> ProjectBuilder {
    ProjectBuilder::new(name)
}
