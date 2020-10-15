mod utils;

use crate::utils::*;

#[test]
fn fresh_install_of_latest_version() {
    sys::remove_soul();
    sys::remove_soultrain();

    let _ = cmd::soultrain::select("latest").unwrap();
    let output = cmd::soultrain::show().unwrap();

    assert!(output.stderr.is_empty());
    assert!(sys::is_soul_installed());

    sys::remove_soultrain();
    sys::remove_soul();
}

#[test]
fn fresh_install_of_previous_version() {
    sys::remove_soul();
    sys::remove_soultrain();

    let _ = cmd::soultrain::select("previous").unwrap();
    let output = cmd::soultrain::show().unwrap();

    assert!(output.stderr.is_empty());
    assert!(sys::is_soul_installed());

    sys::remove_soultrain();
    sys::remove_soul();
}
