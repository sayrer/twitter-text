// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use pest_derive::Parser;

/// Parser using a permissive TLD grammar. TLD validation is done externally
/// via phf lookup for O(1) performance. This is the default parser.
#[derive(Parser)]
#[grammar = "twitter_text.pest"]
pub struct TwitterTextParser;

/// Module containing the full TLD parser with all ~1500 TLDs in the grammar.
pub mod full_tld {
    use pest_derive::Parser;

    /// Parser using the full TLD grammar with all ~1500 TLDs as alternations.
    /// TLD validation is done by Pest itself. This is slower but self-contained.
    #[derive(Parser)]
    #[grammar = "twitter_text_full_tld.pest"]
    pub struct TwitterTextFullTldParser;
}
