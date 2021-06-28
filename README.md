# russimp-sys ![russimp-sys](https://github.com/jkvargas/russimp-sys/workflows/russimp-sys/badge.svg?branch=main) [![Crates.io](https://img.shields.io/crates/v/russimp-sys.svg)](https://crates.io/crates/russimp-sys)

Assimp raw bindings for Rust. 
There is a high chance that you are actually looking for russimp https://github.com/jkvargas/russimp

In order to use this you will need to have assimp installed on your system.
If you are an ubuntu user I believe you will need to install libassimp-dev.
Ubuntu users, please, correct me if I am wrong.

Pipeline is now building for windows as well, it is using vcpkg to get assimp.
Please if youre a windows user, let me know if this is working on your side.

This package uses [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg) to manage system dependencies (particularly on 
windows). Running ```cargo vcpkg build``` will build the necessary dependencies within the target directory. 
Alternatively provide a VCPKG_ROOT environment variable pointed at the location of a shared vcpkg installation.
