fn main() {
	println!("cargo:rustc-link-lib=hbs");
	println!("cargo:rustc-link-lib=hbs-acc");
    println!("cargo:rustc-link-lib=hbs-pow");
    println!("cargo:rustc-link-lib=hbs-acc-pow");
}
