mod bindings;
mod callers;
mod config;

use std::{env, fs};

use crate::config::Config;

fn main() {
    let tm_sdk = env::var("TM_SDK_DIR").expect("TM_SDK_DIR environment variable wasn't set");
    let config_toml =
        fs::read_to_string("./Machinery.toml").expect("Unable to find Machinery.toml");

    let config: Config = toml::from_str(&config_toml).unwrap();

    let mut blocklist = Vec::new();

    for (name, project) in config.projects {
        println!("Generating \"{}\"", name);

        bindings::generate(&tm_sdk, &project, &blocklist);
        callers::generate(&project, &mut blocklist);
    }
}
