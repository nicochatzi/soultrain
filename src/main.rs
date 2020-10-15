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

fn main() {
    // soultrain::auto_update::run();
    let endpoint = &*github::endpoints::soul::RELEASES;
    let releases = github::Releases::pull(endpoint);
    let installer = Installer::new(&releases);

    match App::from_args() {
        App::Update => installer.install_version("latest"),
        App::Select { version } => installer.install_version(&version),
        App::Show => println!("{}", installer.current_version()),
        App::List { length } => println!("{}", releases.list(length)),
        App::Latest => println!("{}", releases.latest().unwrap().version()),
        App::Uninstall => installer.uninstall(),
        App::Cleanup => installer.cleanup(),
    }
}
