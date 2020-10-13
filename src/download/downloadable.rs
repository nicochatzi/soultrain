use crate::download::{*, progress::*};

use std::{fs, io, path::PathBuf};

use exitfailure::ExitFailure;
use reqwest::{
    blocking::{Client, Response},
    header, Url,
};

pub type DownloadResult<T> = Result<T, ExitFailure>;

pub struct DownloadInfo {
    pub url: Url,
    pub file: PathBuf,
    pub size: u64,
}

pub trait Downloadable {
    fn info(&self) -> DownloadResult<DownloadInfo>;

    fn download(&self) -> DownloadResult<Response> {
        let info = self.info()?;
        let mut request = Client::new().get(info.url.as_str());
        let progress_bar = progress::bar::create(info.size);

        if info.file.exists() {
            let size = info.file.metadata()?.len() - 1;
            request = request.header(header::RANGE, format!("bytes={}-", size));
            progress_bar.inc(size);
        }

        let mut source = DownloadProgress::new(request.send()?, progress_bar);
        let mut dest = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&info.file)?;
        let _ = io::copy(&mut source, &mut dest)?;

        Ok(source.result())
    }
}

// fn content_length(&self) -> u64 {
//     let url = self.info().unwrap().url;
//     Client::new()
//         .head(url)
//         .headers(headers())
//         .send()
//         .unwrap()
//         .headers()
//         .get(header::CONTENT_LENGTH)
//         .and_then(|length| length.to_str().ok())
//         .and_then(|length| length.parse().ok())
//         .unwrap()
// }