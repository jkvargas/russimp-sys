use russimp_sys::*;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;

const LICENSE_FILEPATH: &str = "LICENSE";

const fn static_lib() -> &'static str {
    if cfg!(feature = "build-assimp") && cfg!(not(feature = "static-link")) {
        "dylib"
    } else if cfg!(feature = "static-link") {
        "static"
    } else {
        ""
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if static_lib().is_empty() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "You must specify either the `static-link` or `build-assimp` feature",
        )));
    }

    let out_dir = PathBuf::from(env!("OUT_DIR"));
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let ar_dst_dir = PathBuf::from(option_env!("RUSSIMP_PACKAGE_DIR").unwrap_or(env!("OUT_DIR")));

    let target = russimp_sys::built_info::TARGET;
    let ar_filename = format!(
        "russimp-{}-{}-{}.tar.gz",
        env!("CARGO_PKG_VERSION"),
        target,
        static_lib()
    );

    let from_dir = out_dir.join(static_lib());
    let mut licence = File::open(manifest_dir.join(LICENSE_FILEPATH))?;

    fs::create_dir_all(&ar_dst_dir)?;
    println!("Packaging at: {}", ar_dst_dir.display());

    let tar_file = File::create(ar_dst_dir.join(&ar_filename))?;
    let mut archive = tar::Builder::new(GzEncoder::new(tar_file, Compression::best()));

    // On Windows, the dynamic libraries are located in the bin directory.
    if static_lib() == "dylib" && cfg!(target_env = "msvc") {
        archive.append_dir_all(format!("bin"), from_dir.join("bin"))?;
    }

    archive.append_dir_all("include", from_dir.join("include"))?;
    archive.append_dir_all(format!("lib"), from_dir.join("lib"))?;
    archive.append_file(format!("{}", LICENSE_FILEPATH), &mut licence)?;

    archive.finish()?;

    let (major, minor, patch) = unsafe {
        (
            aiGetVersionMajor(),
            aiGetVersionMinor(),
            aiGetVersionPatch(),
        )
    };

    println!(
        "Package created at: {}\nAssimp version: {}.{}.{}",
        ar_dst_dir.join(&ar_filename).display(),
        major,
        minor,
        patch,
    );

    Ok(())
}
