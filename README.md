# russimp-sys ![russimp-sys](https://github.com/jkvargas/russimp-sys/workflows/russimp-sys/badge.svg?branch=main) [![Crates.io](https://img.shields.io/crates/v/russimp-sys.svg)](https://crates.io/crates/russimp-sys)

Assimp raw bindings for Rust.

There is a high chance that you are actually looking for russimp https://github.com/jkvargas/russimp

Assimp just released v5.1.0 which is used for the linux build.

Vcpkg only has assimp 5.0.1 only, it might take some time for them to update it.

If you want to help maintaining this package on windows or macos, please let me know.

In order to use this you will need to have assimp installed on your system.
If you are an ubuntu user I believe you will need to install libassimp-dev.

This package uses [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg) to manage system dependencies (particularly on 
windows). Running ```cargo vcpkg build``` will build the necessary dependencies within the target directory. 
Alternatively provide a VCPKG_ROOT environment variable pointed at the location of a shared vcpkg installation.

## Changelog

### 1.0.0
* Builds based on 5.1.0 release
