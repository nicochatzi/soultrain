macro_rules! github_endpoints {
    ($owner:expr, $repo:expr) => {
        lazy_static::lazy_static! {
            pub static ref RELEASES: reqwest::Url = {
                reqwest::Url::parse(
                    &format!("https://api.github.com/repos/{}/{}/releases", $owner, $repo)
                ).unwrap()
            };
            pub static ref LATEST_RELEASE: reqwest::Url = {
                reqwest::Url::parse(
                    &format!("https://api.github.com/repos/{}/{}/releases/latest", $owner, $repo)
                ).unwrap()
            };
        }
    };
}

pub mod soul {
    github_endpoints!("soul-lang", "SOUL");
}

pub mod soultrain {
    github_endpoints!("nicochatzi", "soultrain");
}
