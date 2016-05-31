extern crate clap;
extern crate regex;

use clap::{App, ErrorKind};

include!("../clap-test.rs");

static VERSION: &'static str = "clap-test v1.4.8";

#[test]
fn version_short() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "-V"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn version_long() {
    let m = App::new("test")
        .author("Kevin K.")
        .about("tests stuff")
        .version("1.3")
        .get_matches_from_safe(vec!["myprog", "--version"]);

    assert!(m.is_err());
    assert_eq!(m.unwrap_err().kind, ErrorKind::VersionDisplayed);
}

#[test]
fn complex_version_output() {
    test::check_version(test::complex_app(), VERSION);
}
