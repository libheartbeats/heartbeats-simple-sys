extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    match pkg_config::find_library("heartbeats-simple") {
        Ok(_) => (),
        Err(_) => {
            let src = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/heartbeats-simple"));
            let mut config = Config::new(&src);

            // build/install everything - the build is fast and portable enough
            // cmake crate makes it too troublesome to do individual targets since it expects to install
            let dst: PathBuf = config.build();
            // none of the libraries have transitive dependencies
            println!("cargo:rustc-link-lib=static=heartbeats-simple");
            println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
        },
    }
}
