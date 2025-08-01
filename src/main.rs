//! CLI to generate some fake data under JSON format.

#![warn(
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
#![expect(clippy::multiple_crate_versions, reason = "needed by used crates")]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![expect(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    reason = "bad lints"
)]
#![expect(
    clippy::mod_module_files,
    clippy::module_name_repetitions,
    reason = "chosen style"
)]

mod clap;
mod data;
mod dialog;
mod errors;
mod json;
use rand::rng;

use crate::clap::CliArgs;

fn main() {
    let mut rng = rng();
    CliArgs::parse_and_run(&mut rng);
}
