use crate::{
    download::Downloadable,
    files::{directory, utils},
    installer::{link, version},
    github::{Releases, Release},
};

use std::{fs, path::PathBuf};

use semver::Version;

pub struct Installer<'a> {
    releases: &'a Releases,
}

impl<'a> Installer<'a> {
    pub fn new(releases: &'a Releases) -> Self {
        Self { releases }
    }

    pub fn current_version(&self) -> Version {
        version::installed()
    }

    pub fn install_version(&self, version: &str) {
        match self.install_named_version(version) {
            Some(version) => println!("Now using SOUL v{}", version.to_string()),
            None => println!(
                "Could not find the specified version: {}",
                version.to_string()
            ),
        }
    }

    pub fn uninstall(&self) {
        link::remove_links();
    }

    pub fn cleanup(&self) {
        version::clear_cache();
    }

    fn parse_named(&self, version: &str) -> Option<Version> {
        match version {
            "latest" => Some(self.releases.latest()?.version()),
            "previous" => Some(self.releases.previous()?.version()),
            _ => Version::parse(&version).ok(),
        }
    }

    fn install_named_version(&self, version: &str) -> Option<Version> {
        let version = self.parse_named(version)?;
        Self::install(self.releases.version(&version)?);
        Some(version::installed())
    }

    fn install(release: &Release) {
        if !version::is_cached(&release.version()) {
            release.asset().download().unwrap();
            Self::unzip_release(release);
        }

        let release_dir = directory::release(release.version());
        link::setup(release_dir.clone());
        Self::set_permissions(release_dir);
    }

    fn set_permissions(release_dir: PathBuf) {
        let soul = release_dir.join("osx").join("x64").join("soul");
        utils::set_permissions(soul);
    }

    fn unzip_release(release: &Release) -> PathBuf {
        let release_dir = directory::release(release.version());
        let zip_file = release.asset().file();

        utils::unzip_file(zip_file.clone(), release_dir.clone());
        fs::remove_file(zip_file);

        release_dir
    }
}
