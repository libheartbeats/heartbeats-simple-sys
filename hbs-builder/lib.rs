extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

pub fn find_or_build(lib: &str) {
    match pkg_config::find_library(lib) {
        Ok(_) => (),
        Err(_) => {
            // get source and build directories
            let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                                   .parent().unwrap().join("heartbeats-simple");
            let build = PathBuf::from(&env::var_os("OUT_DIR").unwrap()).join("_build");
            // get extra CMake parameters
            let target: String = env::var("TARGET").unwrap();
            let target_parts: Vec<&str> = target.split('-').collect();
            let cmake_toolchain = match target_parts[target_parts.len() - 1].starts_with("android") {
                true => format!("-DCMAKE_TOOLCHAIN_FILE={}",
                                src.join("cmake-toolchain").join("android.toolchain.cmake").display()),
                false => "".to_owned(),
            };
            let cmake_mingw = match env::var("MSYSTEM") {
                Ok(val) => {
                    if val.contains("MINGW") {
                        "-GMSYS Makefiles".to_owned()
                    } else {
                        "".to_owned()
                    }
                },
                Err(_) => "".to_owned(),
            };
            // always remake the build directory
            fs::remove_dir_all(&build).ok();
            fs::create_dir_all(&build).unwrap();
            // run the build commands
            run(Command::new("cmake").arg("-DBUILD_SHARED_LIBS=false").arg(cmake_toolchain).arg(&cmake_mingw)
                .arg(src.to_str().unwrap()).current_dir(&build));
            run(Command::new("make").arg(lib).current_dir(&build));
            // none of the libraries have transitive dependencies
            println!("cargo:rustc-link-lib=static={}", lib);
            println!("cargo:rustc-link-search=native={}", build.join("lib").display());
        },
    }
}

fn run(cmd: &mut Command) {
    match cmd.status() {
        Ok(status) => assert!(status.success()),
        Err(e) => panic!("Unable to execute {:?}! {}", cmd, e),
    }
}
