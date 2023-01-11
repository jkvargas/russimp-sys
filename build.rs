use std::{env, path::PathBuf};

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

fn lib_names() -> Vec<&'static str> {
    let mut names = Vec::new();

    if cfg!(target_os = "windows") {
        if cfg!(target_env = "gnu") {
            panic!("Windows GNU is not supported, assimp fails to build for some reason.\nSee https://github.com/assimp/assimp/issues/4868");
        } else {
            names.push("assimp-vc143-mt");
        }
    } else {
        names.push("assimp");
    }

    names.push("zlibstatic");

    if cfg!(target_os = "linux") {
        names.push("stdc++");
    }

    if cfg!(target_os = "macos") {
        names.push("c++");
    }

    names
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build Zlib from source?
    let build_zlib = if cfg!(feature = "nozlib") {
        "OFF"
    } else {
        "ON"
    };

    // Build static libs?
    let build_static = if cfg!(feature = "static-link") {
        "OFF"
    } else {
        "ON"
    };

    // CMake
    let mut cmake = cmake::Config::new("assimp");
    cmake
        .profile("Release")
        .static_crt(true)
        .define("BUILD_SHARED_LIBS", build_static)
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("ASSIMP_BUILD_ZLIB", build_zlib);

    // Add compiler flags
    for flag in compiler_flags().iter() {
        cmake.cflag(flag);
        cmake.cxxflag(flag);
    }

    let cmake_dir = cmake.build();

    bindgen::builder()
        .header("wrapper.h")
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
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Could not generate russimp bindings, for details see https://github.com/jkvargas/russimp-sys");

    println!(
        "cargo:rustc-link-search=native={}",
        cmake_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        cmake_dir.join("bin").display()
    );

    for n in lib_names().iter() {
        println!("cargo:rustc-link-lib=static={}", n);
    }
}
