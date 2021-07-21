use std::{env, path::Path};

fn main() {
    let out_path = Path::new("./src/foundation.rs");

    // Only bindgen if the file doesn't exist already
    if out_path.exists() {
        return;
    }

    let tm_sdk = env::var("TM_SDK_DIR").expect("TM_SDK_DIR environment variable wasn't set");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell it where to find the includes for machinery
        .clang_arg(format!("-I{}/headers", tm_sdk))
        .parse_callbacks(Box::new(ParseCallbacks))
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}

#[derive(Debug)]
pub struct ParseCallbacks;

impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
    fn include_file(&self, filename: &str) {
        println!("cargo:rerun-if-changed={}", filename);
    }
}
