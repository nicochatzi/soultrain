use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "soultrain", about = "SOUL Version Manager")]
enum App {
    /// Update to the newest version of SOUL.
    Update,
    /// Select and use a specific version of SOUL.
    Show,
    // /// Select and use a specific version of SOUL.
    // Select,
    /// List all available versions of SOUL.
    List,
    /// Get the latest version number of SOUL.
    Latest,
}

fn main() {
    match App::from_args() {
        App::Update => soultrain::install_latest(),
        // App::Select => soultrain::install_version(version),
        App::Show => println!("{}", soultrain::current_version()),
        App::List => println!("{}", soultrain::list_releases()),
        App::Latest => println!("{}", soultrain::latest_version()),
    }
}
