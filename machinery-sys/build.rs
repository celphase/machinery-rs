use std::{env, path::Path};

fn main() {
    generate("./headers/foundation.h", "./src/foundation.rs");
    generate("./headers/plugins/the_machinery_shared.h", "./src/plugins/the_machinery_shared.rs");
}

fn generate(input: &str, output: &str) {
    let out_path = Path::new(output);

    // Only bindgen if the file doesn't exist already
    if out_path.exists() {
        return;
    }

    let tm_sdk = env::var("TM_SDK_DIR").expect("TM_SDK_DIR environment variable wasn't set");

    let bindings = bindgen::Builder::default()
        .header(input)
        // Tell clang where to find the includes for machinery
        .clang_arg(format!("-I{}/headers", tm_sdk))
        .prepend_enum_name(false)
        .derive_debug(false)
        .derive_default(true)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings");
}
