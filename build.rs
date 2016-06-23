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

            // check for Android
            let target: String = env::var("TARGET").unwrap();
            let target_parts: Vec<&str> = target.split('-').collect();
            if target_parts[target_parts.len() - 1].starts_with("android") {
                config.define("CMAKE_TOOLCHAIN_FILE",
                              src.join("cmake-toolchain").join("android.toolchain.cmake"));
                // the cmake toolchain above has some specific ideas of what
                // needs to be set
                env::set_var("ANDROID_STANDALONE_TOOLCHAIN", env::var("ANDROID_TOOLCHAIN").unwrap());
                env::remove_var("ANDROID_NDK");
            }

            // build/install everything - the build is fast and portable enough
            // cmake crate makes it too troublesome to do individual targets since it expects to install
            let dst: PathBuf = config.build();
            // none of the libraries have transitive dependencies
            println!("cargo:rustc-link-lib=static=heartbeats-simple");
            println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
        },
    }
}
