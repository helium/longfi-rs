// This build script's sole purpose is to build the raw longfi-core
// (`lfc`) library. Since this is a glacially moving API, we generate
// bindings manually and check in the generated code to save the user
// from a slow build step.
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
