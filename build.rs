extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let hbs = pkg_config::find_library("hbs");
    let hbs_acc = pkg_config::find_library("hbs-acc");
    let hbs_pow = pkg_config::find_library("hbs-pow");
    let hbs_acc_pow = pkg_config::find_library("hbs-acc-pow");
    if hbs.is_err() || hbs_acc.is_err() || hbs_pow.is_err() || hbs_acc_pow.is_err() {
        let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                               .join("heartbeats-simple");
        let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap());
        let _ = fs::create_dir(&dst);
        run(Command::new("make").current_dir(&src));
        println!("cargo:rustc-flags=-l hbs -l hbs-acc -l hbs-pow -l hbs-acc-pow");
        println!("cargo:rustc-flags=-L {}", dst.display());
    }
}

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}
