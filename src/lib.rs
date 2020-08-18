#![cfg_attr(all(docs, not(doctest)), feature(doc_cfg))]
#![cfg_attr(all(docs, not(doctest)), feature(external_doc))]
#![cfg_attr(all(docs, not(doctest)), doc(include = "../README.md"))]
//!
// ^ is there so it looks like we have at some crate level docs when building
// without `--cfg docs` (i.e. on stable, when not building docs).

#![forbid(
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused_allocation,
    unused_lifetimes,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![deny(
    unused,
    bad_style,
    missing_debug_implementations,
    intra_doc_link_resolution_failure,
    missing_docs,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    rust_2018_idioms
)]
#![doc(test(attr(
    deny(rust_2018_idioms, warnings),
    allow(unused_extern_crates)
)))]
#![doc(
    html_logo_url = "https://github.com/chiragsakhuja/lc3tools/raw/848bb987d3675b45fdc794ebf995cba5c60373ac/frontend/gui/static/icons/256x256.png",
    html_root_url = "https://docs.rs/lc3tools-sys/1.0.6-alpha2", // remember to bump!
)]
#![allow(
    clippy::all,
    clippy::pedantic,
    improper_ctypes,
    missing_docs,
    missing_debug_implementations,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    trivial_casts,
    unsafe_code,
    unused_qualifications
)]
#![deny(clippy::cargo)]
// Note: Our MSRV doesn't have `broken_intra_doc_links` so we do this.
#![allow(unknown_lints)]
#![deny(broken_intra_doc_links)]
#![warn(unknown_lints)]

include!("../generated/bindings.rs");
