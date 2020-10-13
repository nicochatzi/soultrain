use crate::{
    download::{DownloadInfo, DownloadResult, Downloadable},
    files::directory,
};

use std::path::PathBuf;

use reqwest::Url;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct ReleaseAsset {
    pub name: String,
    pub size: u64,
    pub browser_download_url: String,
}

impl std::fmt::Display for ReleaseAsset {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "Version: {}\nAsset: {}\nFrom: {}\nSize: {}",
            self.version(),
            self.name,
            self.browser_download_url,
            self.size
        )
    }
}

impl Downloadable for ReleaseAsset {
    fn info(&self) -> DownloadResult<DownloadInfo> {
        Ok(DownloadInfo {
            url: Url::parse(&self.browser_download_url)?,
            file: self.directory().join(self.name.clone()),
            size: self.size,
        })
    }
}

impl ReleaseAsset {
    pub fn version(&self) -> Version {
        let split = self.browser_download_url.split("/");
        let splits = split.collect::<Vec<&str>>();
        Version::parse(splits.get(splits.len() - 2).unwrap()).unwrap()
    }

    pub fn directory(&self) -> PathBuf {
        let release_dir = directory::release(self.version());
        std::fs::create_dir_all(release_dir.clone()).unwrap();
        release_dir
    }

    pub fn is_native(&self) -> bool {
        self.name.contains(os_name())
    }
}

fn os_name() -> &'static str {
    if std::cfg!(target_os = "linux") {
        return "linux";
    }
    if std::cfg!(target_os = "windows") {
        return "windows";
    }
    if std::cfg!(target_os = "macos") {
        return "osx";
    }
    panic!("Unsupported OS!");
}
