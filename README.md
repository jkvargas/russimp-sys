# russimp-sys ![russimp-sys](https://github.com/jkvargas/russimp-sys/workflows/russimp-sys/badge.svg?branch=main) [![Crates.io](https://img.shields.io/crates/v/russimp-sys.svg)](https://crates.io/crates/russimp-sys)

Unsafe Rust bindings for the Open Asset Import Library (assimp).  
See: [Our safe assimp Rust library](https://github.com/jkvargas/russimp)

Raw bindings for the C API of assimp.

## Platform Support
We build, test, and provide prebuilt packages for the following targets:
- x86_64-pc-windows-msvc
- x86_64-apple-darwin
- x86_64-unknown-linux-gnu

Additional targets that work when building from source:
- aarch64-apple-darwin (M1 Macs, cross-compiled on x86_64.)
- aarch64-unknown-linux-gnu (Raspberry Pi 4b, built on the machine itself.)

Platforms that are not supported and won't build:
- x86_64-pc-windows-gnu (See: [assimp/4686]([https://github.com/assimp/assimp/issues/4868))

## Installation

**By default** `russimp-sys` is looking for the `assimp` library in the system.  
However there are many ways for the crate to install the library for you by specifying these crate features:

### `prebuilt`
This features will download a prebuilt package from this repo's release page, these packages are built and published automatically every time we release a new version. 

In addition, you can specify a local package by setting the `RUSSIMP_PACKAGE_DIR` environment variable to the path of the package.
You can run the provided package binary to generate a package for your platform.

```cargo run --bin package --features <INSERT-LINK-TYPE>```

### `build-assimp` or `static-link`
The `build-assimp` feature will build the library from source and link it dynamically.  
The `static-link` feature will build the library from source and link it statically.

Building from source requires the following dependencies:
- CMake
- libclang (for `bindgen`)
- A C/C++ compiler
- RECOMMENDED: Ninja (For Windows users the buildscript automatically uses Ninja if it finds it in the PATH)

### Additional Features:

### `nozlib`

By default `russimp-sys` will statically link `zlibstatic`. Enabling this feature will link to the system's `zlib` library.

## Changelog
### 2.0.0
* Complete overhaul of the build script.
* Expose all assimp headers.
* Rework CI pipeline.
* Support for local assimp packaging and local package usage. (See: `prebuilt` feature)
* Remove vcpkg support.
* Remove `nolibcxx` feature.
### 1.0.3
* Builds based on 5.2.5 release
### 1.0.0
