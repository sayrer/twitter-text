// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

pub mod extractor;
pub mod hit_highlighter;
pub mod autolinker;
pub mod entity;
pub mod validator;

use twitter_text_config::Configuration;
use twitter_text_config::Range;
use twitter_text_config::WeightedRange;
use autolinker::Autolinker;
use extractor::Extract;
use extractor::ValidatingExtractor;
use hit_highlighter::HitHighlighter;

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
    pub valid_text_range: Range
}

impl TwitterTextParseResults {
    /// A new TwitterTextParseResults struct with all fields supplied as arguments.
    pub fn new(weighted_length: i32,
               permillage: i32,
               is_valid: bool,
               display_text_range: Range,
               valid_text_range: Range) -> TwitterTextParseResults {
        TwitterTextParseResults {
            weighted_length,
            permillage,
            is_valid,
            display_text_range,
            valid_text_range
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
            valid_text_range: Range::empty()
        }
    }
}

/// Produce a [TwitterTextParseResults] struct from a [str]. If extract_urls is true, the weighted
/// length will give all URLs the weight supplied in [Configuration](twitter_text_configuration::Configuration),
/// regardless of their length.
/// 
/// This function will allocate an NFC-normalized copy of the input string. If the text is already
/// NFC-normalized, [ValidatingExtractor::new_with_nfc_input] will be more efficient.
pub fn parse(text: &str, config: &Configuration, extract_urls: bool) -> TwitterTextParseResults {
    let mut extractor = ValidatingExtractor::new(config);
    let input = extractor.prep_input(text);
    if extract_urls {
        extractor.extract_urls_with_indices(input.as_str()).parse_results
    } else {
        extractor.extract_scan(input.as_str()).parse_results
    }
}

#[cxx::bridge(namespace = twitter_text_ffi)]
pub mod ffi {
    pub struct Range {
        pub start: i32,
        pub end: i32,
    }

    pub struct WeightedRange {
        pub range: Range,
        pub weight: i32
    }

    pub struct Configuration {
        pub version: i32,
        pub max_weighted_tweet_length: i32,
        pub scale: i32,
        pub default_weight: i32,
        pub transformed_url_length: i32,
        pub ranges: Vec<WeightedRange>,
        pub emoji_parsing_enabled: bool,
    }

    pub struct AutolinkerConfig {
        pub no_follow: bool,
        pub url_class: String,
        pub url_target: String,
        pub symbol_tag: String,
        pub text_with_symbol_tag: String,
        pub list_class: String,
        pub username_class: String,
        pub hashtag_class: String,
        pub cashtag_class: String,
        pub username_url_base: String,
        pub list_url_base: String,
        pub hashtag_url_base: String,
        pub cashtag_url_base: String,
        pub invisible_tag_attrs: String,
        pub username_include_symbol: bool,
    }

    pub struct HitHighlighter {
        pub highlight_tag: String,
    }

    pub struct TwitterTextParseResults {
        pub weighted_length: i32,
        pub permillage: i32,
        pub is_valid: bool,
        pub display_text_range: Range,
        pub valid_text_range: Range
    }

    extern "C" {
        include!("cxx.h");
    }

    extern "Rust" {
        // Configuration
        fn config_v1() -> Configuration;
        fn config_v2() -> Configuration;
        fn config_v3() -> Configuration;
        fn default_config() -> Configuration;

        // Autolinker
        fn autolink_default_config() -> AutolinkerConfig;
        fn autolink(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_usernames_and_lists(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_hashtags(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_urls(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_cashtags(text: &str, config: &AutolinkerConfig) -> String;

        // HitHighlighter
        fn hit_highlight(text: &str, config: &HitHighlighter) -> String;

        fn parse_ffi(text: &str, config: &Configuration, extract_urls: bool) -> TwitterTextParseResults;
    }
}

impl ffi::TwitterTextParseResults {
    fn from(results: TwitterTextParseResults) -> ffi::TwitterTextParseResults {
        ffi::TwitterTextParseResults {
            weighted_length: results.weighted_length,
            permillage: results.permillage,
            is_valid: results.is_valid,
            display_text_range: ffi::Range::from(&results.display_text_range),
            valid_text_range: ffi::Range::from(&results.valid_text_range),
        }
    }
}

impl ffi::Range {
    fn from(r: &Range) -> ffi::Range {
        ffi::Range {
            start: r.start(),
            end: r.end(),
        }
    }

    fn to(r: &ffi::Range) -> Range {
        Range::new(r.start, r.end)
    }
}


impl ffi::WeightedRange {
    fn from(wr: &WeightedRange) -> ffi::WeightedRange {
        ffi::WeightedRange {
            range: ffi::Range::from(&wr.range),
            weight: wr.weight,
        }
    }

    fn to(wr: &ffi::WeightedRange) -> WeightedRange {
        WeightedRange {
            range: ffi::Range::to(&wr.range),
            weight: wr.weight,
        }
    }
}

impl ffi::Configuration {
    pub fn from(config: &Configuration) -> ffi::Configuration {
        ffi::Configuration {
            version: config.version,
            max_weighted_tweet_length: config.max_weighted_tweet_length,
            scale: config.scale,
            default_weight: config.default_weight,
            transformed_url_length: config.transformed_url_length,
            ranges: config.ranges.iter().map(|r| { ffi::WeightedRange::from(r) }).collect(),
            emoji_parsing_enabled: config.emoji_parsing_enabled
        }
    }

    pub fn to(config: &ffi::Configuration) -> Configuration {
        let ranges = &*config.ranges;
        Configuration {
            version: config.version,
            max_weighted_tweet_length: config.max_weighted_tweet_length,
            scale: config.scale,
            default_weight: config.default_weight,
            transformed_url_length: config.transformed_url_length,
            ranges: ranges.iter().map(|wr| { ffi::WeightedRange::to(&wr) }).collect(),
            emoji_parsing_enabled: config.emoji_parsing_enabled
        }
    }
}

pub fn config_v1() -> ffi::Configuration {
    ffi::Configuration::from(twitter_text_config::config_v1())
}

pub fn config_v2() -> ffi::Configuration {
    ffi::Configuration::from(twitter_text_config::config_v2())
}

pub fn config_v3() -> ffi::Configuration {
    ffi::Configuration::from(twitter_text_config::config_v3())
}

pub fn default_config() -> ffi::Configuration {
    ffi::Configuration::from(twitter_text_config::default())
}

// Autolinker
pub fn autolink_default_config() -> ffi::AutolinkerConfig {
    Autolinker::default_config()
}

pub fn autolink(text: &str, config: &ffi::AutolinkerConfig) -> String {
    Autolinker::new_with_config(config).autolink(text)
}

pub fn autolink_usernames_and_lists(text: &str, config: &ffi::AutolinkerConfig) -> String {
    Autolinker::new_with_config(config).autolink_usernames_and_lists(text)
}

pub fn autolink_hashtags(text: &str, config: &ffi::AutolinkerConfig) -> String {
    Autolinker::new_with_config(config).autolink_hashtags(text)
}

pub fn autolink_urls(text: &str, config: &ffi::AutolinkerConfig) -> String {
    Autolinker::new_with_config(config).autolink_urls(text)
}

pub fn autolink_cashtags(text: &str, config: &ffi::AutolinkerConfig) -> String {
    Autolinker::new_with_config(config).autolink_cashtags(text)
}

pub fn hit_highlight(text: &str, config: &ffi::HitHighlighter) -> String {
    let highlighter = HitHighlighter {
        highlight_tag: &config.highlight_tag,
    };
    // TODO
    highlighter.highlight(text, vec![])
}

pub fn parse_ffi(text: &str, config: &ffi::Configuration, extract_urls: bool) -> ffi::TwitterTextParseResults {
    ffi::TwitterTextParseResults::from(parse(text, &ffi::Configuration::to(config), extract_urls))
}