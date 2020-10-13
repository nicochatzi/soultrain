use std::io;

use indicatif::{ProgressBar, ProgressStyle};

pub mod bar {
    use super::*;

    pub fn create(length: u64) -> ProgressBar {
        let bar = ProgressBar::new(length);
        bar.set_style(style());
        bar
    }

    fn style() -> ProgressStyle {
        ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("█ ")
    }
}

pub struct DownloadProgress<R> {
    inner: R,
    progress_bar: ProgressBar,
}

impl<R: io::Read> io::Read for DownloadProgress<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buffer).map(|increment| {
            self.progress_bar.inc(increment as u64);
            increment
        })
    }
}

impl<R> DownloadProgress<R> {
    pub fn new(request: R, bar: ProgressBar) -> Self {
        Self {
            inner: request,
            progress_bar: bar,
        }
    }

    pub fn result(self) -> R {
        self.inner
    }
}
