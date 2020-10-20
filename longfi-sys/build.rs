use cmake;

// Builds the project in the directory located in `libfoo`, installing it
// into $OUT_DIR
//
fn main() {
    let install_dir = cmake::build("vendor");

    let includedir = install_dir.join("include");
    let libdir = install_dir.join("lib");
    let libname = "lfc";

    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:root={}", install_dir.display());
    println!("cargo:include={}", includedir.display());
    println!("cargo:rustc-link-lib=static={}", libname);
}
