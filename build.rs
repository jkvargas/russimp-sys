mod build_support;

use std::{
    env, io,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

const BINDINGS_FILE: &str = "bindings.rs";
const WRAPPER_FILE: &str = "wrapper.h";

// Compiler specific compiler flags for CMake
fn compiler_flags() -> Vec<&'static str> {
    let mut flags = Vec::new();

    if cfg!(target_env = "msvc") {
        flags.push("/EHsc");

        // Find Ninja
        if which::which("ninja").is_ok() {
            env::set_var("CMAKE_GENERATOR", "Ninja");
        }
    }

    flags
}

fn lib_name() -> &'static str {
    return if cfg!(target_env = "msvc") {
        "assimp-vc143-mt"
    } else {
        "assimp"
    };
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build Zlib from source?
    let build_zlib = if cfg!(feature = "nozlib") {
        "OFF"
    } else {
        "ON"
    };

    // CMake
    let mut cmake = cmake::Config::new("assimp");
    cmake
        .profile("Release")
        .static_crt(true)
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("ASSIMP_BUILD_ZLIB", build_zlib);

    if cfg!(target_os = "windows") && cfg!(target_env = "gnu") {
        panic!("Windows GNU is not supported, assimp fails to build for some reason\nSee https://github.com/assimp/assimp/issues/4868");
    }

    // Add compiler flags
    for flag in compiler_flags().iter() {
        cmake.cflag(flag);
        cmake.cxxflag(flag);
    }

    let cmake_dir = cmake.build();

    bindgen::builder()
        .header(WRAPPER_FILE)
        .clang_arg(format!("-I{}", out_dir.join("include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("ai.*")
        .allowlist_function("ai.*")
        .allowlist_var("ai.*")
        .allowlist_var("AI_.*")
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_debug(true)
        .generate()
        .unwrap()
        .write_to_file(out_dir.join(BINDINGS_FILE))
        .expect("Could not generate russimp bindings, for details see https://github.com/jkvargas/russimp-sys");

    println!(
        "cargo:rustc-link-search=native={}",
        cmake_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static={}", lib_name());
    println!("cargo:rustc-link-lib=static=zlibstatic");
}
