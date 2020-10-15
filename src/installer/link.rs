use crate::files::directory;

use std::{fs, path::{Path, PathBuf}};

fn symlink<P, Q>(src: P, dst: Q) -> std::io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    return std::os::unix::fs::symlink(src, dst);

    #[cfg(target_os = "windows")]
    return std::os::windows::fs::symlink_file(src, dst);

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    panic!("Unsupported OS");
}

pub fn setup(release_dir: PathBuf) {
    setup_latest_links(release_dir);
    setup_install_links();
}

fn setup_latest_links(release_dir: PathBuf) {
    let release_dir = release_dir.join("osx").join("x64");
    let latest_dir = directory::latest();
    fs::create_dir_all(latest_dir.clone());
    let link = |file| {
        fs::remove_file(latest_dir.clone().join(file));
        symlink(release_dir.join(file), latest_dir.clone().join(file));
    };
    link("SOUL_PatchLoader.dylib");
    link("soul");
}

fn setup_install_links() {
    let latest_dir = directory::latest();
    let patch_dir = dirs::data_local_dir().unwrap().join("SOUL");
    fs::create_dir_all(patch_dir.clone());

    symlink(
        latest_dir.join("SOUL_PatchLoader.dylib"),
        patch_dir.join("SOUL_PatchLoader.dylib"),
    );
    symlink(
        latest_dir.join("soul"),
        PathBuf::from("/usr/local/bin/soul"),
    );
}

pub fn remove_links() {
    remove_latest_links();
    remove_install_links();
}

fn remove_latest_links() {
    fs::remove_file(directory::latest().join("soul"));
    fs::remove_file(directory::latest().join("SOUL_PatchLoader.dylib"));
}

fn remove_install_links() {
    fs::remove_file(
        dirs::data_local_dir()
            .unwrap()
            .join("SOUL")
            .join("SOUL_PatchLoader.dylib"),
    );
    fs::remove_file(PathBuf::from("/usr/local/bin/soul"));
}
