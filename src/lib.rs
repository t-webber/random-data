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
    reason = "features can make lint unfulfilled"
)]
#![allow(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![allow(clippy::missing_docs_in_private_items, reason = "explicit names")]
#![allow(clippy::non_ascii_literal, reason = "not-understandable")]
#![allow(clippy::self_named_module_files, reason = "style choice")]
#![allow(clippy::pub_use, reason = "simpler and more robust API")]
#![allow(
    clippy::single_call_fn,
    clippy::pattern_type_mismatch,
    clippy::module_name_repetitions,
    clippy::else_if_without_else,
    clippy::implicit_return,
    reason = "bad lint"
)]
#![allow(clippy::unseparated_literal_suffix, reason = "chosen style")]
#![allow(clippy::missing_inline_in_public_items, reason = "bad lint")]
#![allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    reason = "numbers are small"
)]

#[cfg(not(any(
    feature = "address",
    feature = "animals",
    feature = "art",
    feature = "colours",
    feature = "computer",
    feature = "currencies",
    feature = "datetime",
    feature = "education",
    feature = "finance",
    feature = "france",
    feature = "history",
    feature = "internet",
    feature = "minimal",
    feature = "names",
    feature = "people",
    feature = "personal",
    feature = "science",
    feature = "sky_space",
    feature = "text",
    feature = "uk",
    feature = "us",
    feature = "work",
    feature = "world",
)))]
compile_error!("You must enable at least one feature!");

mod data;
mod generator;
mod primitives;

pub use crate::{data::DataType, generator::DataGenerator};
