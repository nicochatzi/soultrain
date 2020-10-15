use crate::{download::downloadable::Downloadable, files::utils};

use std::{fs, io};

use semver::Version;

pub fn run() {
    let endpoint = &*crate::github::endpoints::soultrain::RELEASES;
    let releases = crate::github::Releases::pull(endpoint);

    if let Some(latest) = releases.latest() {
        if let Some(message) = check(latest.version()) {
            if has_user_accepted() {
                update(latest);
            }
        }
    }

}

pub fn check(latest: Version) -> Option<String> {
    let current = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    if current < latest {
        return Some(format!(
            "Currently installed version of soultrain is: {}\n\
                                 Latest version is: {},\n\
                                 Would you like to update? [y/n]",
            current.to_string(),
            latest.to_string()
        ));
    }

    None
}

pub fn has_user_accepted() -> bool {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => return input == "y".to_owned(),
        Err(_) => {
            println!("Invalid user input.");
            false
        }
    }
}

pub fn update(release: &crate::github::Release) {
    release.asset().download().unwrap();
    let bin_dir = dirs::home_dir().unwrap().join(".soultrain").join("bin");
    let zip_file = release.asset().file();

    utils::unzip_file(zip_file.clone(), bin_dir.clone());
    fs::remove_file(zip_file);

    utils::set_permissions(bin_dir.join("soultrain"));
}
