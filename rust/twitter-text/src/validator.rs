// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::extractor::{Extract, Extractor};
use crate::nom_parser::url::{parse_url, parse_url_without_protocol};
use crate::parse_with_external_validator;
use crate::ExternalValidator;
use twitter_text_config;

use pest::Parser;
use twitter_text_parser::twitter_text::Rule;
use twitter_text_parser::twitter_text::TwitterTextParser;

pub const MAX_TWEET_LENGTH: i32 = 280;

pub struct Validator {
    short_url_length: i32,
    short_url_length_https: i32,
    config: twitter_text_config::Configuration,
    extractor: Extractor,
    external_validator: ExternalValidator,
}

impl Validator {
    pub fn new() -> Validator {
        Validator::with_config(twitter_text_config::config_v1().clone())
    }

    pub fn with_config(config: twitter_text_config::Configuration) -> Validator {
        Validator {
            short_url_length: 23,
            short_url_length_https: 23,
            config,
            extractor: Extractor::new(),
            external_validator: ExternalValidator::default(),
        }
    }

    pub fn with_external_validator(external_validator: ExternalValidator) -> Validator {
        Validator {
            short_url_length: 23,
            short_url_length_https: 23,
            config: twitter_text_config::config_v1().clone(),
            extractor: Extractor::new(),
            external_validator,
        }
    }

    pub fn is_valid_tweet(&self, s: &str) -> bool {
        parse_with_external_validator(s, &self.config, false, self.external_validator).is_valid
    }

    pub fn is_valid_username(&self, s: &str) -> bool {
        // Username must start with @ and have 1-20 valid characters
        if !s.starts_with('@') && !s.starts_with('＠') {
            return false;
        }
        let mentions = self.extractor.extract_mentioned_screennames(s);
        // Valid if exactly one mention that spans the whole string (minus the @)
        mentions.len() == 1 && mentions[0].chars().count() == s.chars().count() - 1
    }

    pub fn is_valid_list(&self, s: &str) -> bool {
        // List must be @username/listname format
        if !s.starts_with('@') && !s.starts_with('＠') {
            return false;
        }
        let lists = self.extractor.extract_mentions_or_lists_with_indices(s);
        // Valid if exactly one list entity that spans the whole string
        if lists.len() != 1 {
            return false;
        }
        let list = &lists[0];
        !list.list_slug.is_empty() && list.end as usize == s.chars().count()
    }

    pub fn is_valid_hashtag(&self, s: &str) -> bool {
        // Hashtag must start with # and have valid hashtag text
        if !s.starts_with('#') && !s.starts_with('＃') {
            return false;
        }
        let hashtags = self.extractor.extract_hashtags(s);
        // Valid if exactly one hashtag that spans the whole string (minus the #)
        hashtags.len() == 1 && hashtags[0].chars().count() == s.chars().count() - 1
    }

    pub fn is_valid_url(&self, s: &str) -> bool {
        match self.external_validator {
            ExternalValidator::Nom => {
                // Use Nom parser
                match parse_url(s) {
                    Ok((remaining, _)) => remaining.is_empty(),
                    Err(_) => false,
                }
            }
            ExternalValidator::Pest | ExternalValidator::External => {
                // Use Pest parser for both Pest and External modes
                TwitterTextParser::parse(Rule::valid_url, s).is_ok()
            }
        }
    }

    pub fn is_valid_url_without_protocol(&self, s: &str) -> bool {
        match self.external_validator {
            ExternalValidator::Nom => {
                // Use Nom parser
                match parse_url_without_protocol(s) {
                    Ok((remaining, _)) => remaining.is_empty(),
                    Err(_) => false,
                }
            }
            ExternalValidator::Pest | ExternalValidator::External => {
                // Use Pest parser for both Pest and External modes
                TwitterTextParser::parse(Rule::url_without_protocol, s).is_ok()
            }
        }
    }

    pub fn get_max_tweet_length(&self) -> i32 {
        MAX_TWEET_LENGTH
    }

    pub fn get_short_url_length(&self) -> i32 {
        self.short_url_length
    }

    pub fn set_short_url_length(&mut self, short_url_length: i32) {
        self.short_url_length = short_url_length;
    }

    pub fn get_short_url_length_https(&self) -> i32 {
        self.short_url_length_https
    }

    pub fn set_short_url_length_https(&mut self, short_url_length_https: i32) {
        self.short_url_length_https = short_url_length_https;
    }
}
