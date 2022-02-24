extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

fn main() {
    let capstone_res = Command::new("make")
        .args(&["-C", "./capstone", "-j"])
        .output()
        .expect("could not `make` capstone disassembly framework");
    if !capstone_res.status.success() {
        eprintln!("Error building libcapstone");
        eprintln!("Stdout:\n{}", from_utf8(&capstone_res.stdout).expect("Could not decode stdout"));
        eprintln!("Stderr:\n{}", from_utf8(&capstone_res.stderr).expect("Could not decode stderr"));
        panic!("Could not build libcapstone")
    }

    let libxdc_res = Command::new("make")
        .args(&["-C", "./libxdc"])
        .env("CFLAGS", "-I../capstone/include/")
        .env("LDFLAGS", "-L../capstone")
        .output()
        .expect("could not `make` libxdc");
    if !libxdc_res.status.success() {
        eprintln!("Error building libxdc!");
        eprintln!("Stdout:\n{}", from_utf8(&libxdc_res.stdout).expect("Could not decode stdout"));
        eprintln!("Stderr:\n{}", from_utf8(&libxdc_res.stderr).expect("Could not decode stderr"));
        panic!("Could not build libxdc")
    }

    println!("cargo:rustc-link-lib=static=xdc");
    println!("cargo:rustc-link-search=./libxdc/");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=./libxdc/libxdc.a");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}