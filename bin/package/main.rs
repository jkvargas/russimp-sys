use flate2::read::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

const LICENSE_FILEPATH: &str = "LICENSE";
const CONFIG_FILEPATH: &str = "include/assimp/config.h";

const fn static_lib() -> &'static str {
    if cfg!(feature = "build-assimp") && cfg!(not(feature = "static-link")) {
        "dylib"
    } else if cfg!(feature = "static-link") {
        "static"
    } else {
        ""
    }
}

fn main() {
    if static_lib().is_empty() {
        panic!("Nothing to package.\nPlease enable either the `build-assimp` or `static-link` feature.");
    }

    let out_dir = PathBuf::from(env!("OUT_DIR"));
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ar_dir = PathBuf::from(option_env!("RUSSIMP_PACKAGE_DIR").unwrap_or(env!("OUT_DIR")));

    let target = russimp_sys::built_info::TARGET;
    let ar_filename = format!(
        "russimp-{}-{}-{}.tar.gz",
        env!("CARGO_PKG_VERSION"),
        target,
        static_lib()
    );

    let from_dir = out_dir.join(static_lib());
    let mut licence = File::open(manifest_dir.join(LICENSE_FILEPATH)).unwrap();
    let mut config_filename = File::open(from_dir.join(CONFIG_FILEPATH)).unwrap();

    fs::create_dir_all(&ar_dir).unwrap();
    let tar_file = File::create(ar_dir.join(&ar_filename)).unwrap();
    let mut archive = tar::Builder::new(GzEncoder::new(tar_file, Compression::default()));

    // On Windows, the dynamic libraries are located in the bin directory.
    if static_lib() == "dylib" && cfg!(target_env = "msvc") {
        archive
            .append_dir_all(format!("bin"), from_dir.join("bin"))
            .unwrap();
    }

    archive
        .append_dir_all(format!("lib"), from_dir.join("lib"))
        .unwrap();
    archive
        .append_file(format!("{}", LICENSE_FILEPATH), &mut licence)
        .unwrap();
    archive
        .append_file(format!("{}", CONFIG_FILEPATH), &mut config_filename)
        .unwrap();

    archive.finish().unwrap();

    println!("Package created at: {}", ar_dir.join(&ar_filename).display());
}
