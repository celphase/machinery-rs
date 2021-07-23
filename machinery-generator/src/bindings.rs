use std::{
    collections::HashSet,
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
) {
    let mut wrapper = String::new();

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

        writeln!(&mut wrapper, "#include <{}>", header_path).unwrap();
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
}

#[derive(Debug)]
struct TmCallbacks;
