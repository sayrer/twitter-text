// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

pub mod autolinker;
pub mod entity;
pub mod extractor;
pub mod hit_highlighter;
pub mod nom_parser;
pub mod tlds;
pub mod validator;

#[cfg(feature = "ffi")]
pub mod ffi;

use extractor::{Extract, ValidatingExtractor};
use twitter_text_config::Configuration;
use twitter_text_config::Range;

// Re-export ParserBackend for convenience
pub use extractor::ParserBackend;

/// A struct that represents a parsed tweet containing the length of the tweet,
/// its validity, display ranges etc. The name mirrors Twitter's Java implementation.
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct TwitterTextParseResults {
    /// The weighted length is the number used to determine the tweet's length for the purposes of Twitter's limit of 280. Most characters count
    /// for 2 units, while a few ranges (like ASCII and Latin-1) count for 1. See [Twitter's blog post](https://blog.twitter.com/official/en_us/topics/product/2017/Giving-you-more-characters-to-express-yourself.html).
    pub weighted_length: i32,

    /// The weighted length expressed as a number relative to a limit of 1000.
    /// This value makes it easier to implement UI like Twitter's tweet-length meter.
    pub permillage: i32,

    /// Whether the tweet is valid: its weighted length must be under the configured limit, it must
    /// not be empty, and it must not contain invalid characters.
    pub is_valid: bool,

    /// The display range expressed in UTF-16.
    pub display_text_range: Range,

    /// The valid display range expressed in UTF-16. After the end of the valid range, clients
    /// typically stop highlighting entities, etc.
    pub valid_text_range: Range,
}

impl TwitterTextParseResults {
    /// A new TwitterTextParseResults struct with all fields supplied as arguments.
    pub fn new(
        weighted_length: i32,
        permillage: i32,
        is_valid: bool,
        display_text_range: Range,
        valid_text_range: Range,
    ) -> TwitterTextParseResults {
        TwitterTextParseResults {
            weighted_length,
            permillage,
            is_valid,
            display_text_range,
            valid_text_range,
        }
    }

    /// An invalid TwitterTextParseResults struct. This function produces the return value when
    /// empty text or invalid UTF-8 is supplied to parse().
    pub fn empty() -> TwitterTextParseResults {
        TwitterTextParseResults {
            weighted_length: 0,
            permillage: 0,
            is_valid: false,
            display_text_range: Range::empty(),
            valid_text_range: Range::empty(),
        }
    }
}

/// Produce a [TwitterTextParseResults] struct from a [str]. If extract_urls is true, the weighted
/// length will give all URLs the weight supplied in [Configuration](twitter_text_configuration::Configuration),
/// regardless of their length.
///
/// This function uses the default parser backend (Nom). Use [parse_with_parser_backend] to
/// specify a different parsing strategy.
///
/// This function will allocate an NFC-normalized copy of the input string. If the text is already
/// NFC-normalized, [ValidatingExtractor::new_with_nfc_input] will be more efficient.
pub fn parse(text: &str, config: &Configuration, extract_urls: bool) -> TwitterTextParseResults {
    parse_with_parser_backend(text, config, extract_urls, ParserBackend::default())
}

/// Produce a [TwitterTextParseResults] struct from a [str] using the specified parser backend.
///
/// If extract_urls is true, the weighted length will give all URLs the weight supplied in
/// [Configuration](twitter_text_configuration::Configuration), regardless of their length.
///
/// The `parser_backend` parameter controls how TLDs are validated:
/// - [ParserBackend::Pest]: Trust the Pest grammar's TLD matching (original behavior)
/// - [ParserBackend::External]: Use phf lookup for O(1) TLD validation
/// - [ParserBackend::Nom]: Nom parser with external TLD/emoji validation (default, fastest)
///
/// This function will allocate an NFC-normalized copy of the input string. If the text is already
/// NFC-normalized, [ValidatingExtractor::new_with_nfc_input_and_parser_backend] will be more efficient.
pub fn parse_with_parser_backend(
    text: &str,
    config: &Configuration,
    extract_urls: bool,
    parser_backend: ParserBackend,
) -> TwitterTextParseResults {
    let mut extractor = ValidatingExtractor::with_parser_backend(config, parser_backend);
    let input = extractor.prep_input(text);
    if extract_urls {
        extractor
            .extract_urls_with_indices(input.as_str())
            .parse_results
    } else {
        extractor.extract_scan(input.as_str()).parse_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted_length_mixed_unicode_and_emoji() {
        // Test case from conformance suite that requires v2 config
        // Text: "H🐱☺👨‍👩‍👧‍👦"
        // Expected weighted_length: 16
        let config = twitter_text_config::config_v2();
        let text = "H🐱☺👨‍👩‍👧‍👦";
        let result = parse(text, config, false);

        assert_eq!(
            result.weighted_length, 16,
            "Mixed single/double byte Unicode and emoji family counting is incorrect"
        );
        assert!(result.is_valid);
        assert_eq!(result.permillage, 57);
    }

    #[test]
    fn test_weighted_length_emoji_with_skin_tone_modifiers() {
        // Test case from conformance suite that requires v2 config
        // Text: "🙋🏽👨‍🎤"
        // Expected weighted_length: 9
        let config = twitter_text_config::config_v2();
        let text = "🙋🏽👨‍🎤";
        let result = parse(text, config, false);

        assert_eq!(
            result.weighted_length, 9,
            "Emoji with skin tone modifiers counting is incorrect"
        );
        assert!(result.is_valid);
        assert_eq!(result.permillage, 32);
    }

    #[test]
    fn test_weighted_length_mixed_unicode_and_emoji_v3() {
        // Same test as above but with v3 config (emoji parsing enabled)
        // Text: "H🐱☺👨‍👩‍👧‍👦"
        // With v3 config, emoji families are counted as single units
        let config = twitter_text_config::config_v3();
        let text = "H🐱☺👨‍👩‍👧‍👦";
        let result = parse(text, config, false);

        assert_eq!(
            result.weighted_length, 7,
            "V3: Mixed single/double byte Unicode and emoji family counting is incorrect"
        );
        assert!(result.is_valid);
        assert_eq!(result.permillage, 25);
    }

    #[test]
    fn test_weighted_length_emoji_with_skin_tone_modifiers_v3() {
        // Same test as above but with v3 config (emoji parsing enabled)
        // Text: "🙋🏽👨‍🎤"
        // With v3 config, emojis with modifiers are counted as single units
        let config = twitter_text_config::config_v3();
        let text = "🙋🏽👨‍🎤";
        let result = parse(text, config, false);

        assert_eq!(
            result.weighted_length, 4,
            "V3: Emoji with skin tone modifiers counting is incorrect"
        );
        assert!(result.is_valid);
        assert_eq!(result.permillage, 14);
    }
}
