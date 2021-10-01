use std::{
    collections::{HashMap, HashSet},
    fmt::Write,
    path::{Path, PathBuf},
};

use crate::config::Project;

pub fn generate(
    tm_sdk: &str,
    project: &Project,
    headers_dir: &Path,
    target_headers: &[PathBuf],
    blocklist: &HashSet<String>,
) -> HashMap<String, (u32, u32, u32)> {
    let mut wrapper = String::new();
    let mut versions = HashMap::new();

    // Anonymous structs aren't correctly picked up by clang, so always fallback to explicit super
    wrapper.push_str("#define TM_DISABLE_INHERIT\n");

    if let Some(ref prefix_headers) = project.prefix_headers {
        for header in prefix_headers {
            writeln!(&mut wrapper, "#include <{}>", header).unwrap();
        }
    }

    for header in target_headers {
        // Create the header string relative to the directory plus the prefix
        let stripped = header.strip_prefix(&headers_dir).unwrap();
        let header_path = Path::new(&project.headers_prefix).join(stripped);
        let header_path = header_path.to_str().unwrap().replace('\\', "/");

        // Write the header to the include wrapper
        writeln!(&mut wrapper, "#include <{}>", header_path).unwrap();

        // Also scan the header for any version defines
        let header_str = std::fs::read_to_string(header).unwrap();
        for line in header_str.lines() {
            if !line.starts_with("#define")
                || !line.contains("TM_VERSION(")
                || line.contains("TM_LITERAL")
            {
                continue;
            }

            let sections: Vec<_> = line.split(" TM_VERSION(").collect();
            let api_name = sections[0][8..].trim_end_matches("_version").to_string();

            let numbers: Vec<_> = sections[1]
                .trim_end_matches(')')
                .split(", ")
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            versions.insert(api_name, (numbers[0], numbers[1], numbers[2]));
        }
    }

    let mut builder = bindgen::Builder::default()
        .header_contents("wrapper.h", &wrapper)
        // Tell clang where to find the includes for machinery
        .clang_arg(format!("-I{}/headers", tm_sdk))
        .prepend_enum_name(false)
        .derive_debug(false)
        .derive_default(true)
        .layout_tests(false);

    for item in blocklist {
        builder = builder.blocklist_item(item);
    }
    if let Some(project_blocklist) = &project.blocklist {
        for item in project_blocklist {
            builder = builder.blocklist_item(item.clone());
        }
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(&project.target)
        .expect("Couldn't write bindings");

    versions
}

#[derive(Debug)]
struct TmCallbacks;
