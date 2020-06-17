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
use extractor::{Extract, Extractor, ValidatingExtractor};
use hit_highlighter::HitHighlighter;
use validator::Validator;
use cxx::{CxxVector, UniquePtr};
use std::path::PathBuf;

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
    #[derive(Copy, Clone)]
    pub struct Range {
        pub start: i32,
        pub end: i32,
    }

    #[derive(Copy, Clone)]
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

    pub struct Hit { 
        start: usize, 
        end: usize
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
        fn config_v1() -> UniquePtr<Configuration>;
        fn config_v2() -> UniquePtr<Configuration>;
        fn config_v3() -> UniquePtr<Configuration>;
        fn default_config() -> UniquePtr<Configuration>;
        fn get_config_weighted_ranges(config: &Configuration) -> Vec<WeightedRange>;
        fn configuration_from_path(path: &str) -> UniquePtr<Configuration>;
        fn configuration_from_json(json: &str) -> UniquePtr<Configuration>;

        // Autolinker
        fn autolink_default_config() -> UniquePtr<AutolinkerConfig>;
        fn autolink(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_usernames_and_lists(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_hashtags(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_urls(text: &str, config: &AutolinkerConfig) -> String;
        fn autolink_cashtags(text: &str, config: &AutolinkerConfig) -> String;

/*
        // Extractor
        type Extractor;
        fn make_extractor() -> Box<Extractor>;
        fn get_extract_url_without_protocol(e: &Extractor) -> bool;
        fn set_extract_url_without_protocol(e: &Extractor, extract_url_without_protocol: bool);
        fn extract_entities_with_indices(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_mentioned_screennames(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_mentioned_screennames_with_indices(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_mentions_or_lists_with_indices(e: &Extractor, text: &str)  -> Vec<Entity>;
        fn extract_reply_username(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_urls(e: &Extractor, text: &str) -> Vec<String>;
        fn extract_urls_with_indices(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_hashtags(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_hashtags_with_indices(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_cashtags(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_cashtags_with_indices(e: &Extractor, text: &str) -> Vec<Entity>;

        // ValidatingExtractor
        type ValidatingExtractor;
        fn make_validating_extractor() -> Box<ValidatingExtractor>;
        fn get_extract_url_without_protocol(e: &Extractor) -> bool;
        fn set_extract_url_without_protocol(e: &Extractor, extract_url_without_protocol: bool);
        fn extract_entities_with_indices(e: &Extractor, text: &str);
        fn extract_mentioned_screennames(e: &Extractor, text: &str);
        fn extract_mentioned_screennames_with_indices(e: &Extractor, text: &str);
        fn extract_mentions_or_lists_with_indices(e: &Extractor, text: &str);
        fn extract_reply_username(e: &Extractor, text: &str);
        fn extract_urls(e: &Extractor, text: &str) -> Vec<String>;
        fn extract_urls_with_indices(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_hashtags(e: &Extractor, text: &str) -> Vec<Entity>;
        fn extract_cashtags(e: &Extractor, text: &str) -> Vec<Entity>;
*/
        // HitHighlighter
        type HitHighlighter;
        fn make_highlighter(highlight_tag: &str) -> Box<HitHighlighter>;
        fn make_default_highlighter() -> Box<HitHighlighter>;
        fn hit_highlight(hh: &HitHighlighter, text: &str, hits: &CxxVector<Hit>) -> String;

        // Validator
        type Validator;
        fn make_default_validator() -> Box<Validator>;
        fn is_valid_tweet(validator: &Validator, s: &str) -> bool;
        fn is_valid_username(validator: &Validator, s: &str) -> bool;
        fn is_valid_list(validator: &Validator, s: &str) -> bool;
        fn is_valid_hashtag(validator: &Validator, s: &str) -> bool;
        fn is_valid_url(validator: &Validator, s: &str) -> bool;
        fn is_valid_url_without_protocol(validator: &Validator, s: &str) -> bool;
        fn get_max_tweet_length() -> i32;
        fn get_short_url_length(validator: &Validator) -> i32;
        fn set_short_url_length(validator: &mut Validator, short_url_length: i32);
        fn get_short_url_length_https(validator: &Validator) -> i32;
        fn set_short_url_length_https(validator: &mut Validator, short_url_length_https: i32);

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

pub fn config_v1() -> UniquePtr<ffi::Configuration> {
    UniquePtr::new(ffi::Configuration::from(twitter_text_config::config_v1()))
}

pub fn config_v2() -> UniquePtr<ffi::Configuration> {
    UniquePtr::new(ffi::Configuration::from(twitter_text_config::config_v2()))
}

pub fn config_v3() -> UniquePtr<ffi::Configuration> {
    UniquePtr::new(ffi::Configuration::from(twitter_text_config::config_v3()))
}

pub fn default_config() -> UniquePtr<ffi::Configuration> {
    UniquePtr::new(ffi::Configuration::from(twitter_text_config::default()))
}

pub fn get_config_weighted_ranges(config: &ffi::Configuration) -> Vec<ffi::WeightedRange> {
    config.ranges.to_vec()
}

pub fn configuration_from_path(path: &str) -> UniquePtr<ffi::Configuration> {
    let pathbuf = PathBuf::from(path);
    UniquePtr::new(ffi::Configuration::from(&Configuration::configuration_from_path(&pathbuf)))
}

pub fn configuration_from_json(json: &str) -> UniquePtr<ffi::Configuration> {
    UniquePtr::new(ffi::Configuration::from(&Configuration::configuration_from_json(json)))
}

pub fn autolink_default_config() -> UniquePtr<ffi::AutolinkerConfig> {
    UniquePtr::new(Autolinker::default_config())
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

// Extractor
pub fn make_extractor() -> Box<Extractor> {
    Box::new(Extractor::new())
}

// HitHighlighter
pub fn make_default_highlighter() -> Box<HitHighlighter> {
    Box::new(HitHighlighter::new())
}

pub fn make_highlighter(highlight_tag: &str) -> Box<HitHighlighter> {
    Box::new(HitHighlighter::new_with_tag(highlight_tag))
}

pub fn hit_highlight(hh: &HitHighlighter, text: &str, hits: &CxxVector<ffi::Hit>) -> String {
    let mut rust_hits: Vec<(usize, usize)> = Vec::with_capacity(hits.len());
    for hit in hits {
        rust_hits.push((hit.start, hit.end));
    }
    hh.highlight(text, rust_hits)
}

// Validator
pub fn make_default_validator() -> Box<Validator> {
    Box::new(Validator::new())
}

pub fn is_valid_tweet(validator: &Validator, s: &str) -> bool {
    validator.is_valid_tweet(s)
}

pub fn is_valid_username(validator: &Validator, s: &str) -> bool {
    validator.is_valid_username(s)
}

pub fn is_valid_list(validator: &Validator, s: &str) -> bool {
    validator.is_valid_list(s)
}

pub fn is_valid_hashtag(validator: &Validator, s: &str) -> bool {
    validator.is_valid_hashtag(s)
}

pub fn is_valid_url(validator: &Validator, s: &str) -> bool {
    validator.is_valid_url(s)
}

pub fn is_valid_url_without_protocol(validator: &Validator, s: &str) -> bool {
    validator.is_valid_url_without_protocol(s)
}

pub fn get_max_tweet_length() -> i32 { validator::MAX_TWEET_LENGTH }

pub fn get_short_url_length(validator: &Validator) -> i32 {
    validator.get_short_url_length()
}

pub fn set_short_url_length(validator: &mut Validator, short_url_length: i32) {
    validator.set_short_url_length(short_url_length);
}

pub fn get_short_url_length_https(validator: &Validator) -> i32 {
    validator.get_short_url_length_https()
}

pub fn set_short_url_length_https(validator: &mut Validator, short_url_length_https: i32) {
    validator.set_short_url_length_https(short_url_length_https);
}

pub fn parse_ffi(text: &str, config: &ffi::Configuration, extract_urls: bool) -> ffi::TwitterTextParseResults {
    ffi::TwitterTextParseResults::from(parse(text, &ffi::Configuration::to(config), extract_urls))
}