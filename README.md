## lc3tools-sys

[![Build Status][ci]][actions] [![License: Apache 2.0][license-badge]][license] [![crates.io][crates-badge]][crates] [![API Docs][docs-badge]][docs]

Rust bindings for [`LC3Tools`][lc3tools].

## Usage

Add this to your `Cargo.toml`:
```TOML
[dependencies]
lc3tools-sys = "1.0.6"
```

Since the bindings this crate exposes are exactly one to one with the `LC3Tools` API, the `LC3Tools` source code and documentation are the best place to go for information about how to use this crate, especially the [API documentation][api-docs].

### Headers

Headers are exposed at the path the `DEP_LC3CORE_INCLUDE` env var points to.

### Caveats

However, note that `LC3Tools` exposes a C++ API. Though bindings for it are provided in this crate, it's extremely unlikely they will work with your OS/platform/compiler/compiler flags. Different platforms [seem to have different name-mangling conventions](https://github.com/google/bloaty/issues/43#issuecomment-288270723) and layout isn't (afaik) stable across different configurations (switching from `-O3` to `-O0` breaks the [C++ interface example][cpp-interface-ex] with my configuration, for example).

All of this (mangled symbols, layout information) is encoded in the [generated bindings][bindings]. Note that [because linkers are often lazy](https://kornel.ski/rust-sys-crate#linking), even though the symbols in the generated bindings don't match those that are actually produced on your platform, you may not get a compile error unless/until you actually go to _use_ (transitively) those symbols in your final binary. This is why the [C++ interface example][cpp-interface-ex] is [feature gated][cpp-interface-ex-feature-gate].

We offer a [`generate-fresh`](#generate-fresh) feature so that you can generate this file locally at build time, but it still remains unlikely that the C++ interface will work/be of use. Things like vtables are represented by opaque types and even if you [manage to get a hold of a C++ generated vtable to pass along][vtable] sometimes things still don't work.

For example, for reasons still unknown, when running the [C++ interface example][cpp-interface-ex], the `printer` [that's passed to the simulator][mystery] mysteriously turns into a `NULL` but _only_ in the copy of the logger that's given to the `state` instance; the copy that's in the `logger` remains unchanged. I was only able to get the example to work after making the following changes to `LC3Tools`:

<details>
<summary>Click to show the diff.</summary>

```diff
diff --git a/backend/logger.h b/backend/logger.h
index b7146ac..c172acb 100644
--- a/backend/logger.h
+++ b/backend/logger.h
@@ -28,10 +28,17 @@ namespace utils
       template<typename ... Args>
       void printf(PrintType level, bool bold, std::string const & format, Args ... args) const;
       void newline(PrintType level = PrintType::P_ERROR) const {
-            if(static_cast<uint32_t>(level) <= print_level) { printer.newline(); }
       }
       void print(std::string const & str) {
-            if(print_level > static_cast<uint32_t>(PrintType::P_NONE)) { printer.print(str); }
       }
       uint32_t getPrintLevel(void) const { return print_level; }
       void setPrintLevel(uint32_t print_level) { this->print_level = print_level; }
diff --git a/backend/simulator.cpp b/backend/simulator.cpp
index c8004e8..bd7f1db 100644
--- a/backend/simulator.cpp
+++ b/backend/simulator.cpp
@@ -112,7 +112,7 @@ void Simulator::simulate(void)
       collecting_input = true;
-        inputter.beginInput();
       if(threaded_input) {
           input_thread = std::thread(&core::Simulator::inputThread, this);
       }
@@ -125,7 +125,7 @@ void Simulator::simulate(void)
           executeEventChain(events);
           updateDevices();
           if(! threaded_input) {
-                collectInput();
           }
           checkAndSetupInterrupts();
       }
@@ -139,7 +139,7 @@ void Simulator::simulate(void)
   if(threaded_input && input_thread.joinable()) {
       input_thread.join();
   }
-    inputter.endInput();
```
</details>

### Workarounds

To make this crate at least somewhat usable, we offer a [limited set of C bindings][c-bindings-header] that are only really good for running whole programs.

This is incredibly clunky but it was good enough™ for our use case. If actual Rust bindings for `LC3Tools` are a thing you need, [`cxx`](https://github.com/dtolnay/cxx) is probably worth looking into. Since this crate [exports the `LC3Tools` headers](#headers) you could depend on this crate and use it for it's `cc` setup (ignoring the bindings it has).

Alternatively, if there are specific additions to the C bindings you need, PRs are very welcome!

[api-docs]: https://github.com/chiragsakhuja/lc3tools/blob/master/docs/API.md

[cpp-interface-ex]: https://github.com/rrbutani/lc3tools-sys/blob/e2e6f72106b577be7a90a380540bd5cbb1e0f7a8/examples/mul.rs#L133-L180
[c-interface-ex]: https://github.com/rrbutani/lc3tools-sys/blob/e2e6f72106b577be7a90a380540bd5cbb1e0f7a8/examples/mul.rs#L85-L131

[cpp-interface-ex-feature-gate]: https://github.com/rrbutani/lc3tools-sys/blob/fac13ea6e385be076d7c12bd693bfdde1dc1d610/examples/mul.rs#L81

[mystery]: https://github.com/chiragsakhuja/lc3tools/blob/433a4c224f3a70bee532d12a7b1cb227ba71dd77/backend/simulator.cpp#L25

[vtable]: https://github.com/rrbutani/lc3tools-sys/blob/e2e6f72106b577be7a90a380540bd5cbb1e0f7a8/examples/mul.rs#L142-L143

[c-bindings-header]: https://github.com/rrbutani/lc3tools-sys/tree/main/extra/bindings.h

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

[skip]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/build.rs#L122-L141
[generate-fresh-feat]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L62

#### `lto`

Unfortunately, we can't enable LTO by default as it requires some setup and somewhat specific tools to be used (see [this commit][c1] and [this commit][c2] for some context and [this page][lto] for details about what setup is needed).

So, we offer an [`lto` feature][lto-feat] that passes the compiler [`cc`][cc] invokes [the necessary flag][lto-flag]. When using this feature you'll also need to make sure that you pass `rustc` the LTO linker plugin flag and instruct it to use an appropriate linker, as described [here][lto-setup]. For this specific crate the necessary flags exist [here][cargo-config-lto], but commented out.

The final thing you need to do when using the `lto` feature is to make sure that the compiler that `cc` ends up using will work with the LTO linker plugin. The table [here][lto-setup] offers some information on what version should work, but it's somewhat out of date; if the same version of `clang` is used by `cc` to build `LC3Tools` and by `rustc` to do the linking, things should work (provided it's a relatively modern version of clang — CI in this repo uses version 11, successfully).

Actually figuring out and changing which compiler [`cc`][cc] uses is tricker; on Linux based systems ensuring that the `c++` alias points to the desired version seems to do the trick (`update-alternatives` might be able to help you with this). Or just set `CC` to `clang` and `CXX` to `clang++`.

macOS has slightly different flags; our [CI configuration][lto-ci] has details and an example.

[lto-ci]: https://github.com/rrbutani/lc3tools-sys/blob/423a8b3c5a02373bfad5db2b57e5c67e1ca5a0ec/.github/workflows/full.yml#L208-L237

[c1]: https://github.com/rrbutani/lc3tools-sys/commit/3cf85f4afbf35ffa711dc4a4eaa401ab74ff95c3
[c2]: https://github.com/rrbutani/lc3tools-sys/commit/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687

[lto]: https://doc.rust-lang.org/rustc/linker-plugin-lto.html#toolchain-compatibility
[lto-setup]: https://doc.rust-lang.org/rustc/linker-plugin-lto.html#cc-code-as-a-dependency-in-rust

[cc]: https://github.com/alexcrichton/cc-rs

[lto-feat]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/Cargo.toml#L63
[lto-flag]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/build.rs#L499
[cargo-config-lto]: https://github.com/rrbutani/lc3tools-sys/blob/c8139dc1a6af4f55e3e1b55ed8f68473c7e74687/.cargo/config#L4-L5

## Examples

Right now we have [one example][mul] that runs an LC-3 program that multiplies two unsigned numbers. As mentioned, it has a [C++ interface part][cpp-interface-ex] and a [C interface part][c-interface-ex]. By default the C++ part is [disabled][cpp-interface-ex-feature-gate] as it's [unlikely it will work on your machine](#caveats).

`cargo run --example mul` _should_ run the C interface part.

[mul]: https://github.com/rrbutani/lc3tools-sys/tree/main/examples/mul.rs

## Minimum Supported Rust Version (MSRV)

This crate is currently guaranteed to compile on stable Rust 1.43 and newer. We offer no guarantees that this will remain true in future releases but do promise to always support (at minimum) the latest stable Rust version and to document changes to the MSRV in the [changelog][changelog].

## Contributing

PRs are (very) welcome! See [CONTRIBUTING.md] for details.

[ci]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Frrbutani%2Flc3tools-sys%2Fbadge%3Fref%3Dmain&style=for-the-badge&labelColor=505050&color=90BB7D
[license-badge]: https://img.shields.io/github/license/rrbutani/lc3tools-sys?style=for-the-badge&logo=GNU&labelColor=505050&color=998DCB
[crates-badge]: https://img.shields.io/crates/v/lc3tools-sys?style=for-the-badge&logo=rust&labelColor=505050&color=CB8DA0
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&labelColor=505050&color=7DAFBB

[changelog]: https://github.com/rrbutani/lc3tools-sys/tree/main/CHANGELOG.md

[CONTRIBUTING.md]: https://github.com/rrbutani/lc3tools-sys/tree/main/.github/CONTRIBUTING.md

[actions]: https://github.com/rrbutani/lc3tools-sys/actions
[license]: https://opensource.org/licenses/Apache-2.0
[crates]: https://crates.io/crates/lc3tools-sys
[docs]: https://rrbutani.github.io/lc3tools-sys/docs/lc3tools_sys

[lc3tools]: https://github.com/chiragsakhuja/lc3tools

[bindings]: https://github.com/rrbutani/lc3tools-sys/tree/main/generated/bindings.rs
