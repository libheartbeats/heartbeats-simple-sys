extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::path::PathBuf;

pub fn find_or_build(lib: &str) {
    match pkg_config::find_library(lib) {
        Ok(_) => (),
        Err(_) => {
            let src = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/heartbeats-simple"));
            let mut config = Config::new(&src);
            config.define("BUILD_SHARED_LIBS", "false");

            // check for Android
            let target: String = env::var("TARGET").unwrap();
            let target_parts: Vec<&str> = target.split('-').collect();
            if target_parts[target_parts.len() - 1].starts_with("android") {
                config.define("CMAKE_TOOLCHAIN_FILE",
                              src.join("cmake-toolchain").join("android.toolchain.cmake"));
            }

            // build/install everything - the build is fast and portable enough
            // cmake crate makes it too troublesome to do individual targets since it expects to install
            let dst: PathBuf = config.build();
            // none of the libraries have transitive dependencies
            println!("cargo:rustc-link-lib=static={}", lib);
            println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
        },
    }
}
