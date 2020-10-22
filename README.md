twitter-text in Rust
============

This repo is a Rust implementation of twitter-text. All aspects of tweet text are parsed by a [Pest](https://github.com/pest-parser/pest) [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar) grammar, with the exception of URL length and character weighting. See the [parser](rust/parser/src) directory for the grammar. Procedural validation for URL lengths and  character weights is performed by the [Extractor](twitter-text/src/extractor.rs) code.

To run the tests, [install Rust](https://www.rust-lang.org/tools/install), and then try this in the terminal:
```
> cargo build
> cargo test
```

The original Twitter README content is below.

twitter-text
============

This repo is a collection of libraries and conformance tests to standardize parsing of Tweet text. It synchronizes development, testing, creating issues, and pull requests for twitter-text's implementations and specification. These libraries are responsible for determining the quantity of characters in a Tweet and identifying and linking any url, @username, #hashtag, or $cashtag.

See implementations and conformance in this repo below:

* [Conformance](conformance)
* [Java](java)
* [Ruby](rb)
* [JavaScript](js)
* [Objective-C](objc)

## Copyright and License

Copyright 2012-2018 Twitter, Inc and other contributors

Copyright 2019 Robert Sayre

Licensed under the Apache License, Version 2.0: http://www.apache.org/licenses/LICENSE-2.0
