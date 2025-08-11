use flate2::read::GzDecoder;
use std::{env, fs::{self, File}, io, path::PathBuf};

struct Library(&'static str, &'static str);

const fn static_lib() -> &'static str {
    if cfg!(feature = "static-link") {
        "static"
    } else {
        "dylib"
    }
}

const fn build_zlib() -> bool {
    cfg!(not(feature = "nozlib"))
}

const fn build_assimp() -> bool {
    cfg!(feature = "build-assimp")
}

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

fn lib_names() -> Vec<Library> {
    let mut names = Vec::new();

    names.push(Library("assimp", static_lib()));

    if build_assimp() && build_zlib() {
        names.push(Library("zlibstatic", "static"));
    } else {
        if cfg!(target_os = "windows") {
            names.push(Library("zlibstatic", "dylib"));
        } else {
            names.push(Library("z", "dylib"));
        }
    }

    if cfg!(target_os = "linux") {
        names.push(Library("stdc++", "dylib"));
    }

    if cfg!(target_os = "macos") {
        names.push(Library("c++", "dylib"));
    }

    names
}

fn build_from_source() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Ensure the assimp source directory is cloned and can compile. The lack of this was causing the previous issue. 
    let assimp_src_dir = match ensure_submodules() {
        Ok(path) => path,
        Err(e) => {show_error_mgs(e); panic!("Extra panic! Ahhh!")},
    };

    // Build Zlib from source?
    let build_zlib = if build_zlib() { "ON" } else { "OFF" };

    // Build static libs?
    let build_shared = if static_lib() == "static" {
        "OFF"
    } else {
        "ON"
    };

    // CMake
    let mut cmake = cmake::Config::new(&assimp_src_dir);
    cmake
        .profile("Release")
        .static_crt(true)
        .out_dir(out_dir.join(static_lib()))
        .define("BUILD_SHARED_LIBS", build_shared)
        .define("ASSIMP_BUILD_ASSIMP_TOOLS", "OFF")
        .define("ASSIMP_BUILD_TESTS", "OFF")
        .define("ASSIMP_BUILD_ZLIB", build_zlib)
        // Disable being overly strict with warnings, which can cause build issues
        // such as: https://github.com/assimp/assimp/issues/5315
        .define("ASSIMP_WARNINGS_AS_ERRORS", "OFF")
        .define("LIBRARY_SUFFIX", "");

    // Add compiler flags
    for flag in compiler_flags().iter() {
        cmake.cflag(flag);
        cmake.cxxflag(flag);
    }

    let cmake_dir = cmake.build();

    let assimp_lib_path = cmake_dir.join("lib").join("assimp.lib");
//     if !assimp_lib_path.exists() {
//         panic!(
// "
// Error while compiling russimp-sys:

// assimp.lib was not found (specifically at {}). The assimp build may have failed or made libraries in a different location. 
// Please check the CMake output and ensure all deps are installed. 
// ",
//             assimp_lib_path.display()
//         );
//     }

    println!(
        "cargo:rustc-link-search=native={}",
        cmake_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        cmake_dir.join("bin").display()
    );
}

/// This function ensures the assimp library is downloaded for building from source, which caused the static-link feature to break
/// 
/// Reference: https://github.com/jkvargas/russimp-sys/issues/49
fn ensure_submodules()
 -> Result<PathBuf, Box<dyn std::error::Error>> 
 {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let assimp_dir = out_dir.join("assimp");
    let assimp_cmake = assimp_dir.join("CMakeLists.txt");
    println!("cargo:warning=out_dir: {:?}", &out_dir);

    if !assimp_cmake.exists() {
        // clone repo
        println!("cargo:warning=assimp aint found, cloning...");

        if assimp_dir.exists() {
            std::fs::remove_dir_all(&assimp_dir)?;
        }

        // let zip_url = "https://github.com/assimp/assimp/archive/refs/heads/master.zip";
        let zip_url = "https://github.com/assimp/assimp/archive/6a08c39e3a91ef385e76515cfad86aca4bfd57ff.zip";
        let zip_path = out_dir.join("assimp.zip");

        println!("cargo:warning=downloading from github");
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        let response = client.get(zip_url).send()?;
        let bytes = response.bytes()?;
        std::fs::write(&zip_path, &bytes)?;

        println!("cargo:warning=extracting zip file contents");
        // inflate zip
        let file = File::open(&zip_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => out_dir.join(path),
                None => continue,
            };
            
            if (*file.name()).ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        let extracted_dir = out_dir.join("assimp-6a08c39e3a91ef385e76515cfad86aca4bfd57ff");
        if extracted_dir.exists() {
            std::fs::rename(&extracted_dir, &assimp_dir)?;
        }

        let _ = std::fs::remove_file(&zip_path);

        if !assimp_cmake.exists() {
            return Err("CMakeLists.txt not found after extracting assimp".into());
        }

        println!("cargo:warning=cloning went well, happy :)");
    }

    Ok(assimp_dir)
}

fn link_from_package() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target = env::var("TARGET").unwrap();
    let crate_version = env::var("CARGO_PKG_VERSION").unwrap();
    let archive_name = format!(
        "russimp-{}-{}-{}.tar.gz",
        crate_version,
        target,
        static_lib()
    );

    let ar_src_dir;

    if option_env!("RUSSIMP_PACKAGE_DIR").is_some() {
        ar_src_dir = PathBuf::from(env::var("RUSSIMP_PACKAGE_DIR").unwrap());
    } else {
        ar_src_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let dl_link = format!(
            "https://github.com/jkvargas/russimp-sys/releases/download/v{}/{}",
            crate_version, archive_name
        );

        match fs::File::open(ar_src_dir.join(&archive_name)) {
            Ok(_) => {}
            Err(_) => {
                let resp = reqwest::blocking::get(dl_link).unwrap();
                let mut bytes = io::Cursor::new(resp.bytes().unwrap());

                let mut file = fs::File::create(ar_src_dir.join(&archive_name)).unwrap();
                io::copy(&mut bytes, &mut file).unwrap();
            }
        }
    }

    dbg!(ar_src_dir.join(&archive_name));

    let file = fs::File::open(ar_src_dir.join(&archive_name)).unwrap();
    let mut archive = tar::Archive::new(GzDecoder::new(file));
    let ar_dest_dir = out_dir.join(static_lib());

    archive.unpack(&ar_dest_dir).unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        ar_dest_dir.join("lib").display()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        ar_dest_dir.join("bin").display()
    );
}

fn show_error_mgs(e: Box<dyn std::error::Error>) {
    panic!(
"
Failed to fetch the assimp source, specifically \"https://github.com/assimp/assimp/tree/6a08c39e3a91ef385e76515cfad86aca4bfd57ff\". 

This create requires the assimp git repository source in order to work (duh), so here are your options.
Either:
1. Use the 'prebuilt' feature. This will use a prebuilt dll from the russimp-sys repository releases. 
2. Download assimp system-wide and don't use any features (as the build script will fetch for you).
3. Make sure your network works because your wifi might be borked. 

=====================================================================================================================================
Specific error message: {}
=====================================================================================================================================

Sorry :(
", e)
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Look for assimp lib in Brew install paths on MacOS.
    // See https://stackoverflow.com/questions/70497361/homebrew-mac-m1-cant-find-installs
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib/");

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    println!("cargo:rustc-link-search=native=/opt/brew/lib/");

    if build_assimp() {
        build_from_source();
    } else if cfg!(feature = "prebuilt") {
        link_from_package();
    }

    let assimp_include_path = out_dir.join("assimp").join("include").join("assimp");
    match ensure_submodules() {
        Ok(_) => {},
        Err(e) => {show_error_mgs(e); panic!("Extra panic! Ahhh!")},
    };

    // assimp/defs.h requires config.h to be present, which is generated at build time when building
    // from the source code (which is disabled by default).
    // In this case, place an empty config.h file in the include directory to avoid compilation errors.
    println!("{:?}", assimp_include_path);    
    let config_file = assimp_include_path.join("config.h");
    let config_exists = config_file.clone().exists();
    if !config_exists {
        fs::write(&config_file, "")
        // fix up this error message
        .expect(
            r#"Unable to write config.h to assimp/include/assimp/,
            make sure you cloned submodules with "git submodule update --init --recursive""#,
        );
    }

    bindgen::builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", out_dir.join(static_lib()).join("include").display()))
        .clang_arg(format!("-I{}", out_dir.join("assimp").join("include").display()))
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

    if !config_exists {
        // Clean up config.h
        let _ = fs::remove_file(config_file);
    }

    let mut built_opts = built::Options::default();
    built_opts
        .set_dependencies(false)
        .set_compiler(false)
        .set_ci(false)
        .set_cfg(false);

    built::write_built_file_with_opts(&built_opts, &manifest_dir, &out_dir.join("built.rs"))
        .unwrap();

    for n in lib_names().iter() {
        println!("cargo:rustc-link-lib={}={}", n.1, n.0);
    }
}
