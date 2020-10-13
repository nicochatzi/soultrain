pub mod download;
pub mod files;
pub mod github;

use crate::download::Downloadable;

use std::{fs, io, path::PathBuf};

use regex::Regex;
use semver::Version;

pub fn latest_version() -> Version {
    let version = github::Release::latest().tag_name;
    Version::parse(&version).unwrap()
}

pub fn current_version() -> Version {
    let link = std::fs::read_link(files::directory::latest().join("soul")).unwrap();
    let version = Regex::new(r"\d+\.\d+\.\d+")
        .unwrap()
        .find(&link.to_str().unwrap())
        .unwrap();
    Version::parse(&version.as_str()).unwrap()
}

pub fn install_latest() {
    let latest = github::Release::latest();
    latest.asset().download().unwrap();
    let release_dir = unzip_release(latest);
    crate::files::setup_links(release_dir.clone());
    set_permissions(release_dir);
}

pub fn list_releases() -> String {
    let releases = github::Releases(github::Release::all());
    format!("{}", releases)
}

fn set_permissions(release_dir: PathBuf) {
    let soul = release_dir.join("osx").join("x64").join("soul");
    std::process::Command::new("chmod")
        .arg("+x")
        .arg(soul)
        .output();
    std::process::Command::new("exec").output();
}

fn unzip_release(latest: github::Release) -> PathBuf {
    let version = Version::parse(&latest.tag_name).unwrap();
    let release_dir = files::directory::release(version);
    let zip_file = latest.asset().info().unwrap().file;

    unzip_file(zip_file.clone(), release_dir.clone());
    fs::remove_file(zip_file);

    release_dir
}

fn unzip_file(zip_file: PathBuf, dest_dir: PathBuf) {
    let zip_file = std::fs::File::open(&zip_file).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        if !(&*file.name()).ends_with('/') {
            #[allow(deprecated)]
            let path = dest_dir.join(file.sanitized_name().clone());
            create_parent_dirs(&path);

            let mut dest = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .unwrap();
            io::copy(&mut file, &mut dest).unwrap();
        }
    }
}

fn create_parent_dirs(path: &PathBuf) {
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(&parent).unwrap();
    }
}
