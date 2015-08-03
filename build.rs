extern crate pkg_config;

fn main() {
    pkg_config::find_library("hbs").unwrap();
    pkg_config::find_library("hbs-acc").unwrap();
    pkg_config::find_library("hbs-pow").unwrap();
    pkg_config::find_library("hbs-acc-pow").unwrap();
}
