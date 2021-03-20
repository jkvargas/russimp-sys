#![allow(unused_imports, dead_code, unused_variables)]

use std::{env::var, path::PathBuf};
use vcpkg::{find_package, Library};

const BINDINGS_FILE: &str = "bindings.rs";
const WRAPPER_FILE: &str = "wrapper.h";

fn main() {
    let (include, libdir, libname) = assimp_lib_data();

    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include))
        .header(WRAPPER_FILE)
        .whitelist_type("aiPostProcessSteps")
        .whitelist_type("aiPrimitiveType")
        .whitelist_type("aiTextureType")
        .whitelist_function("aiImportFile")
        .whitelist_function("aiImportFileFromMemory")
        .whitelist_function("aiReleaseImport")
        .whitelist_function("aiGetErrorString");

    if cfg!(target_os = "windows") {
        builder = builder.generate_comments(false);
    }

    builder
        .generate()
        .unwrap()
        .write_to_file(get_output_path(BINDINGS_FILE))
        .unwrap();

    println!("cargo:rustc-link-search={}", libdir);
    println!("cargo:include={}", include);
    println!("cargo:rustc-link-lib={}", libname);
}

fn assimp_lib_data() -> (String, String, String) {
    let lib = find_package("assimp").unwrap_or(Library {
        include_paths: vec![PathBuf::from("/usr/local/include")],
        link_paths: vec![PathBuf::from("/usr/local/lib")],
        found_names: vec!["assimp".to_owned()],

        ports: vec![],
        cargo_metadata: vec![],
        dll_paths: vec![],
        found_dlls: vec![],

        is_static: false,
        found_libs: vec![],
        vcpkg_triplet: "".to_string(),
    });

    (
        lib.include_paths[0].to_str().unwrap().to_owned(),
        lib.link_paths[0].to_str().unwrap().to_owned(),
        lib.found_names
            .iter()
            .filter(|n| n.starts_with("assimp"))
            .nth(0)
            .unwrap()
            .to_owned(),
    )
}

fn get_output_path<'a>(content: &str) -> String {
    let output_path = PathBuf::from(var("OUT_DIR").expect("env variable OUT_DIR not found"));
    let path_bindings_buf_src = output_path.join(content);
    path_bindings_buf_src
        .as_os_str()
        .to_str()
        .unwrap()
        .to_string()
}

fn assimp_path(relative_path: &str) -> String {
    let mut assimp_install_path = std::env::var("GITHUB_WORKSPACE").unwrap();

    assimp_install_path.push_str("\\");
    assimp_install_path.push_str(relative_path);

    assimp_install_path
}
