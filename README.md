# russimp-sys ![russimp-sys](https://github.com/jkvargas/russimp-sys/workflows/russimp-sys/badge.svg?branch=main) [![Crates.io](https://img.shields.io/crates/v/russimp-sys.svg)](https://crates.io/crates/russimp-sys)

Assimp raw bindings for Rust.

There is a high chance that you are actually looking for russimp https://github.com/jkvargas/russimp

## How to use

**By default**, you will need to have `assimp` installed on your system, if you are an ubuntu user I believe you will need to install `libassimp-dev`.

`russimp-sys` will look for `assimp` library and headers from your system, generates rust bindings and [dynamic linking](<https://en.wikipedia.org/wiki/Library_(computing)#Dynamic_linking>) to `assimp` shared library.

**For Windows**, This package uses [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg) to manage system dependencies. Running ```cargo vcpkg build``` will build the necessary dependencies within the target directory. Alternatively provide a VCPKG_ROOT environment variable pointed at the location of a shared vcpkg installation.

**For who want a standalone executable file**. Enable [`prebuilt`](#prebuilt) feature, then `russimp-sys` will download the prebuilt `assimp` [static library](<https://en.wikipedia.org/wiki/Library_(computing)#Static_libraries>) and its dependencies from github, linking libraries to your executable, but it will increase the size of the executable.

## Features

You can use the following [FEATURES](https://doc.rust-lang.org/cargo/reference/features.html#the-features-section) to configure the behavior of `russimp-sys`.

### `prebuilt`

Download prebuilt `Assimp` static library binaries from github and skip building from source.

Because `Assimp` build is slow and have build environment requirements. We provide prebuilt binaries for common platforms and features.

When a new version is released, github action automatically runs pre-build tasks, and all prebuilt binaries are saved in [github releases](https://github.com/jkvargas/russimp-sys/releases).

The `russimp-sys` build script will try to download the prebuilt binaries from github first, and skip the full source build.

### `static-link`

Enabling `static-link` feature without `prebuilt` feature, will build `assimp` from source.

Build from source need the following dependencies:

* cmake
* clang
* Ninja for Linux and MacOS, Visual Studio 2019 for Windows

### `nozlib`

By default `russimp-sys` will statically link zlibstatic, you can disable this feature if it conflicts with other dependencies.

### `nolibcxx`

By default `russimp-sys` links to `libstdc++` in linux and `libc++` in macos, turning this on `russimp-sys` won't link to the c++ standard library.

## Changelog

### 1.0.0

- Builds based on 5.1.0 release
