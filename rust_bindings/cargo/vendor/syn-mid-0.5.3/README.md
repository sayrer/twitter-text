# syn-mid

[![crates.io](https://img.shields.io/crates/v/syn-mid.svg?style=flat-square&logo=rust)](https://crates.io/crates/syn-mid)
[![docs.rs](https://img.shields.io/badge/docs.rs-syn--mid-blue?style=flat-square)](https://docs.rs/syn-mid)
[![license](https://img.shields.io/badge/license-Apache--2.0_OR_MIT-blue.svg?style=flat-square)](#license)
[![rustc](https://img.shields.io/badge/rustc-1.31+-blue.svg?style=flat-square)](https://www.rust-lang.org)
[![build status](https://img.shields.io/github/workflow/status/taiki-e/syn-mid/CI/master?style=flat-square)](https://github.com/taiki-e/syn-mid/actions?query=workflow%3ACI+branch%3Amaster)

Providing the features between "full" and "derive" of syn.

This crate provides the following two unique data structures.

- `syn_mid::ItemFn` -- A function whose body is not parsed.

  ```text
  fn process(n: usize) -> Result<()> { ... }
  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ^     ^
  ```

- `syn_mid::Block` -- A block whose body is not parsed.

  ```text
  { ... }
  ^     ^
  ```

Other data structures are the same as data structures of [syn]. These are
defined in this crate because they cannot be used in [syn] without "full"
feature.

[syn]: https://github.com/dtolnay/syn

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
syn-mid = "0.5"
```

*Compiler support: requires rustc 1.31+*

[**Examples**](examples)

## Optional features

- **`clone-impls`** — Clone impls for all syntax tree types.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
