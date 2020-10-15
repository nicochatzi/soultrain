use crate::files::directory;

use std::{fs, path::PathBuf, process::Command};

pub fn unzip_file(zip_file: PathBuf, dest_dir: PathBuf) {
    let zip_file = fs::File::open(&zip_file).unwrap();
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        if !(&*file.name()).ends_with('/') {
            #[allow(deprecated)]
            let path = dest_dir.join(file.sanitized_name().clone());
            directory::create_parents(&path);

            let mut dest = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .unwrap();
            std::io::copy(&mut file, &mut dest).unwrap();
        }
    }
}

pub fn set_permissions(file: PathBuf) {
    Command::new("chmod").arg("+x").arg(file).output();
    Command::new("exec").output();
}
