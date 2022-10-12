
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-arg=foo.o");
    println!("cargo:rustc-link-search=./");
    println!("cargo:rustc-link-lib=bar");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/mnt/disk/xx/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
