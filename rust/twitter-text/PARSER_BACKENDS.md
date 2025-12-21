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

## Emoji Matching

The emoji case is similar to TLDs - it's a large set of patterns (emoji sequences, ZWJ combinations, skin tone modifiers, etc.) that Pest handles as a big alternation.

The Unicode Consortium publishes machine-readable emoji data files that could drive a procedural matcher:
- `emoji-sequences.txt`
- `emoji-zwj-sequences.txt`
- `emoji-test.txt` (already in conformance tests)

A procedural emoji matcher would:
1. Check if a code point is in the emoji range
2. Handle ZWJ (U+200D) sequences by consuming the full chain
3. Handle variation selectors (U+FE0E, U+FE0F)
4. Handle skin tone modifiers (U+1F3FB-U+1F3FF)
5. Handle flag sequences (regional indicator pairs)

This is more complex than TLD matching since it's stateful (consuming a variable-length sequence), but it's well-defined by Unicode TR51.

## Compositional Configuration

Rather than a single enum, a compositional approach allows testing each optimization independently:

```rust
pub enum ParserBackend {
    /// Pure Pest parsing (original behavior)
    Pest,
    /// Pest for structure, external validation via phf/procedural code
    External,
}
```

This lets you:
- Test TLD optimization in isolation
- Test emoji optimization in isolation
- Combine both when validated
- Keep pure Pest as a reference implementation

## Conformance Test Coverage

The conformance tests are mostly backend-agnostic - they test through the public API:
- `autolink.rs`, `extract.rs`, `validate.rs`, `hit_highlighting.rs`, `tlds.rs` - use public API
- `emoji.rs` - directly uses Pest to validate emoji parsing

For backend changes, all tests except `emoji.rs` would validate correctness through the public API. The `emoji.rs` test specifically validates that the Pest grammar handles all Unicode 11 emoji sequences, so it would need updating if emoji parsing moves to procedural code.

## Phased Optimization Approach

### Phase 1: Extract TLD matching
- Replace Pest TLD alternation with `phf` perfect hash or hand-written trie
- Biggest algorithmic win (O(k*n) → O(1) or O(n) in TLD length)
- Low risk - Pest still handles structural parsing
- Validate with conformance tests

### Phase 2: Extract emoji matching
- Replace Pest emoji alternation with procedural Unicode TR51 matcher
- Similar algorithmic win
- Validate with conformance tests and `emoji.rs`

### Phase 3: Translate remaining grammar to nom
- With TLD/emoji removed, the Pest grammar is much simpler
- Mechanical translation becomes tractable
- Constant-factor speedup (2-3x) from eliminating bytecode VM
- nom compiles to native code, enables LLVM inlining

At each phase, the pure Pest backend remains as a reference implementation to validate correctness.

## Why nom alone isn't enough

A direct Pest-to-nom translation preserves PEG ordered-choice semantics. Translating `tld = { "com" | "org" | ... | "zone" }` to `alt(("com", "org", ..., "zone"))` still tries each alternative sequentially - same O(k*n) algorithm, just without interpreter overhead.

The real wins come from changing the algorithm (phases 1-2), not just the parser framework. Phase 3 then provides a constant-factor improvement on top of the algorithmic gains.

## Recommendation

Start with Phase 1 (external TLD matching) - it's the known bottleneck, lowest risk, and the conformance tests will validate correctness. Progress through phases as profiling confirms each optimization.
