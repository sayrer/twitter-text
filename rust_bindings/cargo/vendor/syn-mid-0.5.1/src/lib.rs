//! Providing the features between "full" and "derive" of syn.
//!
//! This crate provides the following two unique data structures.
//!
//! * [`syn_mid::ItemFn`] -- A function whose body is not parsed.
//!
//!   ```text
//!   fn process(n: usize) -> Result<()> { ... }
//!   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^     ^
//!   ```
//!
//! * [`syn_mid::Block`] -- A block whose body is not parsed.
//!
//!   ```text
//!   { ... }
//!   ^     ^
//!   ```
//!
//! Other data structures are the same as data structures of [syn]. These are defined in this crate
//! because they cannot be used in [syn] without "full" feature.
//!
//! # Optional features
//!
//! syn-mid in the default features aims to provide the features between "full"
//! and "derive" of [syn].
//!
//! * **`clone-impls`** — Clone impls for all syntax tree types.
//!
//! [`syn_mid::ItemFn`]: ItemFn
//! [`syn_mid::Block`]: Block
//! [syn]: https://github.com/dtolnay/syn

#![doc(html_root_url = "https://docs.rs/syn-mid/0.5.1")]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms, single_use_lifetimes), allow(dead_code))
))]
#![forbid(unsafe_code)]
#![warn(future_incompatible, rust_2018_idioms, single_use_lifetimes, unreachable_pub)]
#![warn(clippy::all, clippy::default_trait_access)]
// mem::take and #[non_exhaustive] requires Rust 1.40, matches! requires Rust 1.42
#![allow(
    clippy::mem_replace_with_default,
    clippy::manual_non_exhaustive,
    clippy::match_like_matches_macro
)]
#![allow(clippy::eval_order_dependence, clippy::large_enum_variant)]

// Many of the code contained in this crate are copies from https://github.com/dtolnay/syn.

#[macro_use]
mod macros;

mod func;
mod pat;
mod path;

pub use crate::{
    func::{Block, FnArg, ItemFn, Receiver, Signature},
    pat::{
        FieldPat, Pat, PatIdent, PatPath, PatReference, PatStruct, PatTuple, PatTupleStruct,
        PatType, PatWild,
    },
};
