use semver::Version;
use std::path::PathBuf;

pub fn soultrain() -> PathBuf {
    dirs::home_dir().unwrap().join(".soultrain")
}

pub fn latest() -> PathBuf {
    soultrain().join("latest")
}

pub fn releases() -> PathBuf {
    soultrain().join("releases")
}

pub fn release(version: Version) -> PathBuf {
    releases().join(version.to_string())
}

pub fn create_parents(path: &PathBuf) {
    let parent = path.parent().unwrap();
    if !parent.exists() {
        std::fs::create_dir_all(&parent).unwrap();
    }
}
