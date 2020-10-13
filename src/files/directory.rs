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
