use structopt::StructOpt;
use soultrain::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "soultrain", about = "SOUL Version Manager")]
enum App {
    /// Update to the newest version of SOUL.
    Update,
    /// Select and use a specific version of SOUL.
    /// To select the latest version pass "latest".
    /// To select the previous version pass "previous".
    Select { version: String },
    /// Select and use a specific version of SOUL.
    Show,
    /// List all available versions of SOUL.
    /// An number can be supplied to specify
    /// the length of the list. Otherwise, it
    /// will default to the last 5 releases.
    List { length: Option<usize> },
    /// Displays the latest available version of SOUL.
    Latest,
    /// Uninstalls the currently installed version of SOUL.
    Uninstall,
    /// Clears all cached versions except the one currently used.
    /// If `uninstall` is called before this, all versions
    /// will be removed.
    Cleanup,
}

fn get_soul_releases() -> github::Releases {
    let endpoint = &*github::endpoints::soul::RELEASES;
    github::Releases::pull(endpoint)
}

fn main() {
    soultrain::installer::auto_update::run();

    match App::from_args() {
        App::Update => {
            let releases = get_soul_releases();
            Installer::new(&releases).install_version("latest")
        },
        App::Select { version } => {
            let releases = get_soul_releases();
            Installer::new(&releases).install_version(&version)
        },
        App::Show => {
            let releases = get_soul_releases();
            println!("{}", Installer::new(&releases).current_version())
        },
        App::List { length } => {
            println!("{}", get_soul_releases().list(length))
        },
        App::Latest => {
            println!("{}", get_soul_releases().latest().unwrap().version())
        },
        App::Uninstall => {
            let releases = get_soul_releases();
            Installer::new(&releases).uninstall()
        },
        App::Cleanup => {
            let releases = get_soul_releases();
            Installer::new(&releases).cleanup()
        },
    }
}
