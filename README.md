# twitter-text in Rust

A Rust implementation of [twitter-text](https://github.com/twitter/twitter-text) that parses tweet text using a [Pest](https://github.com/pest-parser/pest) [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar. Includes bindings for Ruby, Python, Java, C++, Swift, Objective-C, and WebAssembly. There's a [blog post](https://sayrer.com/blog/whats-in-a-tweet/) that covers the background.

## Features

- **Entity extraction**: URLs, @mentions, #hashtags, $cashtags, and emoji
- **Tweet validation**: 280 weighted character limit with configurable weights
- **Autolinking**: Convert entities to HTML links
- **Hit highlighting**: Highlight search terms in tweet text
- **Unicode 18.0 emoji**: Full emoji support including ZWJ sequences and skin tone modifiers
- **Fair weighting for every alphabet**: An opt-in v4 configuration, derived from the Unicode 18.0 Character Database, weighs Ethiopic, Khmer, Cherokee, Runic, Adlam, and some 170 other alphabetic scripts like Latin instead of like CJK

## Quick Start

### Using Bazel

Bazel downloads every toolchain it needs (Rust, LLVM, Python, JDK, Swift on Linux), so this is the most reproducible path.

```bash
# Build everything
bazel build //rust/...

# Run all tests
bazel test //rust/...
```

### Using Cargo

The Cargo workspace lives under `rust/`, not at the repository root.

```bash
cd rust
cargo build
cargo test
```

## Configuration

Tweet length is computed from a JSON configuration (`config/v*.json`) that assigns a weight to each range of code points. Four versions are included:

| Version | Description |
|---------|-------------|
| v1 | Legacy 140-character tweets, every code point weighs 1 |
| v2 | 280 weighted characters; code points up to U+10FF and general punctuation weigh 1, everything else weighs 2 |
| v3 | v2 plus emoji parsing, so an emoji sequence counts as one character. **This is the default and matches X/Twitter.** |
| v4 | v3 plus every alphabetic script encoded above U+10FF weighted like Latin. Opt-in. |

v1 through v3 are byte-identical to upstream twitter-text. v4 is generated from the Unicode Character Database by `config/generate_v4.py`, which also emits a Rust test for each of the 171 included blocks. See `config/README.md` for the format and the inclusion rule, and [twitter-text#430](https://github.com/twitter/twitter-text/issues/430) for the motivation.

## Language Bindings

| Language | Directory | Requirements | Technology |
|----------|-----------|--------------|------------|
| Ruby | `rust/ruby-bindings/` | Ruby 3.4.1+ | [Magnus](https://github.com/matsadler/magnus) FFI |
| Python | `rust/python-bindings/` | Python 3.12 | [PyO3](https://github.com/PyO3/pyo3) |
| Java | `rust/java-bindings/` | JDK 25 (hermetic) | Foreign Function & Memory API, [jextract](https://jdk.java.net/jextract/) |
| C++ | `rust/cpp-bindings/` | C++17 | [cxx.rs](https://github.com/dtolnay/cxx) |
| Swift | `rust/swift-bindings/` | Swift 5.9+ | C FFI via `//rust/ffi-bindings` |
| Objective-C | `rust/swift-bindings/objc/` | Xcode | C FFI via `//rust/ffi-bindings` |
| WebAssembly | `rust/wasm-bindings/` | Node.js for tests | [wasm-bindgen](https://github.com/wasm-bindgen/wasm-bindgen) |

### Building Bindings

```bash
# Ruby
bazel build //rust/ruby-bindings:twittertext

# Python
bazel build //rust/python-bindings:twitter_text

# Java
bazel build //rust/java-bindings:twitter_text_java_ffm

# C++
bazel build //rust/cpp-bindings/...

# Swift
bazel build //rust/swift-bindings:TwitterText

# WebAssembly (browser and Node.js packages)
bazel build //rust/wasm-bindings:twitter_text_wasm
bazel build //rust/wasm-bindings:twitter_text_wasm_nodejs
```

Swift tests run under Bazel on macOS. On Linux, rules_swift passes Darwin-only linker flags to the test binary, so use `swift test` in `rust/swift-bindings/` instead.

## Architecture

### Core Components

- **PEG Grammar Parser** (`rust/parser/`): Pest grammar for parsing tweet entities
- **Nom Parser** (`rust/twitter-text/src/nom_parser/`): High performance parser using [Nom](https://docs.rs/nom/latest/nom/), tested against the PEG grammar for correctness
- **Main Library** (`rust/twitter-text/`): Extraction, validation, autolinking, and highlighting
- **Configuration** (`rust/config/`): Character weights and URL length settings
- **Conformance Tests** (`rust/conformance/`): Tests against canonical twitter-text test suites
- **C FFI** (`rust/ffi-bindings/`): The C layer shared by the Java, C++, Swift, and Objective-C bindings

### Entity Parsing Order

The grammar processes entities in this order to resolve ambiguities:
1. URLs (including t.co short URLs)
2. Hashtags
3. Mentions
4. Cashtags

## Dependencies

Bazel downloads nearly all of these. They are listed to show which versions the project is built and tested with. The exception is Ruby, which must be installed on the host because the bindings build against the system Ruby.

- **Rust**: 1.99.1 (edition 2021 under Bazel)
- **Bazel**: 9 (use [bazelisk](https://github.com/bazelbuild/bazelisk))
- **LLVM**: 17.0.6 (hermetic toolchain with a Debian sysroot on Linux)
- **Python**: 3.12
- **Java**: Temurin JDK 25
- **Swift**: 6.4.0 hermetic toolchain on Linux; Xcode's toolchain on macOS
- **Ruby**: 3.4.1+ on the host (requires libyaml: `brew install libyaml` on macOS)
- **Unicode data**: [emojis](https://github.com/rossmacarthur/emojis) 0.9.1 for emoji and [unicode-normalization](https://github.com/unicode-rs/unicode-normalization) 0.1.26 for NFC, both at Unicode 18.0

## Continuous Integration

- **BuildBuddy** runs `bazel test` over every Rust, binding, and conformance target on Linux for each push and pull request (`buildbuddy.yaml`).
- **GitHub Actions** runs `cargo fmt --check` and `cargo clippy -D warnings` (`.github/workflows/rust.yml`).

## Conformance

This implementation passes the canonical twitter-text conformance tests in `conformance/*.yml`. These tests cover:
- Autolink (URL/mention/hashtag linking)
- Extract (entity extraction)
- Validation (tweet validity)
- Hit highlighting

## License

Apache 2.0
