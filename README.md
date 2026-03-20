# Pelican

A Rust UI framework built on SDL2.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg) — install with:
  ```sh
  cargo install cargo-vcpkg
  ```

## Setup

Build the native dependencies (SDL2, SDL2_image, SDL2_ttf, SDL2_gfx, SDL2_mixer) via vcpkg:

```sh
cargo vcpkg build
```

This clones vcpkg and compiles the dependencies locally — no system-level package manager required.

## Build

```sh
cargo build
```

## Run examples

```sh
cargo run --example graphics
cargo run --example ui
cargo run --example label
```
