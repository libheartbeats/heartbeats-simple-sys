extern crate pkg_config;

use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    match pkg_config::find_library("hbs-pow-static") {
        Ok(_) => (),
        Err(_) => {
            let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                                   .parent().unwrap().join("heartbeats-simple");
            let build = PathBuf::from(&env::var_os("OUT_DIR").unwrap()).join("_build");
            let target: String = env::var("TARGET").unwrap();
            let target_parts: Vec<&str> = target.split('-').collect();
            let cmake_var = match target_parts[target_parts.len() - 1].starts_with("android") {
                true => format!("-DCMAKE_TOOLCHAIN_FILE={}",
                                src.join("cmake-toolchain").join("android.toolchain.cmake").display()),
                false => "".to_owned(),
            };
            fs::remove_dir_all(&build).ok();
            fs::create_dir_all(&build).unwrap();
            run(Command::new("cmake").arg(cmake_var).arg(src.to_str().unwrap()).current_dir(&build));
            run(Command::new("make").arg("hbs-pow-static").current_dir(&build));
            println!("cargo:rustc-link-lib=static=hbs-pow-static");
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
