#![allow(unused_imports, dead_code, unused_variables)]

use std::{env::var, path::PathBuf};

const BINDINGS_FILE: &str = "bindings.rs";
const WRAPPER_FILE: &str = "wrapper.h";

fn main() {
    // let output_path = PathBuf::from(var("OUT_DIR").expect("env variable OUT_DIR not found"));
    // let path_bindings_buf_src = output_path.join(BINDINGS_FILE);
    // let path_bindings_file_src = path_bindings_buf_src.as_os_str().to_str().unwrap();

    if cfg!(windows) {
        println!("cargo:rustc-link-search={}", assimp_path("lib").as_str());
        println!("cargo:include={}", assimp_path("include").as_str());
    } else {
        println!("cargo:rustc-link-search={}", "/usr/local/lib");
        println!("cargo:include={}", "/usr/local/include");
    }

    // bindgen::Builder::default()
    //     .header(WRAPPER_FILE)
    //     .clang_args(&["-I", "/usr/include"])
    //     .whitelist_function("aiImportFile")
    //     .whitelist_type("aiPostProcessSteps")
    //     .whitelist_type("aiPrimitiveType")
    //     .whitelist_type("aiTextureType")
    //     .whitelist_function("aiReleaseImport")
    //     .whitelist_function("aiGetErrorString")
    //     .generate()
    //     .unwrap()
    //     .write_to_file(path_bindings_file_src)
    //     .unwrap();

    println!("cargo:rustc-flags=-l assimp");
}

fn assimp_path(relative_path: &str) -> String {
    let mut assimp_install_path = std::env::var("GITHUB_WORKSPACE").unwrap();

    assimp_install_path.push_str("\\out\\");
    assimp_install_path.push_str(relative_path);

    assimp_install_path
}