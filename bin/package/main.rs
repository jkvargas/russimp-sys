use flate2::read::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

const LICENSE_FILEPATH: &str = "LICENSE";
const CONFIG_FILEPATH: &str = "include/assimp/config.h";

const fn static_lib() -> &'static str {
    return if cfg!(feature = "static-link") {
        "static"
    } else {
        "dylib"
    };
}

fn main() {
    let out_dir = PathBuf::from(env!("OUT_DIR"));
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let target = russimp_sys::built_info::TARGET;
    let ar_filename = format!("russimp-{}-{}.tar.gz", env!("CARGO_PKG_VERSION"), target);

    let from_dir = out_dir.join(static_lib());
    let mut licence = File::open(manifest_dir.join(LICENSE_FILEPATH)).unwrap();
    let mut config_filename = File::open(from_dir.join(CONFIG_FILEPATH)).unwrap();

    fs::create_dir_all(&out_dir).unwrap();
    let tar_file = File::create(out_dir.join("package").join(&ar_filename)).unwrap();
    let mut archive = tar::Builder::new(GzEncoder::new(tar_file, Compression::default()));

    archive
        .append_dir_all(format!("{}/bin", static_lib()), from_dir.join("bin"))
        .unwrap();
    archive
        .append_dir_all(format!("{}/lib", static_lib()), from_dir.join("lib"))
        .unwrap();
    archive
        .append_file(
            format!("{}/{}", static_lib(), LICENSE_FILEPATH),
            &mut licence,
        )
        .unwrap();
    archive
        .append_file(
            format!("{}/{}", static_lib(), CONFIG_FILEPATH),
            &mut config_filename,
        )
        .unwrap();
}
