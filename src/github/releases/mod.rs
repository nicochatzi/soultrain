pub mod asset;
pub mod release;

pub use asset::*;
pub use release::*;

pub struct Releases {
    inner: Vec<Release>,
}

impl Releases {
    pub fn pull(url: &reqwest::Url) -> Self {
        Self {
            inner: crate::github::api::get(&url),
        }
    }

    pub fn latest(&self) -> Option<&Release> {
        self.inner.get(0)
    }

    pub fn previous(&self) -> Option<&Release> {
        self.inner.get(1)
    }

    pub fn all(&self) -> &Vec<Release> {
        &self.inner
    }

    pub fn version(&self, version: &semver::Version) -> Option<&Release> {
        Some(&self.inner
            .iter()
            .find(|&release| release.tag_name == version.to_string())
            .unwrap())
    }

    pub fn list(&self, length: Option<usize>) -> String {
        let mut releases = String::new();

        for (i, release) in self.all().iter().enumerate() {
            if let Some(length) = length {
                if i >= length {
                    break;
                }
            }
            releases.push_str(&format!("{}\n\n", release));
        }

        format!("{}", releases)
    }
}
