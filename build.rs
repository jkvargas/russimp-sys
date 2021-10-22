#![allow(unused_imports, dead_code, unused_variables)]

use std::{env::var, path::PathBuf};
use vcpkg::{find_package, Library};

const BINDINGS_FILE: &str = "bindings.rs";
const WRAPPER_FILE: &str = "wrapper.h";

fn main() {
    let (include, libdir, libname) = assimp_lib_data();

    if cfg!(target_os = "windows") {
        let result = std::process::Command::new("cargo")
            .arg("vcpkg")
            .arg("build")
            .output()
            .unwrap();
    }

    let mut builder = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include))
        .header(WRAPPER_FILE)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_type("ai.*")
        .whitelist_function("ai.*")
        .whitelist_var("ai.*")
        .whitelist_var("AI_.*")
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_debug(true);

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
    let target = std::env::var("TARGET").unwrap();
    let vcpkg_root = std::env::var("VCPKG_ROOT").unwrap_or("target/vcpkg".to_string());

    let include_path = if target.contains("apple") { "/opt/homebrew/opt/assimp/include" } else { "/usr/local/include"};

    let mut lib = vcpkg::Config::new()
        .vcpkg_root(vcpkg_root.into())
        .find_package("assimp")
        .unwrap_or(Library {
            include_paths: vec![PathBuf::from(include_path)],
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

    if cfg!(target_os = "windows") {
        // Following dependencies are pulled in via Irrlicht.
        // vcpkg doesn't know how to find these system dependencies, so we list them here.
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=gdi32");
        lib.link_paths[0] = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join(lib.link_paths[0].as_path()).into();
    }

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

fn get_output_path(content: &str) -> String {
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
