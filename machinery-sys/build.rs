use std::{env, path::Path};

fn main() {
    let out_path = Path::new("./src/foundation.rs");

    // Only bindgen if the file doesn't exist already
    if out_path.exists() {
        return;
    }

    let tm_sdk = env::var("TM_SDK_DIR").expect("TM_SDK_DIR environment variable wasn't set");

    let bindings = bindgen::Builder::default()
        .header("foundation.h")
        // Tell clang where to find the includes for machinery
        .clang_arg(format!("-I{}/headers", tm_sdk))
        .clang_arg("-fretain-comments-from-system-headers")
        .clang_arg("-fparse-all-comments")
        .prepend_enum_name(false)
        .derive_debug(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}
