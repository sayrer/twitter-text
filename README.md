twitter-text in Rust
============

This repo is a Rust implementation of twitter-text. All aspects of tweet text are parsed by a [Pest](https://github.com/pest-parser/pest) [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar, with the exception of URL length and character weighting. See the [parser](rust/parser/src) directory for the grammar. Procedural validation for URL lengths and  character weights is performed by the [Extractor](rust/twitter-text/src/extractor.rs) code.

To run the tests, [install Rust](https://www.rust-lang.org/tools/install), and then try this in the terminal:
```
> cargo build
> cargo test
```

### Ruby Bindings

The Ruby bindings require **Ruby 3.2.3 or higher**. If you're on macOS with the system Ruby (2.6.x), you'll need to install a newer version:

**Option 1: Homebrew (simplest)**
```bash
brew install ruby
```

**Option 2: Ruby version manager**
* [rbenv](https://github.com/rbenv/rbenv)
* [rvm](https://rvm.io/)
* [asdf](https://asdf-vm.com/)

The Ruby bindings use the [magnus](https://github.com/matsadler/magnus) crate which requires Ruby 3.2.3+ APIs.
