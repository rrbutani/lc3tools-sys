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
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::missing_docs_in_private_items)]
#![allow(clippy::type_repetition_in_bounds)]
#![doc(test(attr(
    deny(rust_2018_idioms, warnings),
    allow(unused_extern_crates)
)))]
#![doc(
    html_logo_url = "{{ crate logo url (or remove this line for the default rust one  )}}",
    html_root_url = "https://docs.rs/{{ crate name }}/*", // * → latest version
)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
