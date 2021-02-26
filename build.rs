#![allow(unused_imports, dead_code, unused_variables)]

use std::env::var;

const BINDINGS_FILE: &str = "bindings.rs";
const WRAPPER_FILE: &str = "wrapper.h";

fn main() {
    bindgen::Builder::default()
        .header(WRAPPER_FILE)
        .whitelist_type("aiPostProcessSteps")
        .whitelist_type("aiPrimitiveType")
        .whitelist_type("aiTextureType")
        .whitelist_function("aiImportFile")
        .whitelist_function("aiImportFileFromMemory")
        .whitelist_function("aiReleaseImport")
        .whitelist_function("aiGetErrorString")
        .generate()
        .unwrap()
        .write_to_file(get_output_path(BINDINGS_FILE))
        .unwrap();

    if cfg!(windows) {
        println!(
            "cargo:rustc-link-search={}",
            assimp_path("vcpkg\\installed\\x64-windows\\lib").as_str()
        );
        println!(
            "cargo:include={}",
            assimp_path("vcpkg\\installed\\x64-windows\\include").as_str()
        );
        println!("cargo:rustc-link-lib=static=assimp-vc142-mt");
    } else {
        println!("cargo:rustc-link-search={}", "/usr/local/lib");
        println!("cargo:include={}", "/usr/local/include");
        println!("cargo:rustc-flags=-l assimp");
    }
}

fn get_output_path<'a>(content: &str) -> String {
    let output_path = PathBuf::from(var("OUT_DIR").expect("env variable OUT_DIR not found"));
    let path_bindings_buf_src = output_path.join(content);
    path_bindings_buf_src.as_os_str().to_str().unwrap().to_string()
}

fn assimp_path(relative_path: &str) -> String {
    let mut assimp_install_path = std::env::var("GITHUB_WORKSPACE").unwrap();

    assimp_install_path.push_str("\\");
    assimp_install_path.push_str(relative_path);

    assimp_install_path
}
