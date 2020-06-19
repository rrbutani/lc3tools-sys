use std::env;
use std::ffi::OsStr;
use std::fs::{self, DirEntry, File};
use std::io::{BufReader, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(feature = "generate-fresh")]
use bindgen::{builder, Builder};
use cc::Build;
use pretty_assertions::assert_eq as eq;
use serde_json::Value;

macro_rules! env {
    ($var:literal) => {
        std::env::var($var).unwrap()
    };
}

const BACKEND: &'static str = "lc3tools/backend";
const FRONTEND: &'static str = "lc3tools/frontend/common";
const GRADER: &'static str = "lc3tools/frontend/grader";

fn in_dir_with_ext<'s, D>(
    dir: &D,
    ext: &'s str,
) -> Result<impl Iterator<Item = DirEntry> + 's>
where
    D: AsRef<OsStr> + ?Sized,
{
    Ok(fs::read_dir(Path::new(&dir))?
        .filter_map(|d| d.ok())
        .filter(|d| d.file_type().unwrap().is_file())
        .filter(|d|
            // This file is not used and is broken.
            d.path().file_name().unwrap().to_str().unwrap() != "device.h"
        )
        .filter(move |de| {
            de.path().extension().unwrap().to_str().unwrap() == ext
        }))
}

fn copy_headers<I>(
    inc_dir: &I,
    cpy_dir: &Path,
) -> Result<()>
where
    I: AsRef<OsStr> + ?Sized,
{
    fs::create_dir_all(&cpy_dir)?;

    let inc_dir_str = inc_dir.as_ref().to_str().unwrap();
    println!("cargo:rerun-if-changed={}", inc_dir_str);

    for header in in_dir_with_ext(inc_dir, "h")
        .expect(format!("Header files in `{}`", inc_dir_str).as_str())
    {
        let path = header.path();
        let to = cpy_dir.join(path.file_name().unwrap());
        fs::copy(&path, &to).expect("Header file copy to succeed");
    }

    Ok(())
}

// This is kind of a duplicate of `rustfmt` functions in the root of the
// `bindgen` crate except we just run `rustfmt` straight on the generated files
// rather than messing with pipes and threads. We can can do this since our use
// case is much narrower.
//
// We also pretty much just assume `rustfmt` is in the PATH or in an env var
// and don't try to search for it (`bindgen` uses `which::which` when the
// `which-rustfmt` feature is enabled).
#[cfg(feature = "generate-fresh")]
fn run_rustfmt<F>(
    files: impl IntoIterator<Item = F>,
) -> Result<()>
where
    F: AsRef<OsStr> + ?Sized
{
    let rustfmt = if let Ok(rustfmt) = env::var("RUSTFMT") {
        rustfmt
    } else {
        String::from("rustfmt")
    };

    let success = Command::new(rustfmt)
        .args(files)
        .status()?
        .success();

    assert!(success, "`rustfmt` failed.");
}

#[cfg(feature = "generate-fresh")]
fn make_bindings<I>(
    inc_dirs: &[&I],
) -> std::result::Result<syn::File, Box<dyn std::error::Error>>
where
    I: AsRef<OsStr> + ?Sized,
{
    let mut builder: Builder = builder();

    for dir in inc_dirs {
        for header in in_dir_with_ext(dir, "h")
            .expect(format!("Header files in `{}`", inc_dir_str).as_str())
        {
            builder = builder
                .header::<String>(path.to_str().unwrap().into())
                .parse_callbacks(Box::new(bindgen::CargoCallbacks));
        }
    }

    #[rustfmt::skip]
    let res = builder
        .enable_cxx_namespaces()
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-Ilc3tools/backend")

        .derive_debug(true)
        .derive_default(true)
        .generate_comments(true)
        .rustfmt_bindings(false) // We'll run this ourselves after processing.

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
        .blacklist_item("__atomic_val_t")
        .blacklist_item("__atomic_diff_t")
        .blacklist_item("std::__atomic_val_t")
        .blacklist_item("std::__atomic_diff_t")
        .blacklist_item("std::basic_ostream_sentry")
        .blacklist_item("std::basic_istream_sentry___istream_type")
        .blacklist_item("std::basic_istream_sentry_traits_type")
        .blacklist_item("std::basic_istream_sentry___streambuf_type")

        .generate()
        .expect("Unable to generate bindings!")
        .to_string();

    // `bindgen` actually has a `proc_macro2::TokenStream` internally that it
    // then turns into a String but since we've got no way to actually access
    // that TokenStream we've gotta do this silly thing where we turn things
    // into a String and then back into a TokenStream and then do the parse.
    //
    // This isn't great but since we don't expect users to run this, it should
    // be okay.
    let parsed: syn::File = syn::parse_str(res)?;

    Ok(parsed)
}

#[cfg(feature = "generate-fresh")]
pub mod binding_support {
    use std::collections::HashMap;
    use std::io::Write;

    use syn::{
        Attribute, File, Item, Ident, PathArguments, PathSegment,
        punctuated::Punctuated,
        token::Colon2,
        visit::Visit,
        visit_mut::VisitMut,
    };

    pub enum Feature {
        Frontend,
        Grader,
    }

    impl Feature {
        pub fn to_attr(&self) -> Vec<Attribute> {
            match self {
                Feature::Frontend => todo!(),
                Feature::Grader => todo!(),
            }
        }
    }

    pub type Path = Punctuated<PathSegment, Colon2>;
    pub type Map = HashMap<Path, Option<Vec<Attribute>>>;

    pub struct ItemRecorder<R, F: for<'ast> FnMut(&mut R, &'ast Item, &Path) -> bool> {
        func: F,
        path: Path,
        record: R,
    }

    trait ToPathSegment {
        fn to_path_seg(&self, path: &mut Path);
    }

    fn push_ident(s: &str, span: proc_macro2::Span, path: &mut Path) {
        Ident::new(s, span).to_path_seg(path)
    }

    impl ToPathSegment for Ident {
        fn to_path_seg(&self, path: &mut Path) {
            let seg = PathSegment { ident: self.clone(), arguments: PathArguments::None };
            path.push(seg);
        }
    }

    impl ToPathSegment for syn::TypeParam {
        fn to_path_seg(&self, path: &mut Path) {
            let syn::TypeParam { ident, colon_token, /*bounds,*/ eq_token, default, .. } = self;
            let span = ident.span();

            let mut s = String::new();

            if let Some(_) = colon_token { s.push_str("::"); }
            s.push_str(format!("{}", ident).as_str());
            if let Some(_) = eq_token { s.push_str("="); }
            if let Some(_) = default { s.push_str("_def_"); }

            // [HACK]: We bail and ignore bounds...

            push_ident(s.as_str(), span, path)
        }
    }

    impl ToPathSegment for syn::Generics {
        fn to_path_seg(&self, path: &mut Path) {
            let span = if let Some(lt) = self.lt_token {
                lt.spans[0];
            } else {
                assert!(self.params.len() == 0);
                return;
            };

            push_ident("%GEN%", span, path);

            for ty in self.type_params {
                ty.to_path_sef(path);
            }

            // let ident = format!("Gen({})-", self.type_params().fold(String::new(), |s, ty| {
            //     // Assumes we won't have multiple impls with the same
            //     // self type, generic arg names, and trait name **but**
            //     // with different bounds on those generic args.
            //     //
            //     // This is definitely not always a valid assumption but
            //     // we'll call it good enough for this!
            //     // [HACK]
            //     write!(s, "{},", ty).unwrap();
            //     s
            // });
        }
    }

    impl ToPathSegment for syn::ItemImpl {
        fn to_path_seg(&self, path: &mut Path) -> PathSegment {
            let syn::ItemImpl { unsafety, defaultness, .. } = self;
            let mut s = String::from("%IMPL%");

            if let Some(_) = unsafety { s.push_str("_unsafe_") }
            if let Some(_) = defaultness { s.push_str("_def_") }

            push_ident(s.as_str(), self.impl_token.span, path);

            let syn::ItemImpl { generics, trait_, self_ty, .. } = self;
            generics.to_path_seg(path);
            trait_.to_path_seg(path);
            self_ty.to_path_seg(path);

        }
    }

    impl<'ast, R, F: FnMut(&mut R, &'ast Item, &Path) -> bool> Visit<'ast> for ItemRecorder<R, F> {
        fn visit_item(&mut self, i: &'ast Item) {
            use Item::*;
            match i {
                ForeignMod(ItemForeignMod),

                // Not perfect but should do...
                //
                // It's unfortunate that we're recreating a bad name mangler
                // here.
                Impl(syn::ItemImpl { generics, trait_, self_ty, .. }) => {
                    let mut ident = String::new();

                    write!(ident, "Gen({})-", generics.type_params().fold(String::new(), |s, ty| {
                        // Assumes we won't have multiple impls with the same
                        // self type, generic arg names, and trait name **but**
                        // with different bounds on those generic args.
                        //
                        // This is definitely not always a valid assumption but
                        // we'll call it good enough for this!
                        // [HACK]
                        write!(s, "{},", ty).unwrap();
                        s
                    }).unwrap();

                    write!(ident, "Trait({})-", if let Some(neg, path, _) = trait_ {
                        let mut s = String::new();

                        if let Some(neg) = neg { s.push_str("!"); }

                        let syn::Path { leading_colon, segments } = path;
                        if let Some(_) = leading_colon { s.push_str("::") }

                        let trait_path = segments.pairs().fold(String::new(), |s, p| {
                            match p {
                                syn::punctuated::Punctuated(seg, _) => {
                                    write!(s, "{}", seg.ident).unwrap();

                                    match seg.arguments {
                                        syn::PathArguments::None,
                                        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { colon2_token, args, .. }) => {
                                            if let Some(_) = colon2_token { s.push_str("::") }

                                            let arg = args.pairs().fold(String::new(), |s, p| {
                                                // Here's where we bail.
                                                // [HACK]
                                                match p {
                                                    syn::punctuated::Punctuated(ga, _) => {

                                                    },

                                                }
                                            });

                                            s.push_str(arg.as_str());
                                        },
                                        syn::PathArguments::Parenthesized(p) => {

                                        }
                                    }

                                    s.push_str("::");
                                }
                                syn::punctuated::End(seg) => {
                                    s.push_str();
                                }
                            }
                        });

                        s.push(trait_path.as_str());
                    } else {
                        String::new();
                    }).unwrap();

                    write!(ident, "For({})", self_ty.)
                }

                Const(syn::ItemConst { ident, .. }) |
                Enum(syn::ItemEnum { ident, .. }) |
                ExternCrate(syn::ItemExternCrate { ident, .. }) |
                Fn(syn::ItemFn { sig: syn::Signature { ident, .. }, .. }) |
                Macro(ItemMacro),
                Macro2(ItemMacro2),
                Mod(ItemMod),
                Static(ItemStatic),
                Struct(ItemStruct),
                Trait(ItemTrait),
                TraitAlias(ItemTraitAlias),
                Type(ItemType),
                Union(ItemUnion),
                Use(ItemUse),
                Verbatim(TokenStream),
            }

            if (self.func)(&mut self.record, i) {
                syn::visit::visit_item(self, i)
            }
        }
    }

    impl<R, F> ItemRecorder<R, F>
    where
        F: for<'ast> FnMut(&mut R, &'ast Item, &Path) -> bool
    {
        pub /*const*/ fn new(record: R, func: F) -> Self {
            Self {
                func,
                path: Punctuated::new(),
                record,
             }
        }
    }

    fn baseline(file: &File) -> Map {
        let visitor = ItemRecorder::new(Map::new(), |m, i, p| {

        });

        syn::visit::visit_file()
    }

    // impl<R, F: for<'ast> FnMut(&mut R, &'ast syn::Item) -> bool> VisitMut for ItemRecorder<R, F> {
    //     fn visit_item_mut(&mut self, i: &mut Item) {
    //         if (self.func)(&mut self.record, i) {
    //             syn::visit_mut::visit_item(self, i)
    //         }
    //     }
    // }
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

    // TODO: right now this does not check if there are two header files with
    // the same name. As of this writing, all the header files in lc3tools have
    // unique names but if this were to change, we'd lose header files in the
    // generated output without any warning.
    copy_headers(BACKEND, &include)?;
    if cfg!(feature = "frontend") { copy_headers(FRONTEND, &include)? }
    if cfg!(feature = "grader") { copy_headers(GRADER, &include)? }

    // TODO: is `canonicalize` actually broken? (rust#42869)
    println!("cargo:include={}", include.canonicalize()?.display());

    // Next, let's do bindgen, if we're asked to.
    #[cfg(feature = "generate-fresh")]
    {
        // First we want to get the baseline bindings — just the backend, no
        // other features — and record what items this has.
        let backend = make_bindings(&[BACKEND]).unwrap();

    }

    // Finally let's go gather the C++ files and do the build.
    let mut build = Build::new();
    // `cc` automatically handles `OPT_LEVEL` and `DEBUG`.
    // `cc` also handles `fPIC`

    build
        .flag_if_supported("-flto")
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-format-security")
        .cargo_metadata(true)
        .warnings(true)
        .extra_warnings(true)
        .cpp(true);

    // Debug settings:
    if env!("PROFILE") == "debug" {
        build.define("_ENABLE_DEBUG", None);
    }

    // Includes:
    build.include(BACKEND)
    if cfg!(feature = "grader") { build.include(GRADER); }
    if cfg!(feature = "frontend") { build.include(FRONTEND); }

    // Collecting files:
    let cpp_dir_iter = |dir| in_dir_with_ext(dir, "cpp")
        .expect(format!("Source files in `{}`", dir).as_str());

    let files = cpp_dir_iter(BACKEND);
    #[cfg(feature = "grader")]
    let files = files.chain(cpp_dir_iter(GRADER));
    #[cfg(feature = "frontend")]
    let files = files.chain(cpp_dir_iter(FRONTEND);

    for source_file in source_files {
        println!("cargo:rerun-if-changed={}", source_file.path().display());
        build.file(source_file.path());
    }

    // And finally, the build:
    // `cc` automatically tells cargo to link to this statically.
    build.out_dir(out.join("build")).compile("lc3core");
    println!("cargo:root={}", out.display());

    Ok(())
}
