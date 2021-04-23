use crate::files::directory;

use std::fs;

use regex::Regex;
use semver::Version;

pub fn cached() -> Vec<Version> {
    if let Err(_) = fs::read_dir(directory::releases()) {
        fs::create_dir_all(directory::releases());
    }

    let mut versions = Vec::<Version>::new();
    for entry in fs::read_dir(directory::releases()).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            let file = path.file_name().unwrap().to_str().unwrap();
            if let Ok(version) = Version::parse(file) {
                versions.push(version);
            }
        }
    }
    versions
}

pub fn latest_cached() -> Version {
    let mut latest = Version::parse("0.0.0").unwrap();
    for version in cached() {
        if latest < version {
            latest = version;
        }
    }
    latest
}

pub fn is_cached(version: &Version) -> bool {
    for cached in cached() {
        if *version == cached {
            return true;
        }
    }
    false
}

pub fn installed() -> Version {
    let link = std::fs::read_link(directory::latest().join("soul")).unwrap();
    let version = Regex::new(r"\d+\.\d+\.\d+")
        .unwrap()
        .find(&link.to_str().unwrap())
        .unwrap();
    Version::parse(&version.as_str()).unwrap()
}

pub fn clear_cache() {
    let installed = installed();
    for version in cached() {
        if version != installed {
            let dir = directory::release(version);
            fs::remove_dir_all(dir);
        }
    }
}
