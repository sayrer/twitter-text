# Parser Backend Options

This document captures a discussion about optimizing the twitter-text parser, particularly TLD matching.

## Problem

Pest doesn't generate efficient code for the TLD trie. The TLD rule in `twitter_text.pest` is a giant ordered-choice expression (`|`) with ~1500 TLDs. Pest's PEG semantics mean it tries each alternative sequentially until one matches. For a TLD like `zone` (near the end alphabetically), it potentially tests hundreds of alternatives first.

Pest also compiles to a bytecode VM, not native code, so each choice point has overhead.

## Proposed Solution

Add a `ParserBackend` enum to select parsing strategies:

```rust
pub enum ParserBackend {
    /// Pure Pest parsing (current behavior)
    Pest,
    /// Pest for structure, external TLD lookup
    PestWithExternalTld,
}
```

Pass this to `Extractor::new()` or `ValidatingExtractor::new()`.

### PestWithExternalTld Implementation

The implementation would branch in `validate_url` (around `extractor.rs:647`). Currently it walks the Pest parse tree to find the `host`/`tco_domain`/`uwp_domain` rule. With external TLD matching:

1. The Pest grammar would match a more permissive "domain-like" rule
2. Rust code extracts the potential TLD suffix
3. A `phf` (perfect hash function) or hand-written trie lookup validates it

Options for the external TLD lookup:
- `phf` / `phf_codegen` - compile-time perfect hash map, O(1) lookup
- `aho-corasick` - efficient multi-pattern matching
- Hand-written trie in Rust

### Testing Parity

Run the full conformance test suite against both backends and diff the results. The `//rust/conformance:tlds_test` is specifically designed for TLD validation.

### Runtime vs Compile-time Selection

Runtime enum is preferred. The branch occurs once in a constructor, not in a hot loop, so the overhead is negligible. Runtime selection is simpler and more flexible for testing that both backends produce identical results.

## Alternative: Full Parser Replacement

For reference, here's an assessment of replacing Pest entirely:

### Pest to Tree-sitter
**Difficulty: High**

- Tree-sitter uses its own grammar DSL (JavaScript-based), not PEG
- Designed for incremental parsing of source code - overkill for tweets
- Unclear payoff since tweets are short strings

### Pest to Nom
**Difficulty: Medium-High**

- Nom uses Rust combinators; grammar becomes Rust code
- Translation is mechanical but tedious (~900 line `.pest` becomes ~1500+ lines of Rust)
- Can be faster than Pest (zero-copy, no VM overhead)
- Harder to maintain: grammar changes require Rust code changes

## Recommendation

Start with `PestWithExternalTld` - it's lower risk than a full parser replacement, targets the known bottleneck (TLD matching), and the conformance tests will validate correctness.
