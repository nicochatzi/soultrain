use std::{io, path::PathBuf, process::{Output, Command}};

pub mod soultrain {
    use super::*;

    fn soultrain() -> Command {
        if std::cfg!(debug_assertions) {
            Command::new("target/debug/soultrain")
        } else {
            Command::new("target/release/soultrain")
        }
    }

    pub fn cleanup() -> io::Result<Output> {
        soultrain().arg("cleanup").output()
    }

    pub fn latest() -> io::Result<Output> {
        soultrain().arg("latest").output()
    }

    pub fn list(length: &str) -> io::Result<Output> {
        soultrain().arg("list").arg(length).output()
    }

    pub fn select(version: &str) -> io::Result<Output> {
        soultrain().arg("select").arg(version).output()
    }

    pub fn show() -> io::Result<Output> {
        soultrain().arg("show").output()
    }

    pub fn uninstall() -> io::Result<Output> {
        soultrain().arg("uninstall").output()
    }

    pub fn update() -> io::Result<Output> {
        soultrain().arg("update").output()
    }
}

pub mod soul {
    use super::*;

    pub fn soul() -> Command {
        Command::new("soul")
    }

    pub fn version() -> io::Result<Output> {
        soul().arg("version").output()
    }
}
