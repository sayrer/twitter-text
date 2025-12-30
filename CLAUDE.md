# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This repository is a Rust implementation of twitter-text that parses tweet text using a Pest PEG grammar. It includes multiple language bindings (Ruby, Python, Java, C++) and maintains conformance with the original twitter-text specification.

## Build System

The project uses **Bazel** as the primary build system, with Cargo available for standalone Rust development.

## Adding crates

New crates are add by modifying //3rdparty:crates_vendor, running that, and then making sure the Cargo.toml files match.

### Common Build Commands

```bash
# Build all Rust code
bazel build //rust/...

# Run all tests
bazel test //rust/...

# Build specific language bindings
bazel build //rust/ruby-bindings:twittertext
bazel build //rust/java-bindings:twitter_text_java_ffm
bazel build //rust/cpp-bindings/...

# Run specific test suites
bazel test //rust/ruby-bindings/spec:all
bazel test //rust/java-bindings/tests:all
bazel test //rust/cpp-bindings/...
```

### Cargo (for Rust development)

```bash
# Build Rust workspace
cargo build

# Run Rust tests
cargo test

# Test a specific package
cargo test -p twitter-text
cargo test -p twitter-text-parser
```

## Architecture

### Core Components

1. **PEG Grammar Parser** (`rust/parser/`)
   - Uses Pest parser generator with `twitter_text.pest` grammar
   - Parses URLs, @mentions, #hashtags, $cashtags, and emojis
   - Order of entity matching is significant (URLs before hashtags to resolve ambiguities)

2. **Main Library** (`rust/twitter-text/src/`)
   - `lib.rs`: Public API and `TwitterTextParseResults` struct
   - `extractor.rs`: Core extraction logic, validates URLs and character weights
   - `validator.rs`: Tweet validation (length limits, invalid characters)
   - `autolinker.rs`: Converts entities to HTML links
   - `hit_highlighter.rs`: Highlights search hits in tweet text
   - `entity.rs`: Entity data structures

3. **Configuration** (`rust/config/`)
   - Loads config from `config/v*.json`
   - Defines character weights and URL length limits
   - Used for tweet length calculation (280 weighted characters)

4. **Conformance Tests** (`rust/conformance/`)
   - Tests against YAML test suites in `conformance/` directory
   - Cross-platform test definitions shared with other twitter-text implementations
   - Do not modify conformance YAML files directly; they are canonical

### Language Bindings

- **Ruby** (`rust/ruby-bindings/`): Uses Magnus FFI, requires Ruby 3.3+
- **Java** (`rust/java-bindings/`): Uses JDK 23+ Foreign Function & Memory API (Project Panama)
- **Python** (`rust/python-bindings/`): Uses PyO3
- **C++** (`rust/cpp-bindings/`): Uses cxx.rs for safe Rust-C++ interop
- **Swift** (`rust/swift-bindings/`): Uses C FFI with auto-generated Clang modules

## Testing

### Running Single Tests

```bash
# Bazel test with filter
bazel test //rust/twitter-text:tests --test_filter=test_name

# Cargo test with filter
cargo test test_name

# Run specific conformance test file
bazel test //rust/conformance:autolink_test
```

### Conformance Tests

The conformance suite (`conformance/*.yml`) defines canonical test cases. All implementations must pass these tests. Tests cover:
- Autolink (URL/mention/hashtag linking)
- Extract (entity extraction)
- Validation (tweet validity)
- Hit highlighting

## Key Implementation Details

### URL Length Calculation

- URLs are weighted based on `Configuration.transformed_url_length`
- Character weights are defined in `Configuration.ranges` (most characters: 2, ASCII/Latin-1: 1)
- Tweet length limit: 280 weighted characters

### NFC Normalization

- Input text is normalized to Unicode NFC form
- Use `ValidatingExtractor::new_with_nfc_input()` if input is already NFC-normalized for efficiency

### Entity Parsing Order

The PEG grammar processes entities in this order:
1. URLs (including t.co short URLs)
2. Hashtags
3. Mentions
4. Cashtags

This order prevents ambiguities (e.g., `goo.gl/4udoLK` could match as hashtag but should be URL).

## Development Workflow

1. **Rust Changes**: Edit code in `rust/` directories, run `cargo test` for quick feedback
2. **Grammar Changes**: Modify `rust/parser/src/twitter_text.pest`, rebuild parser
3. **Conformance**: Ensure changes pass `bazel test //rust/conformance:all`
4. **Bindings**: Test language bindings after core changes to ensure FFI compatibility

## Dependencies

- **Rust**: 1.91.1 (edition 2021)
- **Bazel**: 8.4.2+
- **Ruby**: 3.3+ (for Ruby bindings, requires libyaml)
- **Java**: JDK 23+ (for Java FFM bindings)
- **Python**: 3.12 (for Python bindings)
- **LLVM**: 17.0.6 (hermetic toolchain via Bazel)

## Swift Bindings

The Swift bindings (`rust/swift-bindings/`) provide a Swift-friendly API over the shared C FFI layer.

### Building Swift Library

```bash
# Build Swift library (works on both Linux and macOS)
bazel build //rust/swift-bindings:TwitterText

# Produces: libTwitterText.a, TwitterText.swiftmodule, TwitterText.swiftdoc
```

### Running Swift Tests

**On macOS** (recommended):
```bash
# Tests work via Bazel on macOS
bazel test //rust/swift-bindings:TwitterTextTests
```

**On Linux**:
```bash
# Bazel tests don't work on Linux due to rules_swift linker compatibility
# Use Swift Package Manager instead:
cd rust/swift-bindings
swift test
```

### Swift Toolchain Details

- **Linux**: Uses hermetic Swift 6.0.3 toolchain downloaded by Bazel (see `MODULE.bazel`)
- **macOS**: Uses system Swift from Xcode (automatically detected by rules_swift)
- **C Interop**: C headers from `//rust/ffi-bindings` are auto-wrapped via `swift_interop_hint`
- **Module Name**: Swift code imports `CTwitterText` to access C FFI functions

### Known Limitations

Swift binary tests cannot run via Bazel on Linux because rules_swift passes Darwin-specific linker flags (`-add_ast_path`) that ld.lld doesn't understand. This is a rules_swift limitation when targeting Linux. The workaround is to use Swift Package Manager for testing on Linux, or test on macOS where the native toolchain works correctly.

## Notes

- Ruby bindings require system libyaml (`brew install libyaml` on macOS)
- Java bindings use jextract to generate FFM code from C headers
- Sanitizer tests available: `bazel test //rust/cpp-sanitizer-bindings/...`
- MODULE.bazel contains all external dependencies and toolchain configurations
- Swift bindings share the same C FFI layer (`//rust/ffi-bindings`) with Java and C++ bindings
