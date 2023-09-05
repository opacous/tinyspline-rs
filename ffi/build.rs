use std::env;
use std::path::PathBuf;


#[derive(Debug)]
struct CargoCallbacks;

impl bindgen::callbacks::ParseCallbacks for CargoCallbacks{
    fn process_comment(&self, comment: &str) -> Option<String> {
        Some(format!("````ignore\n{}\n````", comment))
    }
}

fn main() {
    // // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/path/to/lib");
    //
    // // Tell cargo to tell rustc to link the system bzip2
    // // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");


    let project_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut ts_path = project_path;
    ts_path.push("src");

    let files = [
        "tinyspline.c",
    ];



    let mut ts_build = cc::Build::new();

    ts_build
        .include(&ts_path)
        .files(files.iter().map(|file| {
            let mut path = ts_path.clone();
            path.push(file);
            path
        }))
        .warnings(false)
        .opt_level(3);

    ts_build.compile("tinyspline");

    println!("cargo:rustc-link-lib=tinyspline");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks))
        .rustified_enum(".*")
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