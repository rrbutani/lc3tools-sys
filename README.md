## lc3tools-sys

[![Build Status][ci]][actions] [![License: Apache 2.0][license-badge]][license] [![crates.io][crates-badge]][crates] [![API Docs][docs-badge]][docs]

Rust bindings for [`LC3Tools`][lc3tools].

## Usage

Add this to your `Cargo.toml`:
```TOML
[dependencies]
lc3tools-sys = "1.0.6-alpha0"
```

Headers are exposed at the path the `DEP_LC3CORE_LINKS` env var points to.

Since the bindings this crate exposes are exactly one to one with the `LC3Tools` API, the `LC3Tools` source code and documentation are the best place to go for information about how to use this crate, especially the [API documentation][api-docs]

[api-docs]: https://github.com/chiragsakhuja/lc3tools/blob/master/docs/API.md

## Features

### `LC3Tools` functionality features
The [backend part][backend] of `LC3Tools` is always included. The [`frontend` feature][frontend-feat] includes the files in [`frontend/common`][frontend]. The [`grader` feature][grader-feat] (which requires the `frontend` feature) includes the files in [`frontend/grader`][grader] but strips out the [`main` function in `framework.cpp`](https://github.com/chiragsakhuja/lc3tools/blob/433a4c224f3a70bee532d12a7b1cb227ba71dd77/frontend/grader/framework.cpp#L76-L207).

These features are both [enabled by default][default].

[backend]: https://github.com/chiragsakhuja/lc3tools/tree/master/backend
[frontend]: https://github.com/chiragsakhuja/lc3tools/tree/master/frontend/common
[grader]: https://github.com/chiragsakhuja/lc3tools/tree/master/frontend/grader

[frontend-feat]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L60
[grader-feat]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L59
[default]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L58

### Other features

#### `generate-fresh`

By default, Rust bindings for `LC3Tools` aren't [generated][bindgen] anew when building this crate. Instead we bundle [pre-generated bindings][bindings] and use them, by default. We do this because generating the bindings is a little time consuming (takes about a minute — unless you care deeply about how long clean builds take, this is a non-issue), but more importantly because generating the bindings is somewhat system-specific. [`bindgen`][bindgen] walks through the system libc and C++ standard library headers as part of doing so and we maintain [a list of types and things for it to skip][skip] that's very libc/system/OS specific.

You'll probably never need to, but if you find yourself wanting to generate these bindings yourself (i.e. because you modified some headers in `LC3Tools`), then you can build with the [`generate-fresh` feature][generate-fresh-feat] (`build.rs` goes and passes the right instructions to `cargo` so you can just leave the feature enabled — it'll only actually do the work when one of the headers/files in the build graph change).

[bindgen]: https://github.com/rust-lang/rust-bindgen
[bindings]: https://github.com/rrbutani/lc3tools-sys/tree/main/generated/bindings.rs

[skip]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/build.rs#L122-L141
[generate-fresh-feat]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L62

#### `lto`

Unfortunately, we can't enable LTO by default as it requires some setup and somewhat specific tools to be used (see [this commit][c1] and [this commit][c2] for some context and [this page][lto] for details about what setup is needed).

So, we offer an [`lto` feature][lto-feat] that passes the compiler [`cc`][cc] invokes [the necessary flag][lto-flag]. When using this feature you'll also need to make sure that you pass `rustc` the LTO linker plugin flag and instruct it to use an appropriate linker, as described [here][lto-setup]. For this specific crate the necessary flags exist [here][cargo-config-lto], but commented out.

The final thing you need to do when using the `lto` feature is to make sure that the compiler that `cc` ends up using will work with the LTO linker plugin. The table [here][lto-setup] offers some information on what version should work, but it's somewhat out of date; if the same version of `clang` is used by `cc` to build `LC3Tools` and by `rustc` to do the linking, things should work (provided it's a relatively modern version of clang — CI in this repo uses version 9, successfully).

Actually figuring out and changing which compiler [`cc`][cc] uses is tricker; on Linux based systems ensuring that the `c++` alias points to the desired version seems to do the trick (`update-alternatives` might be able to help you with this).

[c1]: https://github.com/rrbutani/lc3tools-sys/commit/3cf85f4afbf35ffa711dc4a4eaa401ab74ff95c3
[c2]: https://github.com/rrbutani/lc3tools-sys/commit/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687

[lto]: https://doc.rust-lang.org/rustc/linker-plugin-lto.html#toolchain-compatibility
[lto-setup]: https://doc.rust-lang.org/rustc/linker-plugin-lto.html#cc-code-as-a-dependency-in-rust

[cc]: https://github.com/alexcrichton/cc-rs

[lto-feat]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L63
[lto-flag]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/build.rs#L499
[cargo-config-lto]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/.cargo/config#L4-L5

## Examples

Right now we have [one example][mul] that runs an LC-3 program that multiplies two unsigned numbers.

`cargo run --example mul` _should_ run it.

[mul]: https://github.com/rrbutani/lc3tools-sys/tree/main/examples/mul.rs

## Minimum Supported Rust Version (MSRV)

This crate is currently guaranteed to compile on stable Rust 1.43 and newer. We offer no guarantees that this will remain true in future releases but do promise to always support (at minimum) the latest stable Rust version and to document changes to the MSRV in the [changelog][changelog].

## Contributing

PRs are (very) welcome! See [CONTRIBUTING.md] for details.

[ci]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Frrbutani%2Flc3tools-sys%2Fbadge%3Fref%3Dmain&style=for-the-badge&labelColor=505050&color=A0CB8D
[license-badge]: https://img.shields.io/github/license/rrbutani/lc3tools-sys?style=for-the-badge&logo=GNU&labelColor=505050&color=998DCB
[crates-badge]: https://img.shields.io/crates/v/lc3tools-sys?style=for-the-badge&logo=rust&labelColor=505050&color=CB8DA0
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&labelColor=505050&color=8DBFCB

[changelog]: https://github.com/rrbutani/lc3tools-sys/tree/main/CHANGELOG.md

[CONTRIBUTING.md]: https://github.com/rrbutani/lc3tools-sys/tree/main/.github/CONTRIBUTING.md

[actions]: https://github.com/rrbutani/lc3tools-sys/actions
[license]: https://opensource.org/licenses/Apache-2.0
[crates]: https://crates.io/crates/lc3tools-sys
[docs]: https://rrbutani.github.io/lc3tools-sys/docs/lc3tools_sys

[lc3tools]: https://github.com/chiragsakhuja/lc3tools
