mod bindings;
mod config;
mod extensions;

use std::{env, ffi::OsStr, fs, path::PathBuf};

use walkdir::WalkDir;

use crate::config::Config;

fn main() {
    let tm_sdk = env::var("TM_SDK_DIR").expect("TM_SDK_DIR environment variable wasn't set");
    let config_toml =
        fs::read_to_string("./Machinery.toml").expect("Unable to find Machinery.toml");

    let config: Config = toml::from_str(&config_toml).unwrap();

    let mut blocklist = Vec::new();

    for (name, project) in config.projects {
        println!("Generating \"{}\"", name);

        // Go over all headers in the target headers directory to generate the wrapper
        let mut target_headers = Vec::new();
        let headers_dir = parse_path(&tm_sdk, &project.headers);
        for entry in WalkDir::new(&headers_dir) {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() || entry.path().extension() != Some(OsStr::new("h")) {
                continue;
            }

            target_headers.push(entry.into_path());
        }

        bindings::generate(&tm_sdk, &project, &headers_dir, &target_headers, &blocklist);
        extensions::generate(&project, &target_headers, &mut blocklist);
    }
}

fn parse_path(tm_sdk: &str, input: &str) -> PathBuf {
    PathBuf::from(input.replace("$TM_SDK_DIR", tm_sdk))
}
