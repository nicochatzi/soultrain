use crate::{files::directory, github::ReleaseAsset};

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

impl Release {
    pub fn version(&self) -> Version {
        Version::parse(&self.tag_name).unwrap()
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
