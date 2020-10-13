use crate::{
    files::directory,
    github::{api, ReleaseAsset},
};

use std::path::PathBuf;

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Release {
    pub html_url: String,
    pub tag_name: String,
    pub published_at: String,
    pub assets: Vec<ReleaseAsset>,
}

impl std::fmt::Display for Release {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Version: {}\nPublished on: {}\n{}",
            self.tag_name,
            self.date(),
            self.html_url
        )
    }
}

pub struct Releases(pub Vec<Release>);

impl std::fmt::Display for Releases {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut releases = String::new();

        for (i, release) in self.0.iter().enumerate() {
            releases.push_str(&format!("{}", release));
            releases.push_str("\n\n");
            if i >= 5 {
                break;
            }
        }

        write!(formatter, "{}", releases)
    }
}

impl Release {
    pub fn latest() -> Self {
        api::get(&api::endpoints::soul::LATEST_RELEASE)
    }

    pub fn with_version(version: Version) -> Self {
        Self::all()
            .iter()
            .find(|&release| release.tag_name == version.to_string())
            .unwrap()
            .clone()
    }

    pub fn all() -> Vec<Self> {
        api::get(&api::endpoints::soul::RELEASES)
    }

    pub fn asset(&self) -> &ReleaseAsset {
        self.assets.iter().find(|&asset| asset.is_native()).unwrap()
    }

    pub fn directory(&self) -> PathBuf {
        let version = self.tag_name.clone();
        directory::release(Version::parse(&version).unwrap())
    }

    pub fn date(&self) -> String {
        chrono::DateTime::parse_from_rfc3339(&self.published_at)
            .unwrap()
            .to_rfc2822()
    }
}
