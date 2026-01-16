# twitter-text in Rust

A Rust implementation of [twitter-text](https://github.com/twitter/twitter-text) that parses tweet text using a [Pest](https://github.com/pest-parser/pest) [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar. Includes bindings for Ruby, Python, Java, C++, Swift, and WebAssembly.

## Features

- **Entity extraction**: URLs, @mentions, #hashtags, $cashtags, and emoji
- **Tweet validation**: 280 weighted character limit with configurable weights
- **Autolinking**: Convert entities to HTML links
- **Hit highlighting**: Highlight search terms in tweet text
- **Unicode 17.0**: Full emoji support including ZWJ sequences and skin tone modifiers

## Quick Start

### Using Cargo

```bash
cargo build
cargo test
```

### Using Bazel

```bash
# Build everything
bazel build //rust/...

# Run all tests
bazel test //rust/...
```

## Language Bindings

| Language | Directory | Requirements | Technology |
|----------|-----------|--------------|------------|
| Ruby | `rust/ruby-bindings/` | Ruby 3.3+ | [Magnus](https://github.com/matsadler/magnus) FFI |
| Python | `rust/python-bindings/` | Python 3.12 | [PyO3](https://github.com/PyO3/pyo3) |
| Java | `rust/java-bindings/` | JDK 23+ | Foreign Function & Memory API |
| C++ | `rust/cpp-bindings/` | C++17 | [cxx.rs](https://github.com/dtolnay/cxx) |
| Swift | `rust/swift-bindings/` | Swift 6.0+ | C FFI |
| WebAssembly | `rust/wasm-bindings/` | - | [wasm-bindgen](https://github.com/AshleyScirra/nicerm) |

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

# WebAssembly
bazel build //rust/wasm-bindings:twitter_text_wasm
```

## Architecture

### Core Components

- **PEG Grammar Parser** (`rust/parser/`): Pest grammar for parsing tweet entities
- **Nom Parser** (`rust/twitter-text/src/nom_parser/`): High performance parser using [Nom](https://docs.rs/nom/latest/nom/), tested against the PEG grammar for correctness
- **Main Library** (`rust/twitter-text/`): Extraction, validation, autolinking, and highlighting
- **Configuration** (`rust/config/`): Character weights and URL length settings
- **Conformance Tests** (`rust/conformance/`): Tests against canonical twitter-text test suites

### Entity Parsing Order

The grammar processes entities in this order to resolve ambiguities:
1. URLs (including t.co short URLs)
2. Hashtags
3. Mentions
4. Cashtags

## Dependencies

- **Rust**: 1.91.1+
- **Bazel**: 8.4.2+ (for full build)
- **Ruby**: 3.3+ (requires libyaml: `brew install libyaml` on macOS)
- **Python**: 3.12
- **Java**: JDK 23+
- **LLVM**: 17.0.6 (hermetic toolchain via Bazel)

## Conformance

This implementation passes the canonical twitter-text conformance tests in `conformance/*.yml`. These tests cover:
- Autolink (URL/mention/hashtag linking)
- Extract (entity extraction)
- Validation (tweet validity)
- Hit highlighting

## License

Apache 2.0
