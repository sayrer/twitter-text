// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::entity::{Entity, Type};
use crate::tlds::is_valid_tld;
use crate::TwitterTextParseResults;
use idna::uts46::{AsciiDenyList, DnsLength, Hyphens, Uts46};
use pest::Parser;
use std::iter::Peekable;
use std::str::CharIndices;
use twitter_text_config::Configuration;
use twitter_text_config::Range;
use twitter_text_parser::twitter_text::Rule;
use twitter_text_parser::twitter_text::TwitterTextParser;
// Full TLD parser for ExternalValidator::Pest mode
use twitter_text_parser::twitter_text::full_tld::Rule as FullTldRule;
use twitter_text_parser::twitter_text::full_tld::TwitterTextFullTldParser;
use unicode_normalization::{is_nfc, UnicodeNormalization};

type RuleMatch = fn(Rule) -> bool;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type FullTldPair<'a> = pest::iterators::Pair<'a, FullTldRule>;

/// Selects the TLD matching strategy for URL extraction.
///
/// This enum allows choosing between different parsing backends as described
/// in PARSER_BACKENDS.md. The choice affects how TLDs are validated during
/// URL extraction.
///
/// # Current Implementation
///
/// The current Pest grammar uses a permissive "domain-like" pattern for TLDs,
/// so `ExternalValidator::External` (the default) is required for correct TLD validation.
/// `ExternalValidator::Pest` trusts whatever the grammar matches, which may include
/// invalid TLDs with the current permissive grammar.
///
/// To use `ExternalValidator::Pest` correctly, the grammar would need to be restored
/// to include the full TLD alternation (the original ~1500 TLD list).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExternalValidator {
    /// Pure Pest parsing - trusts the Pest grammar's TLD matching.
    ///
    /// **Note:** With the current permissive grammar, this will accept any
    /// domain-like pattern. Use `External` for correct TLD validation.
    Pest,

    /// Pest for structure, external TLD lookup via phf perfect hash.
    /// The Pest grammar matches a permissive "domain-like" pattern, then
    /// Rust code validates the TLD using O(1) phf lookup. This is faster
    /// and more correct with the current grammar.
    #[default]
    External,
}

/**
 * A common Trait implemented by the two Extractors, [Extractor] and [ValidatingExtractor].
 */
pub trait Extract<'a> {
    /// The result type returned from the various extract methods.
    type T;

    /// The result type returned from the various mention extract methods.
    type Mention;

    /// Get whether the extractor will detect URLs without schemes, such as "example.com".
    fn get_extract_url_without_protocol(&self) -> bool;

    /// Set whether the extractor will detect URLs without schemes, such as "example.com".
    fn set_extract_url_without_protocol(&mut self, extract_url_without_protocol: bool);

    /// Get the TLD matching strategy used by this extractor.
    fn get_external_validator(&self) -> ExternalValidator;

    /// Extract entities from the source text that match rules allowed by r_match.
    fn extract(&self, s: &'a str, r_match: RuleMatch) -> Self::T;

    /// Create the result type. The concrete type varies by implementation.
    fn create_result(
        &self,
        s: &'a str,
        entity_count: usize,
        pairs: &mut Vec<UnprocessedEntity<'a>>,
    ) -> Self::T;

    /// Create the mention result type. The concrete type varies by implementation.
    fn extract_reply_username(&self, s: &'a str) -> Self::Mention;

    /// Create a mention result type from a pest::Pair.
    fn mention_result(&self, s: &'a str, pairs: Option<Pair<'a>>) -> Self::Mention;

    /// Returns an empty result. Used when the input is invalid.
    fn empty_result(&self) -> Self::T;

    fn extract_impl(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        if s.is_empty() {
            return self.empty_result();
        }

        let external_validator = self.get_external_validator();

        // Branch based on TLD matcher to use the appropriate parser
        match external_validator {
            ExternalValidator::Pest => self.extract_impl_full_tld(s, r_match),
            ExternalValidator::External => self.extract_impl_external(s, r_match),
        }
    }

    /// Implementation using the permissive grammar with external TLD validation via phf.
    fn extract_impl_external(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        match TwitterTextParser::parse(Rule::tweet, s) {
            Ok(p) => {
                let mut scanned = Vec::new();
                let mut entity_count = 0;

                p.flatten().for_each(|pair| {
                    let r = pair.as_rule();
                    if r == Rule::invalid_char || r == Rule::emoji {
                        scanned.push(UnprocessedEntity::Pair(pair));
                    } else if r_match(r) {
                        if r == Rule::url || r == Rule::url_without_protocol {
                            let span = pair.as_span();
                            let requires_exact_tld = r == Rule::url_without_protocol;
                            if let Some(trim_bytes) =
                                validate_url(pair, requires_exact_tld, ExternalValidator::External)
                            {
                                entity_count += 1;
                                // If TLD was shorter than parsed, create trimmed span
                                let final_span = if trim_bytes > 0 {
                                    pest::Span::new(
                                        span.get_input(),
                                        span.start(),
                                        span.end() - trim_bytes,
                                    )
                                    .unwrap_or(span)
                                } else {
                                    span
                                };
                                scanned.push(UnprocessedEntity::UrlSpan(final_span));
                            }
                        } else {
                            entity_count += 1;
                            scanned.push(UnprocessedEntity::Pair(pair));
                        }
                    }
                });
                // Reverse so we can pop from the end in document order
                scanned.reverse();
                self.create_result(s, entity_count, &mut scanned)
            }
            Err(_e) => self.empty_result(),
        }
    }

    /// Implementation using the full TLD grammar - Pest handles TLD validation.
    fn extract_impl_full_tld(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        // Convert the RuleMatch function to work with FullTldRule
        let full_tld_r_match = convert_rule_match(r_match);

        match TwitterTextFullTldParser::parse(FullTldRule::tweet, s) {
            Ok(p) => {
                let mut scanned = Vec::new();
                let mut entity_count = 0;

                p.flatten().for_each(|pair| {
                    let r = pair.as_rule();
                    if r == FullTldRule::invalid_char || r == FullTldRule::emoji {
                        // Convert FullTldPair to regular Pair by re-parsing with regular parser
                        // We store the span and will create the entity from it
                        scanned.push(UnprocessedEntity::FullTldPair(pair));
                    } else if full_tld_r_match(r) {
                        if r == FullTldRule::url || r == FullTldRule::url_without_protocol {
                            let span = pair.as_span();
                            // With full TLD grammar, Pest already validated the TLD
                            // We only need to do punycode validation
                            if validate_url_full_tld(&pair) {
                                entity_count += 1;
                                scanned.push(UnprocessedEntity::UrlSpan(span));
                            }
                        } else {
                            entity_count += 1;
                            scanned.push(UnprocessedEntity::FullTldPair(pair));
                        }
                    }
                });
                // Reverse so we can pop from the end in document order
                scanned.reverse();
                self.create_result(s, entity_count, &mut scanned)
            }
            Err(_e) => self.empty_result(),
        }
    }

    /// Extract all URLs from the text, subject to value returned by [Extract::get_extract_url_without_protocol].
    fn extract_urls_with_indices(&self, s: &'a str) -> Self::T {
        if self.get_extract_url_without_protocol() {
            self.extract(s, |r| r == Rule::url || r == Rule::url_without_protocol)
        } else {
            self.extract(s, |r| r == Rule::url)
        }
    }

    /// Extract all Hashtags from the text
    fn extract_hashtags_with_indices(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| r == Rule::hashtag)
    }

    /// Extract all Cashtags from the text
    fn extract_cashtags_with_indices(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| r == Rule::cashtag)
    }

    /// Extract all usernames from the text.
    fn extract_mentioned_screennames_with_indices(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| r == Rule::username)
    }

    /// Extract all usernames and lists from the text.
    fn extract_mentions_or_lists_with_indices(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| r == Rule::username || r == Rule::list)
    }

    /// Extract a "reply"--a username that appears at the beginning of a tweet.
    fn extract_reply_username_impl(&self, s: &'a str) -> Self::Mention {
        match TwitterTextParser::parse(Rule::reply, s) {
            Ok(pairs) => {
                for pair in pairs.flatten() {
                    return self.mention_result(s, Some(pair));
                }

                return self.mention_result(s, None);
            }
            Err(_) => self.mention_result(s, None),
        }
    }

    /// Extract all entities from the text (Usernames, Lists, Hashtags, Cashtags, and URLs).
    fn extract_entities_with_indices(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| {
            r == Rule::url
                || r == Rule::hashtag
                || r == Rule::cashtag
                || r == Rule::list
                || r == Rule::username
        })
    }

    /// Parse the text without extracting any entities.
    fn extract_scan(&self, s: &'a str) -> Self::T {
        self.extract(s, |_r| false)
    }

    fn entity_from_pair(
        &self,
        ue: UnprocessedEntity<'a>,
        start: i32,
        end: i32,
    ) -> Option<Entity<'a>> {
        match ue {
            UnprocessedEntity::UrlSpan(url) => {
                Some(Entity::new(Type::URL, url.as_str(), start, end))
            }
            UnprocessedEntity::Pair(pair) => {
                let s = pair.as_str();
                match pair.as_rule() {
                    Rule::hashtag => Some(Entity::new(
                        Type::HASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    Rule::cashtag => Some(Entity::new(
                        Type::CASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    Rule::username => Some(Entity::new(
                        Type::MENTION,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    Rule::list => {
                        let mut list_iter = pair.into_inner();
                        let listname = list_iter.find(|p| p.as_rule() == Rule::listname);
                        let list_slug = list_iter.find(|p| p.as_rule() == Rule::list_slug);
                        match (listname, list_slug) {
                            (Some(ln), Some(ls)) => {
                                let name = ln.as_str();
                                Some(Entity::new_list(
                                    Type::MENTION,
                                    &name[calculate_offset(name)..],
                                    &ls.as_str(),
                                    start,
                                    end,
                                ))
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            UnprocessedEntity::FullTldPair(pair) => {
                let s = pair.as_str();
                match pair.as_rule() {
                    FullTldRule::hashtag => Some(Entity::new(
                        Type::HASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    FullTldRule::cashtag => Some(Entity::new(
                        Type::CASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    FullTldRule::username => Some(Entity::new(
                        Type::MENTION,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    FullTldRule::list => {
                        let mut list_iter = pair.into_inner();
                        let listname = list_iter.find(|p| p.as_rule() == FullTldRule::listname);
                        let list_slug = list_iter.find(|p| p.as_rule() == FullTldRule::list_slug);
                        match (listname, list_slug) {
                            (Some(ln), Some(ls)) => {
                                let name = ln.as_str();
                                Some(Entity::new_list(
                                    Type::MENTION,
                                    &name[calculate_offset(name)..],
                                    &ls.as_str(),
                                    start,
                                    end,
                                ))
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
        }
    }
}

/**
 * An [Extract] implementation that does no validation (length checks, validity, etc).
 */
pub struct Extractor {
    extract_url_without_protocol: bool,
    external_validator: ExternalValidator,
}

impl Extractor {
    /// Create a new extractor with the default TLD matcher (External/phf).
    pub fn new() -> Extractor {
        Extractor {
            extract_url_without_protocol: true,
            external_validator: ExternalValidator::default(),
        }
    }

    /// Create a new extractor with the specified TLD matcher.
    pub fn with_external_validator(external_validator: ExternalValidator) -> Extractor {
        Extractor {
            extract_url_without_protocol: true,
            external_validator,
        }
    }

    /// Extract a vector of URLs as [String] objects.
    pub fn extract_urls(&self, s: &str) -> Vec<String> {
        self.extract_urls_with_indices(s)
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
    }

    /// Extract a vector of Hashtags as [String] objects.
    pub fn extract_hashtags(&self, s: &str) -> Vec<String> {
        self.extract_hashtags_with_indices(s)
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
    }

    /// Extract a vector of Cashtags as [String] objects.
    pub fn extract_cashtags(&self, s: &str) -> Vec<String> {
        self.extract_cashtags_with_indices(s)
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
    }

    /// Extract all usernames from the text. The same
    /// as [Extract::extract_mentioned_screennames_with_indices], but included for compatibility.
    pub fn extract_mentioned_screennames(&self, s: &str) -> Vec<String> {
        self.extract_mentioned_screennames_with_indices(s)
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
    }

    // Internal UTF-8 to UTF-16 offset calculation.
    fn scan(&self, iter: &mut Peekable<CharIndices>, limit: usize) -> i32 {
        let mut offset: i32 = 0;

        loop {
            if let Some((peeked_pos, _c)) = iter.peek() {
                if *peeked_pos >= limit {
                    break;
                }
            } else {
                break;
            }

            if let Some((_, c)) = iter.next() {
                offset += c.len_utf16() as i32; // count UTF-16 code units
            }
        }

        offset
    }
}

impl<'a> Extract<'a> for Extractor {
    /// [Extractor] returns a vector of entities with no validation data.
    type T = Vec<Entity<'a>>;

    /// [Extractor] returns a single mention entity with no validation data.
    type Mention = Option<Entity<'a>>;

    fn get_extract_url_without_protocol(&self) -> bool {
        self.extract_url_without_protocol
    }

    fn set_extract_url_without_protocol(&mut self, extract_url_without_protocol: bool) {
        self.extract_url_without_protocol = extract_url_without_protocol;
    }

    fn get_external_validator(&self) -> ExternalValidator {
        self.external_validator
    }

    fn extract(&self, s: &'a str, r_match: RuleMatch) -> Vec<Entity<'a>> {
        self.extract_impl(s, r_match)
    }

    fn create_result(
        &self,
        s: &'a str,
        count: usize,
        scanned: &mut Vec<UnprocessedEntity<'a>>,
    ) -> Vec<Entity<'a>> {
        let mut entities = Vec::with_capacity(count);
        let mut iter = s.char_indices().peekable();
        let mut start_index = 0;

        while let Some(entity) = scanned.pop() {
            start_index += self.scan(iter.by_ref(), entity.start());
            let end_index = start_index + self.scan(iter.by_ref(), entity.end());
            if let Some(e) = self.entity_from_pair(entity, start_index, end_index) {
                entities.push(e);
            }
            start_index = end_index;
        }

        entities
    }

    fn extract_reply_username(&self, s: &'a str) -> Option<Entity<'a>> {
        self.extract_reply_username_impl(s)
    }

    fn mention_result(&self, s: &'a str, entity: Option<Pair<'a>>) -> Option<Entity<'a>> {
        match entity {
            Some(e) => {
                let mut v = Vec::new();
                v.push(UnprocessedEntity::Pair(e));
                self.create_result(s, 1, &mut v).pop()
            }
            None => None,
        }
    }

    fn empty_result(&self) -> Vec<Entity<'a>> {
        Vec::new()
    }
}

/**
 * An [Extract] implementation that extracts entities and provides [TwitterTextParseResults] validation data.
 */
pub struct ValidatingExtractor<'a> {
    extract_url_without_protocol: bool,
    external_validator: ExternalValidator,
    config: &'a Configuration,
    ld: LengthData,
}

impl<'a> ValidatingExtractor<'a> {
    /// Create a new Extractor with the default TLD matcher (External/phf).
    /// [ValidatingExtractor::prep_input] must be called prior to extract.
    pub fn new(configuration: &Configuration) -> ValidatingExtractor<'_> {
        ValidatingExtractor {
            extract_url_without_protocol: true,
            external_validator: ExternalValidator::default(),
            config: configuration,
            ld: LengthData::empty(),
        }
    }

    /// Create a new Extractor with the specified TLD matcher.
    /// [ValidatingExtractor::prep_input] must be called prior to extract.
    pub fn with_external_validator(
        configuration: &Configuration,
        external_validator: ExternalValidator,
    ) -> ValidatingExtractor<'_> {
        ValidatingExtractor {
            extract_url_without_protocol: true,
            external_validator,
            config: configuration,
            ld: LengthData::empty(),
        }
    }

    /// Initialize the [ValidatingExtractor] text length data.
    pub fn prep_input(&mut self, s: &str) -> String {
        // Avoid allocation if already NFC-normalized
        let nfc: String = if is_nfc(s) {
            s.to_string()
        } else {
            s.nfc().collect()
        };
        let (nfc_length, nfc_length_utf8) = calculate_length(nfc.as_str());
        let (original_length, original_length_utf8) = calculate_length(s);
        self.ld = LengthData {
            normalized_length: nfc_length,
            normalized_length_utf8: nfc_length_utf8,
            original_length,
            original_length_utf8,
        };
        nfc
    }

    /// Create a new Extractor from text that is already nfc-normalized. There's no need to call
    /// [ValidatingExtractor::prep_input] for this text.
    pub fn new_with_nfc_input(
        configuration: &'a Configuration,
        s: &str,
    ) -> ValidatingExtractor<'a> {
        let (length, length_utf8) = calculate_length(s);
        ValidatingExtractor {
            extract_url_without_protocol: true,
            external_validator: ExternalValidator::default(),
            config: configuration,
            ld: LengthData {
                normalized_length: length,
                normalized_length_utf8: length_utf8,
                original_length: length,
                original_length_utf8: length_utf8,
            },
        }
    }

    /// Create a new Extractor from text that is already nfc-normalized with the specified TLD matcher.
    pub fn new_with_nfc_input_and_external_validator(
        configuration: &'a Configuration,
        s: &str,
        external_validator: ExternalValidator,
    ) -> ValidatingExtractor<'a> {
        let (length, length_utf8) = calculate_length(s);
        ValidatingExtractor {
            extract_url_without_protocol: true,
            external_validator,
            config: configuration,
            ld: LengthData {
                normalized_length: length,
                normalized_length_utf8: length_utf8,
                original_length: length,
                original_length_utf8: length_utf8,
            },
        }
    }
}

fn calculate_length(text: &str) -> (i32, i32) {
    let mut length: i32 = 0;
    let mut length_utf8: i32 = 0;
    for c in text.chars() {
        length += as_i32(c.len_utf16());
        length_utf8 += 1;
    }
    (length, length_utf8)
}

impl<'a> Extract<'a> for ValidatingExtractor<'a> {
    type T = ExtractResult<'a>;
    type Mention = MentionResult<'a>;

    fn get_extract_url_without_protocol(&self) -> bool {
        self.extract_url_without_protocol
    }

    fn set_extract_url_without_protocol(&mut self, extract_url_without_protocol: bool) {
        self.extract_url_without_protocol = extract_url_without_protocol;
    }

    fn get_external_validator(&self) -> ExternalValidator {
        self.external_validator
    }

    fn extract(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        self.extract_impl(s, r_match)
    }

    fn create_result(
        &self,
        s: &'a str,
        count: usize,
        scanned: &mut Vec<UnprocessedEntity<'a>>,
    ) -> ExtractResult<'a> {
        let mut iter = s.char_indices().peekable();
        let mut metrics = TextMetrics::new(self.config, self.ld.normalized_length);
        let mut entities = Vec::with_capacity(count);
        let mut start_index = 0;
        while let Some(entity) = scanned.pop() {
            start_index += metrics.scan(iter.by_ref(), entity.start(), TrackAction::Text);
            let r = entity.as_rule();
            if r == Rule::invalid_char {
                metrics.is_valid = false;
            } else if r == Rule::emoji && self.config.emoji_parsing_enabled {
                metrics.weighted_count += self.config.default_weight;
                start_index += metrics.scan(iter.by_ref(), entity.end(), TrackAction::Emoji);
            } else {
                let action = if r == Rule::url {
                    TrackAction::Url
                } else {
                    TrackAction::Text
                };
                let end_index = start_index + metrics.scan(iter.by_ref(), entity.end(), action);
                if let Some(e) = self.entity_from_pair(entity, start_index, end_index) {
                    entities.push(e);
                }
                start_index = end_index;
            }
        }

        metrics.scan(iter.by_ref(), s.len(), TrackAction::Text);

        let normalized_tweet_offset: i32 = self.ld.original_length - self.ld.normalized_length;
        let scaled_weighted_length = metrics.weighted_count / self.config.scale;
        let is_valid =
            metrics.is_valid && scaled_weighted_length <= self.config.max_weighted_tweet_length;
        let permillage = scaled_weighted_length * 1000 / self.config.max_weighted_tweet_length;

        let results = TwitterTextParseResults::new(
            scaled_weighted_length,
            permillage,
            is_valid,
            Range::new(0, metrics.offset + normalized_tweet_offset - 1),
            Range::new(0, metrics.valid_offset + normalized_tweet_offset - 1),
        );

        ExtractResult::new(results, entities)
    }

    fn extract_reply_username(&self, s: &'a str) -> MentionResult<'a> {
        self.extract_reply_username_impl(s)
    }

    fn mention_result(&self, s: &'a str, entity: Option<Pair<'a>>) -> MentionResult<'a> {
        match entity {
            Some(_e) => {
                let results = self.extract_entities_with_indices(s);
                MentionResult::new(results.parse_results, Some(results.entities[0].clone()))
            }
            None => MentionResult::new(TwitterTextParseResults::empty(), None),
        }
    }

    fn empty_result(&self) -> ExtractResult<'a> {
        ExtractResult::new(TwitterTextParseResults::empty(), Vec::new())
    }
}

/// Entities and validation data returned by [ValidatingExtractor].
pub struct ExtractResult<'a> {
    pub parse_results: TwitterTextParseResults,
    pub entities: Vec<Entity<'a>>,
}

impl<'a> ExtractResult<'a> {
    pub fn new(results: TwitterTextParseResults, e: Vec<Entity<'a>>) -> ExtractResult<'a> {
        ExtractResult {
            parse_results: results,
            entities: e,
        }
    }
}

/// A mention entity and validation data returned by [ValidatingExtractor].
#[derive(Debug)]
pub struct MentionResult<'a> {
    pub parse_results: TwitterTextParseResults,
    pub mention: Option<Entity<'a>>,
}

impl<'a> MentionResult<'a> {
    pub fn new(results: TwitterTextParseResults, e: Option<Entity<'a>>) -> MentionResult<'a> {
        MentionResult {
            parse_results: results,
            mention: e,
        }
    }
}

// Tracks validation data during entity extraction.
struct TextMetrics<'a> {
    is_valid: bool,
    weighted_count: i32,
    offset: i32,
    valid_offset: i32,
    normalized_length: i32,
    scaled_max_weighted_tweet_length: i32,
    config: &'a Configuration,
}

impl<'a> TextMetrics<'a> {
    fn new(config: &Configuration, normalized_length: i32) -> TextMetrics<'_> {
        TextMetrics {
            is_valid: true,
            weighted_count: 0,
            offset: 0,
            valid_offset: 0,
            normalized_length,
            scaled_max_weighted_tweet_length: config.max_weighted_tweet_length * config.scale,
            config,
        }
    }

    fn add_char(&mut self, c: char) {
        let len_utf16: i32 = as_i32(c.len_utf16());
        self.add_offset(len_utf16);
    }

    fn add_offset(&mut self, offset: i32) {
        self.offset += offset;
        if self.is_valid && self.weighted_count <= self.scaled_max_weighted_tweet_length {
            self.valid_offset += offset;
        }
    }

    fn track_emoji(&mut self, c: char) {
        self.add_char(c);
    }

    fn track_url(&mut self, count: i32) {
        self.weighted_count += self.config.transformed_url_length * self.config.scale;
        self.add_offset(count);
    }

    fn track_text(&mut self, c: char) {
        if self.offset < self.normalized_length {
            let code_point: i32 = c as i32;
            let mut char_weight = self.config.default_weight;
            for (_, range) in self.config.ranges.iter().enumerate() {
                if range.contains(code_point) {
                    char_weight = range.weight;
                    break;
                }
            }
            self.weighted_count += char_weight;
            self.add_char(c);
        }
    }

    fn scan(&mut self, iter: &mut Peekable<CharIndices>, limit: usize, action: TrackAction) -> i32 {
        let mut offset: i32 = 0;

        loop {
            if let Some((peeked_pos, _c)) = iter.peek() {
                if *peeked_pos >= limit {
                    break;
                }
            } else {
                break;
            }

            if let Some((_pos, c)) = iter.next() {
                let len_utf16 = as_i32(c.len_utf16());
                offset += len_utf16; // use UTF-16 length here too
                match action {
                    TrackAction::Text => self.track_text(c), // already uses len_utf16 internally
                    TrackAction::Emoji => self.track_emoji(c), // ditto
                    TrackAction::Url => {}                   // defer to track_url below
                }
            }
        }

        if let TrackAction::Url = action {
            self.track_url(offset); // now offset is in UTF-16 code units
        }

        offset
    }
}

enum TrackAction {
    Text,
    Emoji,
    Url,
}

pub enum UnprocessedEntity<'a> {
    UrlSpan(pest::Span<'a>),
    Pair(Pair<'a>),
    FullTldPair(FullTldPair<'a>),
}

impl<'a> UnprocessedEntity<'a> {
    fn start(&self) -> usize {
        match self {
            UnprocessedEntity::UrlSpan(span) => span.start(),
            UnprocessedEntity::Pair(pair) => pair.as_span().start(),
            UnprocessedEntity::FullTldPair(pair) => pair.as_span().start(),
        }
    }

    fn end(&self) -> usize {
        match self {
            UnprocessedEntity::UrlSpan(span) => span.end(),
            UnprocessedEntity::Pair(pair) => pair.as_span().end(),
            UnprocessedEntity::FullTldPair(pair) => pair.as_span().end(),
        }
    }

    fn as_rule(&self) -> Rule {
        match self {
            UnprocessedEntity::UrlSpan(_span) => Rule::url,
            UnprocessedEntity::Pair(pair) => pair.as_rule(),
            // Convert FullTldRule to Rule - they have the same variant names
            UnprocessedEntity::FullTldPair(pair) => full_tld_rule_to_rule(pair.as_rule()),
        }
    }
}

/// Convert a FullTldRule to the equivalent Rule.
/// Both enums have the same variant names, just generated from different grammars.
fn full_tld_rule_to_rule(r: FullTldRule) -> Rule {
    match r {
        FullTldRule::url => Rule::url,
        FullTldRule::url_without_protocol => Rule::url_without_protocol,
        FullTldRule::hashtag => Rule::hashtag,
        FullTldRule::cashtag => Rule::cashtag,
        FullTldRule::username => Rule::username,
        FullTldRule::list => Rule::list,
        FullTldRule::listname => Rule::listname,
        FullTldRule::list_slug => Rule::list_slug,
        FullTldRule::invalid_char => Rule::invalid_char,
        FullTldRule::emoji => Rule::emoji,
        _ => Rule::tweet, // fallback for rules we don't use directly
    }
}

/// Convert a RuleMatch function to work with FullTldRule.
fn convert_rule_match(r_match: RuleMatch) -> impl Fn(FullTldRule) -> bool {
    move |r: FullTldRule| {
        let equivalent_rule = full_tld_rule_to_rule(r);
        r_match(equivalent_rule)
    }
}

/// Validates a URL parsed with the full TLD grammar.
/// Since the grammar already validated the TLD, we only need to check punycode validity.
fn validate_url_full_tld(p: &FullTldPair) -> bool {
    let original = p.as_str();
    match p.clone().into_inner().find(|pair| {
        let r = pair.as_rule();
        r == FullTldRule::host || r == FullTldRule::tco_domain || r == FullTldRule::uwp_domain
    }) {
        Some(pair) => valid_punycode_full_tld(original, &pair),
        None => false,
    }
}

/// Validates punycode for a domain parsed with the full TLD grammar.
fn valid_punycode_full_tld(original: &str, domain: &FullTldPair) -> bool {
    let source = domain.as_span().as_str();
    let uts46 = Uts46::new();

    let result = uts46.to_ascii(
        source.as_bytes(),
        AsciiDenyList::EMPTY,
        Hyphens::Allow,
        DnsLength::Verify,
    );

    match result {
        Ok(s) => length_check(
            original,
            source,
            &s,
            domain.as_rule() != FullTldRule::uwp_domain,
        ),
        Err(_) => false,
    }
}

fn calculate_offset(s: &str) -> usize {
    s.chars().next().unwrap_or(' ').len_utf8()
}

/// Validates a URL and returns the number of bytes to trim from the end.
/// Returns Some(0) if URL is valid as-is, Some(n) if n bytes need trimming,
/// or None if URL is invalid.
///
/// The `external_validator` parameter controls how TLDs are validated:
/// - `ExternalValidator::Pest`: Trust the Pest grammar's TLD matching (no external validation)
/// - `ExternalValidator::External`: Use phf lookup for O(1) TLD validation
fn validate_url(
    p: Pair,
    requires_exact_tld: bool,
    external_validator: ExternalValidator,
) -> Option<usize> {
    let original_span = p.as_span();
    let original = p.as_str();
    match p.into_inner().find(|pair| {
        let r = pair.as_rule();
        r == Rule::host || r == Rule::tco_domain || r == Rule::uwp_domain
    }) {
        Some(pair) => {
            // For tco_domain (t.co), skip TLD validation - it's hardcoded in grammar
            if pair.as_rule() == Rule::tco_domain {
                return if valid_punycode(original, &pair) {
                    Some(0)
                } else {
                    None
                };
            }

            // For Pest backend, trust the grammar's TLD matching entirely
            // Just validate punycode and return without trimming
            if external_validator == ExternalValidator::Pest {
                return if valid_punycode(original, &pair) {
                    Some(0)
                } else {
                    None
                };
            }

            // External TLD validation using phf lookup
            // Get the domain text
            let domain_span = pair.as_span();
            let domain = domain_span.as_str();

            // For URLs without protocol, check for script mixing in domain labels
            // (e.g., "example.comだよね" should be trimmed to "example.com")
            // URLs with protocol can have mixed-script domains via IDNA/punycode
            if requires_exact_tld {
                let valid_domain_end = find_valid_domain_end(domain);
                let domain_trim = domain.len() - valid_domain_end;

                // If we trimmed the domain, we need to recalculate TLD
                if domain_trim > 0 {
                    let trimmed_domain = &domain[..valid_domain_end];
                    // Find the last dot to get the TLD portion
                    if let Some(last_dot) = trimmed_domain.rfind('.') {
                        let tld = &trimmed_domain[last_dot + 1..];
                        if is_valid_tld(&tld.to_lowercase()) {
                            // Calculate bytes from end of original URL span to end of valid domain
                            let domain_end_in_url = domain_span.end() - original_span.start();
                            let url_len = original.len();
                            let after_domain = url_len - domain_end_in_url;
                            let total_trim = domain_trim + after_domain;
                            // Still need punycode validation on the trimmed domain
                            return Some(total_trim);
                        }
                    }
                    return None;
                }
            }

            // Validate TLD by working through domain from right to left
            // This handles cases like "example.comだよね.comtest" where we need to find
            // the rightmost valid TLD boundary (should stop at "example.com")
            match find_valid_tld_boundary(domain, requires_exact_tld) {
                Some(valid_domain_len) => {
                    let domain_trim = domain.len() - valid_domain_len;

                    if domain_trim == 0 {
                        // Full domain is valid, keep entire URL including path/query/fragment
                        if valid_punycode(original, &pair) {
                            Some(0)
                        } else {
                            None
                        }
                    } else {
                        // Need to trim domain - also trim everything after the domain
                        let domain_end_in_url = domain_span.end() - original_span.start();
                        let url_len = original.len();
                        let after_domain = url_len - domain_end_in_url;
                        let total_trim = domain_trim + after_domain;
                        Some(total_trim)
                    }
                }
                None => None,
            }
        }
        _ => None,
    }
}

/// Find the valid TLD boundary in a domain.
///
/// For normal domains like "msdn.microsoft.com", uses right-to-left to find the rightmost TLD.
/// For domains with script mixing like "example.comだよね.comtest", finds the first valid TLD
/// before the script boundary.
///
/// Returns Some(byte_position) of the valid domain end, or None if no valid TLD found.
fn find_valid_tld_boundary(domain: &str, requires_exact_tld: bool) -> Option<usize> {
    // First check if domain has any script mixing (Latin followed by non-Latin in same label)
    let has_script_mixing_in_domain = domain.split('.').any(|label| has_script_mixing(label));

    // Find all dot positions in the domain
    let dot_positions: Vec<usize> = domain
        .char_indices()
        .filter(|(_, c)| *c == '.')
        .map(|(i, _)| i)
        .collect();

    if has_script_mixing_in_domain {
        // For domains with script mixing, work left to right to find the first valid TLD
        // before the script boundary (e.g., "example.comだよね.comtest" -> "example.com")
        for &dot_pos in &dot_positions {
            let after_dot = &domain[dot_pos + 1..];
            let segment_end = after_dot.find('.').unwrap_or(after_dot.len());
            let segment = &after_dot[..segment_end];

            // If this segment has script mixing, find the valid prefix
            if has_script_mixing(segment) {
                let boundary = find_script_boundary(segment);
                let effective_segment = &segment[..boundary];
                let segment_lower = effective_segment.to_lowercase();

                if !segment_lower.is_empty() && is_valid_tld(&segment_lower) {
                    let end_pos = dot_pos + 1 + effective_segment.len();
                    #[cfg(test)]
                    eprintln!(
                        "find_valid_tld_boundary: found valid TLD '{}' at script boundary, position {}",
                        segment_lower, end_pos
                    );
                    return Some(end_pos);
                }
            }
        }
    }

    // Normal case: work right to left to find the rightmost valid TLD
    for &dot_pos in dot_positions.iter().rev() {
        let after_dot = &domain[dot_pos + 1..];
        let segment_end = after_dot.find('.').unwrap_or(after_dot.len());
        let segment = &after_dot[..segment_end];
        let segment_lower = segment.to_lowercase();

        // Check if this segment is a valid TLD
        if is_valid_tld(&segment_lower) {
            let end_pos = dot_pos + 1 + segment.len();
            #[cfg(test)]
            eprintln!(
                "find_valid_tld_boundary: found valid TLD '{}' at position {}",
                segment_lower, end_pos
            );
            return Some(end_pos);
        }

        // For URLs without protocol (requires_exact_tld=true), we need prefix matching
        // for Unicode TLDs like みんな from みんなです.
        // For URLs with protocol, we don't do prefix matching (grammar already determined the TLD).
        if requires_exact_tld && !segment_lower.is_ascii() {
            // Check if a valid Unicode TLD is a prefix of this segment
            // We try progressively shorter prefixes
            for (char_idx, _) in segment.char_indices().skip(2) {
                let prefix = &segment[..char_idx];
                let prefix_lower = prefix.to_lowercase();
                if is_valid_tld(&prefix_lower) {
                    let end_pos = dot_pos + 1 + prefix.len();
                    #[cfg(test)]
                    eprintln!(
                        "find_valid_tld_boundary: found valid Unicode TLD prefix '{}' at position {}",
                        prefix_lower, end_pos
                    );
                    return Some(end_pos);
                }
            }
        }
    }

    // No valid TLD found
    #[cfg(test)]
    eprintln!(
        "find_valid_tld_boundary: no valid TLD found in '{}'",
        domain
    );
    None
}

/// Find the end position of the valid domain, trimming any script-mixed labels.
/// Returns the byte offset where the valid domain ends.
///
/// Domain labels cannot mix ASCII and non-ASCII characters (except punycode xn--).
/// For example, "example.comだよね" should be trimmed to "example.com"
fn find_valid_domain_end(domain: &str) -> usize {
    let mut valid_end = domain.len();

    // Check each label from right to left
    let labels: Vec<&str> = domain.split('.').collect();
    let mut pos = domain.len();

    for label in labels.iter().rev() {
        let label_start = pos - label.len();

        // Check if this label has script mixing (ASCII then non-ASCII)
        if has_script_mixing(label) {
            // Find where the valid part ends within this label
            let valid_label_end = find_script_boundary(label);
            valid_end = label_start + valid_label_end;
            break;
        }

        // Move to before the dot
        pos = label_start.saturating_sub(1);
    }

    valid_end
}

/// Check if a character is Latin (ASCII or Latin Extended)
fn is_latin_char(c: char) -> bool {
    c.is_ascii_alphabetic()
        || ('\u{00C0}'..='\u{00FF}').contains(&c)  // Latin-1 Supplement
        || ('\u{0100}'..='\u{017F}').contains(&c)  // Latin Extended-A
        || ('\u{0180}'..='\u{024F}').contains(&c) // Latin Extended-B
}

/// Check if a label has invalid script mixing (Latin followed by non-Latin scripts)
fn has_script_mixing(label: &str) -> bool {
    // Punycode labels (xn--) can have any characters
    if label.starts_with("xn--") || label.starts_with("XN--") {
        return false;
    }

    let mut seen_latin = false;
    for c in label.chars() {
        if is_latin_char(c) {
            seen_latin = true;
        } else if c.is_ascii_digit() || c == '-' {
            // Digits and hyphens are allowed in any label
            continue;
        } else if seen_latin {
            // Found non-Latin after Latin letter - this is script mixing
            return true;
        }
    }
    false
}

/// Find the byte position where script mixing begins in a label.
/// Returns the valid portion length.
fn find_script_boundary(label: &str) -> usize {
    let mut last_valid = 0;
    let mut seen_latin = false;

    for (i, c) in label.char_indices() {
        if is_latin_char(c) || c.is_ascii_digit() || c == '-' {
            seen_latin = seen_latin || is_latin_char(c);
            last_valid = i + c.len_utf8();
        } else if seen_latin {
            // Non-Latin after Latin - stop here
            break;
        } else {
            // Non-Latin before any Latin - this is a Unicode-start label, allow it
            last_valid = i + c.len_utf8();
        }
    }

    last_valid
}

fn valid_punycode(original: &str, domain: &pest::iterators::Pair<Rule>) -> bool {
    let source = domain.as_span().as_str();
    let uts46 = Uts46::new();

    // This no longer allows transitional processing, we'll see if that matters.
    let result = uts46.to_ascii(
        source.as_bytes(),
        // No ASCII deny list. This corresponds to UseSTD3ASCIIRules=false.
        AsciiDenyList::EMPTY,
        // CheckHyphens=false: Do not place positional restrictions on hyphens.
        // This mode is used by the WHATWG URL Standard for normal User Agent
        // processing (i.e. not conformance checking).
        Hyphens::Allow,
        // VerifyDNSLength=true. (The trailing root label dot is not allowed.)
        DnsLength::Verify,
    );

    match result {
        Ok(s) => length_check(original, source, &s, domain.as_rule() != Rule::uwp_domain),
        Err(_) => false,
    }
}

fn length_check(
    original: &str,
    original_domain: &str,
    punycode_domain: &str,
    has_scheme: bool,
) -> bool {
    let length = if has_scheme { 0 } else { "https://".len() };

    (length + original.len() - original_domain.len() + punycode_domain.len()) < MAX_URL_LENGTH
}

/**
 * The maximum url length that the Twitter backend supports.
 */
pub const MAX_URL_LENGTH: usize = 4096;

// The best that can currently be done per <https://goo.gl/CBHdE9>
fn as_i32(us: usize) -> i32 {
    let u = if us > std::i32::MAX as usize {
        None
    } else {
        Some(us as i32)
    };
    u.unwrap()
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct LengthData {
    normalized_length: i32,
    normalized_length_utf8: i32,
    original_length: i32,
    original_length_utf8: i32,
}

impl LengthData {
    fn empty() -> LengthData {
        LengthData {
            normalized_length: 0,
            normalized_length_utf8: 0,
            original_length: 0,
            original_length_utf8: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_empty_string_mentions() {
        let extractor = Extractor::new();
        let mentions = extractor.extract_mentioned_screennames("");
        assert_eq!(0, mentions.len());
    }

    #[test]
    fn test_extract_single_mention() {
        let extractor = Extractor::new();
        let mentions = extractor.extract_mentioned_screennames("@hi");
        assert_eq!(1, mentions.len());
    }

    #[test]
    fn test_extract_setting() {
        let mut extractor = Extractor::new();
        extractor.set_extract_url_without_protocol(false);
        assert_eq!(false, extractor.get_extract_url_without_protocol());
        extractor.set_extract_url_without_protocol(true);
        assert_eq!(true, extractor.get_extract_url_without_protocol());
    }

    // Reply tests - ported from Java ExtractorTest.ReplyTest

    #[test]
    fn test_reply_at_the_start() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_reply_username("@user reply");
        assert!(extracted.is_some());
        assert_eq!("user", extracted.unwrap().value);
    }

    #[test]
    fn test_reply_with_leading_space() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_reply_username(" @user reply");
        assert!(extracted.is_some());
        assert_eq!("user", extracted.unwrap().value);
    }

    // Mention tests - ported from Java ExtractorTest.MentionTest

    #[test]
    fn test_mention_at_the_beginning() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_mentioned_screennames("@user mention");
        assert_eq!(vec!["user"], extracted);
    }

    #[test]
    fn test_mention_with_leading_space() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_mentioned_screennames(" @user mention");
        assert_eq!(vec!["user"], extracted);
    }

    #[test]
    fn test_mention_in_mid_text() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_mentioned_screennames("mention @user here");
        assert_eq!(vec!["user"], extracted);
    }

    #[test]
    fn test_multiple_mentions() {
        let extractor = Extractor::new();
        let extracted =
            extractor.extract_mentioned_screennames("mention @user1 here and @user2 here");
        assert_eq!(vec!["user1", "user2"], extracted);
    }

    #[test]
    fn test_mention_with_indices() {
        let extractor = Extractor::new();
        let extracted = extractor
            .extract_mentioned_screennames_with_indices(" @user1 mention @user2 here @user3 ");
        assert_eq!(3, extracted.len());
        assert_eq!(1, extracted[0].start);
        assert_eq!(7, extracted[0].end);
        assert_eq!(16, extracted[1].start);
        assert_eq!(22, extracted[1].end);
        assert_eq!(28, extracted[2].start);
        assert_eq!(34, extracted[2].end);
    }

    #[test]
    fn test_mention_with_supplementary_characters() {
        // U+10400 DESERET CAPITAL LETTER LONG I
        let text = format!("\u{10400} @mention \u{10400} @mention");
        let extractor = Extractor::new();

        // Extract with UTF-16 indices
        let extracted = extractor.extract_mentioned_screennames_with_indices(&text);
        assert_eq!(2, extracted.len());

        // First mention
        assert_eq!("mention", extracted[0].value);
        // U+10400 takes 2 UTF-16 code units (surrogate pair), then space (1), then @ (1) = index 3
        assert_eq!(3, extracted[0].start);
        // Start (3) + "@mention" (8) = 11
        assert_eq!(11, extracted[0].end);

        // Second mention
        assert_eq!("mention", extracted[1].value);
        // First mention ends at 11, space (1), U+10400 (2), space (1), @ (1) = 15
        assert_eq!(15, extracted[1].start);
        assert_eq!(23, extracted[1].end);
    }

    // Hashtag tests - ported from Java ExtractorTest.HashtagTest

    #[test]
    fn test_hashtag_at_the_beginning() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_hashtags("#hashtag mention");
        assert_eq!(vec!["hashtag"], extracted);
    }

    #[test]
    fn test_hashtag_with_leading_space() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_hashtags(" #hashtag mention");
        assert_eq!(vec!["hashtag"], extracted);
    }

    #[test]
    fn test_hashtag_in_mid_text() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_hashtags("mention #hashtag here");
        assert_eq!(vec!["hashtag"], extracted);
    }

    #[test]
    fn test_multiple_hashtags() {
        let extractor = Extractor::new();
        let extracted = extractor.extract_hashtags("text #hashtag1 #hashtag2");
        assert_eq!(vec!["hashtag1", "hashtag2"], extracted);
    }

    #[test]
    fn test_hashtag_with_indices() {
        let extractor = Extractor::new();
        let extracted =
            extractor.extract_hashtags_with_indices(" #user1 mention #user2 here #user3 ");
        assert_eq!(3, extracted.len());
        assert_eq!(1, extracted[0].start);
        assert_eq!(7, extracted[0].end);
        assert_eq!(16, extracted[1].start);
        assert_eq!(22, extracted[1].end);
        assert_eq!(28, extracted[2].start);
        assert_eq!(34, extracted[2].end);
    }

    #[test]
    fn test_hashtag_with_supplementary_characters() {
        let text = format!("\u{10400} #hashtag \u{10400} #hashtag");
        let extractor = Extractor::new();

        let extracted = extractor.extract_hashtags_with_indices(&text);
        assert_eq!(2, extracted.len());

        assert_eq!("hashtag", extracted[0].value);
        assert_eq!(3, extracted[0].start);
        assert_eq!(11, extracted[0].end);

        assert_eq!("hashtag", extracted[1].value);
        assert_eq!(15, extracted[1].start);
        assert_eq!(23, extracted[1].end);
    }

    // URL tests - ported from Java ExtractorTest.URLTest

    #[test]
    fn test_url_with_indices() {
        let extractor = Extractor::new();
        let extracted =
            extractor.extract_urls_with_indices("http://t.co url https://www.twitter.com ");
        assert_eq!(2, extracted.len());
        assert_eq!(0, extracted[0].start);
        assert_eq!(11, extracted[0].end);
        assert_eq!(16, extracted[1].start);
        assert_eq!(39, extracted[1].end);
    }

    #[test]
    fn test_url_without_protocol() {
        let extractor = Extractor::new();
        let text = "www.twitter.com, www.yahoo.co.jp, t.co/blahblah, www.poloshirts.uk.com";
        let extracted = extractor.extract_urls(text);
        assert_eq!(
            vec![
                "www.twitter.com",
                "www.yahoo.co.jp",
                "t.co/blahblah",
                "www.poloshirts.uk.com"
            ],
            extracted
        );

        let extracted_with_indices = extractor.extract_urls_with_indices(text);
        assert_eq!(4, extracted_with_indices.len());
        assert_eq!(0, extracted_with_indices[0].start);
        assert_eq!(15, extracted_with_indices[0].end);
        assert_eq!(17, extracted_with_indices[1].start);
        assert_eq!(32, extracted_with_indices[1].end);
        assert_eq!(34, extracted_with_indices[2].start);
        assert_eq!(47, extracted_with_indices[2].end);
    }

    #[test]
    fn test_url_without_protocol_disabled() {
        let mut extractor = Extractor::new();
        extractor.set_extract_url_without_protocol(false);
        let text = "www.twitter.com, www.yahoo.co.jp, t.co/blahblah";
        let extracted = extractor.extract_urls(text);
        assert_eq!(0, extracted.len());
    }

    #[test]
    fn test_url_followed_by_punctuations() {
        let extractor = Extractor::new();
        let text = "http://games.aarp.org/games/mahjongg-dimensions.aspx!!!!!!";
        let extracted = extractor.extract_urls(text);
        assert_eq!(
            vec!["http://games.aarp.org/games/mahjongg-dimensions.aspx"],
            extracted
        );
    }

    #[test]
    fn test_url_with_punctuation() {
        let extractor = Extractor::new();
        let urls = vec![
            "http://www.foo.com/foo/path-with-period./",
            "http://www.foo.org.za/foo/bar/688.1",
            "http://www.foo.com/bar-path/some.stm?param1=foo;param2=P1|0||P2|0",
            "http://foo.com/bar/123/foo_&_bar/",
            "http://foo.com/bar(test)bar(test)bar(test)",
            "www.foo.com/foo/path-with-period./",
            "www.foo.org.za/foo/bar/688.1",
            "www.foo.com/bar-path/some.stm?param1=foo;param2=P1|0||P2|0",
            "foo.com/bar/123/foo_&_bar/",
        ];

        for url in urls {
            let extracted = extractor.extract_urls(url);
            assert_eq!(vec![url], extracted, "Failed to extract URL: {}", url);
        }
    }

    #[test]
    fn test_url_with_supplementary_characters() {
        let text = format!("\u{10400} http://twitter.com \u{10400} http://twitter.com");
        let extractor = Extractor::new();

        let extracted = extractor.extract_urls_with_indices(&text);
        assert_eq!(2, extracted.len());

        assert_eq!("http://twitter.com", extracted[0].value);
        assert_eq!(3, extracted[0].start);
        assert_eq!(21, extracted[0].end);

        assert_eq!("http://twitter.com", extracted[1].value);
        assert_eq!(25, extracted[1].start);
        assert_eq!(43, extracted[1].end);
    }

    #[test]
    fn test_url_with_special_cctld_without_protocol() {
        let extractor = Extractor::new();
        let text = "MLB.tv vine.co";
        let extracted = extractor.extract_urls(text);
        assert_eq!(vec!["MLB.tv", "vine.co"], extracted);

        let extracted_with_indices = extractor.extract_urls_with_indices(text);
        assert_eq!(2, extracted_with_indices.len());
        assert_eq!(0, extracted_with_indices[0].start);
        assert_eq!(6, extracted_with_indices[0].end);
        assert_eq!(7, extracted_with_indices[1].start);
        assert_eq!(14, extracted_with_indices[1].end);
    }

    #[test]
    fn test_url_with_special_cctld_disabled() {
        let mut extractor = Extractor::new();
        extractor.set_extract_url_without_protocol(false);
        let text = "MLB.tv vine.co";
        let extracted = extractor.extract_urls(text);
        assert_eq!(0, extracted.len());
    }

    #[test]
    fn test_url_with_unicode_tld() {
        let extractor = Extractor::new();
        // Korean TLD: 한국
        let text = "https://twitter.한국";
        let extracted = extractor.extract_urls(text);
        assert_eq!(vec!["https://twitter.한국"], extracted);
    }

    #[test]
    fn test_url_with_trailing_cjk() {
        // This is the failing conformance test case
        let extractor = Extractor::new();
        let text = "test http://example.comだよね.comtest/hogehoge";
        let extracted = extractor.extract_urls(text);
        assert_eq!(vec!["http://example.com"], extracted);
    }

    #[test]
    fn test_simple_url() {
        let extractor = Extractor::new();
        let text = "http://example.com/";
        let extracted = extractor.extract_urls(text);
        assert_eq!(vec!["http://example.com/"], extracted);
    }
}

/// Debug tests for URL extraction edge cases.
/// These tests help diagnose issues with TLD validation, script mixing,
/// punycode handling, and other URL extraction behaviors.
#[cfg(test)]
mod debug_tests {
    use super::*;
    use crate::tlds::is_valid_tld;
    use idna::uts46::{AsciiDenyList, DnsLength, Hyphens, Uts46};
    use pest::Parser;
    use twitter_text_parser::twitter_text::Rule;
    use twitter_text_parser::twitter_text::TwitterTextParser;

    // Domain and script boundary tests

    #[test]
    fn test_script_boundary_unicode_domain() {
        // "twitter.한국" - should NOT have script mixing since dot separates them
        let domain = "twitter.한국";
        let valid_end = find_valid_domain_end(domain);
        eprintln!(
            "Domain: {} -> valid_end: {} (len: {})",
            domain,
            valid_end,
            domain.len()
        );
        assert_eq!(
            valid_end,
            domain.len(),
            "Unicode TLD domains should be fully valid"
        );
    }

    #[test]
    fn test_script_boundary_mixed_label() {
        // "comだよね" - ASCII then non-ASCII in same label
        let label = "comだよね";
        assert!(
            has_script_mixing(label),
            "comだよね should have script mixing"
        );
        let boundary = find_script_boundary(label);
        eprintln!("Label: {} -> boundary: {}", label, boundary);
        assert_eq!(boundary, 3, "Should stop after 'com'");
    }

    // TLD validation tests

    #[test]
    fn test_vermogen_tld() {
        let tld = "vermögensberatung";
        eprintln!("Testing TLD: {} (len: {})", tld, tld.len());
        eprintln!("Is valid TLD: {}", is_valid_tld(tld));
        eprintln!("Lowercase: {}", is_valid_tld(&tld.to_lowercase()));

        let extractor = Extractor::new();
        let text = "https://twitter.vermögensberatung";
        eprintln!("Testing URL: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted: {:?}", urls);
    }

    // URL without protocol (UWP) tests

    #[test]
    fn test_uwp_extraction() {
        let extractor = Extractor::new();
        let text = "foo.baz foo.co.jp www.xxxxxxx.baz www.foo.co.uk wwwww.xxxxxxx foo.comm foo.somecom foo.govedu foo.jp";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  {}", url);
        }
        eprintln!("Expected: foo.co.jp, www.foo.co.uk, foo.jp");
    }

    #[test]
    fn test_japanese_uwp() {
        let extractor = Extractor::new();
        let text = "example.comてすとですtwitter.みんなです";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  {}", url);
        }
        eprintln!("Expected: example.com, twitter.みんな");
    }

    // IDN and mixed script tests

    #[test]
    fn test_idn_mixed_domain() {
        let extractor = Extractor::new();
        let text = "http://exampleこれは日本語です.com/path/index.html";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  {}", url);
        }
    }

    #[test]
    fn test_trailing_cjk() {
        let extractor = Extractor::new();
        let text = "http://example.comだよね.comtest/hoge";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  {}", url);
        }
        eprintln!("Expected: http://example.com");
    }

    // Punycode tests

    #[test]
    fn test_punycode_mixed() {
        let uts46 = Uts46::new();

        // Test the problematic domain
        let domain = "example.comだよね.comtest";
        eprintln!("Testing domain: {}", domain);
        let result = uts46.to_ascii(
            domain.as_bytes(),
            AsciiDenyList::EMPTY,
            Hyphens::Allow,
            DnsLength::Verify,
        );
        eprintln!("Result: {:?}", result);

        // What about just the label?
        let label = "comだよね";
        eprintln!("\nTesting label: {}", label);
        let result2 = uts46.to_ascii(
            label.as_bytes(),
            AsciiDenyList::EMPTY,
            Hyphens::Allow,
            DnsLength::Verify,
        );
        eprintln!("Result: {:?}", result2);
    }

    #[test]
    fn test_punycode_trailingdash_twitter() {
        let uts46 = Uts46::new();

        let domain = "trailingdash.twitter";
        eprintln!("Testing domain: {}", domain);
        let result = uts46.to_ascii(
            domain.as_bytes(),
            AsciiDenyList::EMPTY,
            Hyphens::Allow,
            DnsLength::Verify,
        );
        eprintln!("Result: {:?}", result);

        let domain2 = "trailingdash.tw";
        eprintln!("\nTesting domain: {}", domain2);
        let result2 = uts46.to_ascii(
            domain2.as_bytes(),
            AsciiDenyList::EMPTY,
            Hyphens::Allow,
            DnsLength::Verify,
        );
        eprintln!("Result: {:?}", result2);
    }

    #[test]
    fn test_punycode_idn_url() {
        let extractor = Extractor::new();
        let text = "See also: http://xn--80abe5aohbnkjb.xn--p1ai/";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  URL: '{}'", url);
        }
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "http://xn--80abe5aohbnkjb.xn--p1ai/");
    }

    // Complex URL tests

    #[test]
    fn test_msdn_url() {
        let extractor = Extractor::new();
        let text =
            "http://msdn.microsoft.com/ja-jp/library/system.net.httpwebrequest(v=VS.100).aspx";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  {}", url);
        }
    }

    // Trailing dash tests

    #[test]
    fn test_trailing_dash() {
        let extractor = Extractor::new();
        let text = "test http://trailingdash.twitter-.com";
        eprintln!("Testing: {}", text);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted {} URLs:", urls.len());
        for url in &urls {
            eprintln!("  {}", url);
        }
        eprintln!("Expected: 0 URLs");
    }

    #[test]
    fn test_trailing_dash_debug() {
        let domain = "trailingdash.twitter-.com";
        eprintln!("Domain: {}", domain);

        // Find dots
        let dot_positions: Vec<usize> = domain
            .char_indices()
            .filter(|(_, c)| *c == '.')
            .map(|(i, _)| i)
            .collect();
        eprintln!("Dot positions: {:?}", dot_positions);

        // Check for script mixing
        let has_mixing = domain.split('.').any(|l| has_script_mixing(l));
        eprintln!("Has script mixing: {}", has_mixing);

        // Check each segment
        for &dot_pos in dot_positions.iter().rev() {
            let after_dot = &domain[dot_pos + 1..];
            let segment_end = after_dot.find('.').unwrap_or(after_dot.len());
            let segment = &after_dot[..segment_end];
            eprintln!(
                "At dot {}: after_dot='{}', segment='{}'",
                dot_pos, after_dot, segment
            );
            eprintln!(
                "  is_valid_tld('{}') = {}",
                segment.to_lowercase(),
                is_valid_tld(&segment.to_lowercase())
            );
        }

        let result = find_valid_tld_boundary(domain, false);
        eprintln!("Result: {:?}", result);
    }

    // Parser debug tests

    #[test]
    fn test_parser_trailing_dash() {
        let text = "test http://trailingdash.twitter-.com";
        eprintln!("Input: {}", text);

        match TwitterTextParser::parse(Rule::tweet, text) {
            Ok(p) => {
                for pair in p.flatten() {
                    let r = pair.as_rule();
                    if r == Rule::url || r == Rule::host {
                        eprintln!("{:?}: '{}'", r, pair.as_str());
                    }
                }
            }
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }

    // ExternalValidator backend tests

    #[test]
    fn test_external_validator_external_validates_tlds() {
        // External backend should reject invalid TLDs
        let extractor = Extractor::with_external_validator(ExternalValidator::External);

        // Valid TLD should be extracted
        let urls = extractor.extract_urls("Check out http://example.com/path");
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "http://example.com/path");

        // URL with valid punycode TLD
        let urls = extractor.extract_urls("See http://xn--80abe5aohbnkjb.xn--p1ai/");
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn test_external_validator_pest_trusts_grammar() {
        // Pest backend trusts whatever the grammar matches
        let extractor = Extractor::with_external_validator(ExternalValidator::Pest);

        // With the permissive grammar, this will also match
        let urls = extractor.extract_urls("Check out http://example.com/path");
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "http://example.com/path");
    }

    #[test]
    fn test_external_validator_default_is_external() {
        // Default should be External
        assert_eq!(ExternalValidator::default(), ExternalValidator::External);

        // New extractor should use External by default
        let extractor = Extractor::new();
        assert_eq!(
            extractor.get_external_validator(),
            ExternalValidator::External
        );
    }

    #[test]
    fn test_both_backends_produce_same_results() {
        // Test that both backends produce identical results for common cases
        let external = Extractor::with_external_validator(ExternalValidator::External);
        let pest = Extractor::with_external_validator(ExternalValidator::Pest);

        let test_cases = [
            "Check out http://example.com/path",
            "Visit https://twitter.com and https://github.com",
            "See http://xn--80abe5aohbnkjb.xn--p1ai/",
            "URL without protocol: example.com/page",
            "Japanese TLD: http://example.みんな/path",
            "Multiple: foo.co.jp and bar.co.uk are valid",
            "@mention and #hashtag with http://test.org",
        ];

        for text in test_cases {
            let external_urls = external.extract_urls(text);
            let pest_urls = pest.extract_urls(text);
            assert_eq!(
                external_urls, pest_urls,
                "Backends differ on: {}\nExternal: {:?}\nPest: {:?}",
                text, external_urls, pest_urls
            );
        }
    }

    #[test]
    fn test_pest_backend_extracts_punycode_urls() {
        let extractor = Extractor::with_external_validator(ExternalValidator::Pest);

        // Punycode URL should be extracted
        let urls = extractor.extract_urls("See http://xn--80abe5aohbnkjb.xn--p1ai/");
        assert_eq!(urls.len(), 1, "Pest backend should extract punycode URLs");
        assert_eq!(urls[0], "http://xn--80abe5aohbnkjb.xn--p1ai/");
    }
}
