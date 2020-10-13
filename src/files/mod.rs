pub mod directory;

use std::path::{Path, PathBuf};

use semver::Version;

fn get_latest_cached_version() -> Version {
    let mut latest_version = Version::parse("0.0.0").unwrap();
    for entry in std::fs::read_dir(directory::releases()).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            let file = path.file_name().unwrap().to_str().unwrap();
            if let Ok(version) = Version::parse(file) {
                if latest_version < version {
                    latest_version = version;
                }
            }
        }
    }
    latest_version
}

fn symlink<P, Q>(src: P, dst: Q) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    #[cfg(target_os = "macos")]
    return std::os::unix::fs::symlink(src, dst);

    #[cfg(target_os = "windows")]
    return std::os::windows::fs::symlink_file(src, dst);

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    panic!("Unsupported OS");
}

pub fn setup_links(release_dir: PathBuf) {
    setup_latest_links(release_dir);
    setup_install_links();
}

fn setup_latest_links(release_dir: PathBuf) {
    let release_dir = release_dir.join("osx").join("x64");
    let latest_dir = directory::latest();
    std::fs::create_dir_all(latest_dir.clone());
    let link = |file| {
        symlink(release_dir.join(file), latest_dir.clone().join(file));
    };
    link("SOUL_PatchLoader.dylib");
    link("soul");
}

fn setup_install_links() {
    let latest_dir = directory::latest();
    let patch_dir = dirs::data_local_dir().unwrap().join("SOUL");
    std::fs::create_dir_all(patch_dir.clone());

    symlink(
        latest_dir.join("SOUL_PatchLoader.dylib"),
        patch_dir.join("SOUL_PatchLoader.dylib"),
    );
    symlink(
        latest_dir.join("soul"),
        PathBuf::from("/usr/local/bin").join("soul"),
    );
}

// pub fn unzip_file(zip_file: PathBuf, dest_dir: PathBuf) {
//     let mut zip_file = std::fs::File::open(&zip_file).unwrap();
//     let mut archive = zip::ZipArchive::new(zip_file).unwrap();

//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i).unwrap();

//         if !(&*file.name()).ends_with('/') {
//             let mut dest = fs::File::create(&dest_dir).unwrap();
//             io::copy(&mut file, &mut dest).unwrap();
//         }
//     }
// }
// pub fn unzip_file(zip_file: PathBuf, dest_dir: PathBuf) {
//     let mut zip_file = std::fs::File::open(&zip_file).unwrap();
//     let mut archive = zip::ZipArchive::new(zip_file).unwrap();

//     for i in 0..archive.len() {
//         let mut file = archive.by_index(i).unwrap();

//         #[allow(deprecated)]
//         let dest_path = file.sanitized_name();

//         if !(&*file.name()).ends_with('/') {
//             if let Some(dir) = dest_path.parent() {
//                 if !dir.exists() {
//                     fs::create_dir_all(&dir).unwrap();
//                 }
//             }
//             let mut dest = fs::File::create(&dest_dir).unwrap();
//             io::copy(&mut file, &mut dest).unwrap();
//         }
//     }
// }
