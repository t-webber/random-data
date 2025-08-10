//! Generate random data of any kind.

#![warn(
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
#![allow(
    clippy::allow_attributes,
    unused_imports,
    reason = "features can make lint unfulfilled"
)]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![allow(clippy::missing_docs_in_private_items, reason = "explicit names")]
#![allow(clippy::non_ascii_literal, reason = "not-understandable")]
#![expect(clippy::self_named_module_files, reason = "style choice")]
#![expect(clippy::pub_use, reason = "simpler and more robust API")]
#![allow(
    clippy::single_call_fn,
    clippy::pattern_type_mismatch,
    clippy::module_name_repetitions,
    clippy::else_if_without_else,
    clippy::implicit_return,
    reason = "bad lint"
)]
#![expect(clippy::unseparated_literal_suffix, reason = "chosen style")]
#![allow(clippy::missing_inline_in_public_items, reason = "bad lint")]
#![expect(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    reason = "numbers are small"
)]

mod data;
mod generator;
pub use crate::{data::DataType, generator::DataGenerator};
