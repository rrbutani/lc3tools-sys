use std::env;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, File};
use std::io::{BufReader, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use bindgen::{builder, Builder};
use cc::Build;
use pretty_assertions::assert_eq as eq;
use serde_json::Value;

macro_rules! env {
    ($var:literal) => {
        std::env::var($var).unwrap()
    };
}

fn in_dir_with_ext<'s, D>(
    dir: D,
    ext: &'s str,
) -> Result<impl Iterator<Item = DirEntry> + 's>
where
    D: AsRef<OsStr>,
{
    Ok(fs::read_dir(Path::new(&dir))?
        .filter_map(|d| d.ok())
        .filter(|d| d.file_type().unwrap().is_file())
        .filter(move |de| {
            de.path().extension().unwrap().to_str().unwrap() == ext
        }))
}

fn main() -> Result<()> {
    // For path/git deps (when grabbing from crates.io lc3tools will be rolled
    // into the package).
    if !Path::new("lc3tools/.git").exists() {
        let exit_code = Command::new("git")
            .args(&["submodule", "update", "--init"])
            .status()?;

        assert!(exit_code.success(), "Failed to clone `lc3tools`");
    }

    // Next we make sure that the lc3tools source we have matches this crates'
    // version.
    //
    // (This isn't perfect; ideally we'd actually check that we've got a
    // tag of the submodule whose name matches the version we're looking for but
    // we don't keep `.git` around when we publish to `crates.io` so this will
    // have to do.)
    let crate_version = env!("CARGO_PKG_VERSION");

    let lc3tools_package_json = Path::new("lc3tools/frontend/gui/package.json");
    let lc3tools_package_json = File::open(lc3tools_package_json)
        .expect("`package.json` in lc3tools/frontend/gui");
    let reader = BufReader::new(lc3tools_package_json);
    let package_json = serde_json::from_reader::<_, Value>(reader)
        .expect("A valid `package.json`");
    let lc3tools_version = &package_json["version"];

    eq!(crate_version, lc3tools_version.as_str().unwrap());

    // First, lets gather and copy over the header files.
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let include = out.join("include");
    fs::create_dir_all(&include)?;
    let mut builder: Builder = builder();

    println!("cargo:rerun-if-changed={}", "lc3tools/backend");
    for header in in_dir_with_ext("lc3tools/backend", "h")
        .expect("Header files in lc3tools/backend")
    {
        // Tell cargo to invalidate if the file changes.

        let path = header.path();
        // This file is not used and is broken.
        if path.file_name().unwrap().to_str().unwrap() == "device.h" {
            continue;
        }

        builder = builder
            .header::<String>(path.to_str().unwrap().into())
            .parse_callbacks(Box::new(bindgen::CargoCallbacks));

        let to = include.join(path.file_name().unwrap());
        fs::copy(&path, &to).expect("Header file copy to succeed");
    }

    // TODO: is `canonicalize` actually broken? (rust#42869)
    println!("cargo:include={}", include.canonicalize()?.display());

    // Next let's go run bindgen:
    #[rustfmt::skip]
    builder
        .enable_cxx_namespaces()
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")

        .derive_debug(true)
        .generate_comments(true)

        .blacklist_item("std::value")
        .blacklist_item("__gnu_cxx::__max")
        .blacklist_item("__gnu_cxx::__min")

        .blacklist_item("std::collate_string_type")
        .blacklist_item("std::collate_byname_string_type")
        .blacklist_item("std::numpunct_string_type")
        .blacklist_item("std::numpunct_byname_string_type")
        .blacklist_item("size_type")
        .blacklist_item("std::size_type")
        .blacklist_item("int_type")
        .blacklist_item("char_type")
        .blacklist_item(".*__.*t")
        .blacklist_item("std::basic_ostream_sentry")
        .blacklist_item("std::basic_istream_sentry___istream_type")
        .blacklist_item("std::basic_istream_sentry_traits_type")
        .blacklist_item("std::basic_istream_sentry___streambuf_type")

        .generate()
        .expect("Unable to generate bindings!")
        .write_to_file(out.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Finally let's go gather the C++ files and do the build.
    let mut build = Build::new();

    // `cc` automatically handles `OPT_LEVEL` and `DEBUG`.
    // let (opt_level, debug) = (env!("OPT_LEVEL"), env!("DEBUG"));
    // let opt_level = match opt_level {
    //     "z" | "s" | "0" | "1" | "2" => opt_level,
    //     "3" => "2",
    //     _ => panic!("Invalid opt level: {}", opt_level)
    // };
    // let debug = match debug {
    //     "0" | "false" => false,
    //     "1" | "2" | "true" => true,
    //     _ => panic!("Invalid debug setting: {}", debug)
    // };

    // `cc` also handles `fPIC`

    build
        .include("lc3tools/backend")
        .flag_if_supported("-flto")
        .flag_if_supported("-Wno-format-security")
        .cargo_metadata(true)
        .warnings(true)
        .extra_warnings(true)
        .cpp(true);

    if env!("PROFILE") == "debug" {
        build.define("_ENABLE_DEBUG", None);
    }

    for source_file in in_dir_with_ext("lc3tools/backend", "cpp")
        .expect("Source files in lc3tools/backend")
    {
        println!("cargo:rerun-if-changed={}", source_file.path().display());
        build.file(source_file.path());
    }

    build.out_dir(out.join("build")).compile("lc3core");

    println!("cargo:root={}", out.display());

    Ok(())
}
