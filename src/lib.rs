//! Generate random data of any kind.

#![allow(
    bindings_with_variant_name,
    non_snake_case,
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
// #![expect(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
// #![allow(clippy::missing_docs_in_private_items, reason = "explicit names")]
// #![expect(clippy::non_ascii_literal, reason = "not-understandable")]
// #![expect(clippy::self_named_module_files, reason = "style choice")]

mod strings;

pub use strings::DataGenerator;
pub use strings::DataType;
