#![allow(unused_imports, dead_code, unused_variables)]

use std::env::var;

fn main() {
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

fn assimp_path(relative_path: &str) -> String {
    let mut assimp_install_path = std::env::var("GITHUB_WORKSPACE").unwrap();

    assimp_install_path.push_str("\\");
    assimp_install_path.push_str(relative_path);

    assimp_install_path
}
