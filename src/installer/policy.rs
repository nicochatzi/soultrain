
enum LocationRoot {
    Home,
    Executable,
    LocalData,
}

enum LocationPath {
    Release,
    Named(String)
}

struct InstallationLocation {
    root: LocationRoot,
    path: LocationPath,
    file: String,
}

/// Raw path to a file in a release.
type TargetFile = String;

enum InstallationStep {
    PlaceFile(TargetFile, InstallationLocation),
    LinkFile(InstallationLocation, InstallationLocation),
    SetExecutable(InstallationLocation)
}

struct InstallerPolicy {
    steps: Vec<InstallationStep>,
}

struct InstallationRunner;
impl InstallationRunner {
    pub fn run(policy: &InstallerPolicy, release: &Release) {

    }
}