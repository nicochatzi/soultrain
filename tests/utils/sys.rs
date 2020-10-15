use crate::utils::cmd;

use std::{fs::*, path::*};

pub fn is_soul_installed() -> bool {
    match cmd::soul::soul().output() {
        Ok(output) => output.stderr.is_empty(),
        Err(_) => false,
    }
}

pub fn remove_soul() {
    let soul = PathBuf::from("/usr/local/bin/soul");
    if soul.exists() {
        remove_file(soul).unwrap();
    }
    assert!(!is_soul_installed());
}

pub fn are_soultrain_dirs_present() -> bool {
    assert!(!PathBuf::from("~/.soultrain/latest").exists());
    assert!(!PathBuf::from("~/.soultrain/release").exists());
}

pub fn remove_soultrain() {
    let remove_dirs = |root| {
        if root.exists() {
            remove_dir_all(root).unwrap();
        }
    }
    remove_dirs(PathBuf::from("~/.soultrain/latest"));
    remove_dirs(PathBuf::from("~/.soultrain/release"));
    assert!(!are_soultrain_dirs_present());
}

