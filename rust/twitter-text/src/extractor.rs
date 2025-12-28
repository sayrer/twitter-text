// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use crate::entity::{Entity, Type};
use crate::nom_parser::{self, NomEntity, NomEntityType};
use crate::tlds::is_valid_tld_case_insensitive;
use crate::TwitterTextParseResults;
use idna::uts46::{AsciiDenyList, DnsLength, Hyphens, Uts46};
use pest::Parser;
use std::iter::Peekable;
use std::str::CharIndices;
use twitter_text_config::Configuration;
use twitter_text_config::Range;
use twitter_text_parser::twitter_text::Rule;
use twitter_text_parser::twitter_text::TwitterTextParser;
// Full Pest parser for ParserBackend::Pest mode
use twitter_text_parser::twitter_text::full_pest::Rule as FullPestRule;
use twitter_text_parser::twitter_text::full_pest::TwitterTextFullPestParser;
use unicode_normalization::{is_nfc, UnicodeNormalization};

/// Checks if an emoji string is valid using the emojis crate.
/// Handles variation selector FE0F (emoji presentation selector) by stripping it
/// when not part of a ZWJ sequence, similar to twemoji-parser's removeVS16s.
fn is_valid_emoji(s: &str) -> bool {
    // First try direct lookup
    if emojis::get(s).is_some() {
        return true;
    }

    // If direct lookup failed and string contains FE0F but no ZWJ,
    // try stripping FE0F and looking up again
    const VS16: char = '\u{fe0f}';
    const ZWJ: char = '\u{200d}';

    if s.contains(VS16) && !s.contains(ZWJ) {
        let stripped: String = s.chars().filter(|&c| c != VS16).collect();
        return emojis::get(&stripped).is_some();
    }

    false
}

type RuleMatch = fn(Rule) -> bool;
type Pair<'a> = pest::iterators::Pair<'a, Rule>;
type FullPestPair<'a> = pest::iterators::Pair<'a, FullPestRule>;

/// Selects the TLD matching strategy for URL extraction.
///
/// This enum allows choosing between different parsing backends as described
/// in PARSER_BACKENDS.md. The choice affects how TLDs are validated during
/// URL extraction.
///
/// # Current Implementation
///
/// The current Pest grammar uses a permissive "domain-like" pattern for TLDs,
/// so `ParserBackend::External` (the default) is required for correct TLD validation.
/// `ParserBackend::Pest` trusts whatever the grammar matches, which may include
/// invalid TLDs with the current permissive grammar.
///
/// To use `ParserBackend::Pest` correctly, the grammar would need to be restored
/// to include the full TLD alternation (the original ~1500 TLD list).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ParserBackend {
    /// Pure Pest parsing - trusts the Pest grammar's TLD matching.
    ///
    /// **Note:** With the current permissive grammar, this will accept any
    /// domain-like pattern. Use `Nom` for correct TLD validation.
    Pest,

    /// Pest for structure, external TLD lookup via phf perfect hash.
    /// The Pest grammar matches a permissive "domain-like" pattern, then
    /// Rust code validates the TLD using O(1) phf lookup.
    External,

    /// Nom parser with external TLD/emoji validation.
    /// Uses nom combinators compiled to native code instead of Pest's VM.
    /// This is the fastest and recommended backend.
    #[default]
    /// TLD validation via phf, emoji validation via emojis crate.
    /// This is the fastest backend.
    Nom,
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
    fn get_parser_backend(&self) -> ParserBackend;

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

        let parser_backend = self.get_parser_backend();

        // Branch based on TLD matcher to use the appropriate parser
        match parser_backend {
            ParserBackend::Pest => self.extract_impl_full_pest(s, r_match),
            ParserBackend::External => self.extract_impl_external(s, r_match),
            ParserBackend::Nom => self.extract_impl_nom(s, r_match),
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
                    if r == Rule::invalid_char {
                        scanned.push(UnprocessedEntity::Pair(pair));
                    } else if r == Rule::emoji {
                        // Validate emoji using external crate (handles FE0F stripping)
                        if is_valid_emoji(pair.as_str()) {
                            scanned.push(UnprocessedEntity::Pair(pair));
                        }
                        // If not a valid emoji, skip it (treat as regular text)
                    } else if r_match(r) {
                        if r == Rule::url || r == Rule::url_without_protocol {
                            let span = pair.as_span();
                            let requires_exact_tld = r == Rule::url_without_protocol;
                            if let Some(trim_bytes) =
                                validate_url(pair, requires_exact_tld, ParserBackend::External)
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

    /// Implementation using the full Pest grammar - Pest handles TLD and emoji validation.
    fn extract_impl_full_pest(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        // Convert the RuleMatch function to work with FullPestRule
        let full_pest_r_match = convert_rule_match(r_match);

        match TwitterTextFullPestParser::parse(FullPestRule::tweet, s) {
            Ok(p) => {
                let mut scanned = Vec::new();
                let mut entity_count = 0;

                p.flatten().for_each(|pair| {
                    let r = pair.as_rule();
                    if r == FullPestRule::invalid_char || r == FullPestRule::emoji {
                        // Convert FullPestPair to regular Pair by re-parsing with regular parser
                        // We store the span and will create the entity from it
                        scanned.push(UnprocessedEntity::FullPestPair(pair));
                    } else if full_pest_r_match(r) {
                        if r == FullPestRule::url || r == FullPestRule::url_without_protocol {
                            let span = pair.as_span();
                            // With full Pest grammar, Pest already validated the TLD
                            // We only need to do punycode validation
                            if validate_url_full_pest(&pair) {
                                entity_count += 1;
                                scanned.push(UnprocessedEntity::UrlSpan(span));
                            }
                        } else {
                            entity_count += 1;
                            scanned.push(UnprocessedEntity::FullPestPair(pair));
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

    /// Implementation using nom parser with external TLD/emoji validation.
    /// Uses nom combinators compiled to native code for maximum performance.
    fn extract_impl_nom(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        let nom_entities = nom_parser::parse_tweet(s);

        // Pre-filter and count entities we'll keep
        let mut scanned: Vec<UnprocessedEntity<'a>> = Vec::with_capacity(nom_entities.len());
        let mut entity_count = 0;

        for entity in nom_entities {
            let rule = nom_entity_type_to_rule(entity.entity_type);

            if rule == Rule::invalid_char {
                scanned.push(UnprocessedEntity::NomEntity(entity));
            } else if rule == Rule::emoji {
                // Validate emoji using external crate (handles FE0F stripping)
                if is_valid_emoji(entity.value) {
                    scanned.push(UnprocessedEntity::NomEntity(entity));
                }
            } else if r_match(rule) {
                if rule == Rule::url || rule == Rule::url_without_protocol {
                    // Validate URL and potentially trim to valid TLD boundary
                    let requires_exact_tld = rule == Rule::url_without_protocol;
                    if let Some(trim_bytes) = validate_url_nom(&entity, requires_exact_tld) {
                        entity_count += 1;
                        if trim_bytes > 0 {
                            // Create a trimmed entity
                            let trimmed_entity = NomEntity::new_url(
                                entity.entity_type,
                                &entity.value[..entity.value.len() - trim_bytes],
                                entity.start,
                                entity.end - trim_bytes,
                                entity.host_start.unwrap_or(entity.start),
                                entity
                                    .host_end
                                    .unwrap_or(entity.end)
                                    .min(entity.end - trim_bytes),
                            );
                            scanned.push(UnprocessedEntity::NomEntity(trimmed_entity));
                        } else {
                            scanned.push(UnprocessedEntity::NomEntity(entity));
                        }
                    }
                } else {
                    entity_count += 1;
                    scanned.push(UnprocessedEntity::NomEntity(entity));
                }
            }
        }

        // Nom entities are already in document order, so just reverse once for pop()
        scanned.reverse();
        self.create_result(s, entity_count, &mut scanned)
    }

    /// Extract all URLs from the text, subject to value returned by [Extract::get_extract_url_without_protocol].
    fn extract_urls_with_indices(&self, s: &'a str) -> Self::T {
        if self.get_extract_url_without_protocol() {
            // Early exit if no dot present (URLs without protocol need a dot)
            if !s.contains('.') {
                return self.empty_result();
            }
            self.extract(s, |r| r == Rule::url || r == Rule::url_without_protocol)
        } else {
            // Early exit if no colon present (protocol URLs need a colon)
            if !s.contains(':') {
                return self.empty_result();
            }
            self.extract(s, |r| r == Rule::url)
        }
    }

    /// Extract all Hashtags from the text
    fn extract_hashtags_with_indices(&self, s: &'a str) -> Self::T {
        // Early exit if no hash sign present (ASCII # or full-width ＃)
        if !s.contains('#') && !s.contains('＃') {
            return self.empty_result();
        }
        self.extract(s, |r| r == Rule::hashtag)
    }

    /// Extract all Cashtags from the text
    fn extract_cashtags_with_indices(&self, s: &'a str) -> Self::T {
        // Early exit if no dollar sign present
        if !s.contains('$') {
            return self.empty_result();
        }
        self.extract(s, |r| r == Rule::cashtag)
    }

    /// Extract all usernames from the text.
    fn extract_mentioned_screennames_with_indices(&self, s: &'a str) -> Self::T {
        // Early exit if no at sign present (ASCII @ or full-width ＠)
        if !s.contains('@') && !s.contains('＠') {
            return self.empty_result();
        }
        self.extract(s, |r| r == Rule::username)
    }

    /// Extract all usernames and lists from the text.
    fn extract_mentions_or_lists_with_indices(&self, s: &'a str) -> Self::T {
        // Early exit if no at sign present (ASCII @ or full-width ＠)
        if !s.contains('@') && !s.contains('＠') {
            return self.empty_result();
        }
        self.extract(s, |r| r == Rule::username || r == Rule::list)
    }

    /// Extract all mentions from the text, including regular mentions (@user)
    /// and federated mentions (Mastodon-style @user@domain.tld).
    ///
    /// This matches Mastodon's behavior where extract_mentions_or_lists_with_indices
    /// returns both local and federated mentions together.
    ///
    /// Note: Lists (@user/list) are NOT included - Mastodon doesn't use Twitter-style lists.
    ///
    /// Federated mentions follow the format `@username@domain` where:
    /// - Username: ASCII alphanumeric and underscore, with optional `.` or `-` separators
    /// - Domain: ASCII alphanumeric and underscore, with optional `.` or `-` separators
    /// - Max domain length: 253 characters (not enforced by grammar, checked post-extraction)
    fn extract_federated_mentions_with_indices(&self, s: &'a str) -> Self::T {
        // Early exit if no at sign present (ASCII @ or full-width ＠)
        if !s.contains('@') && !s.contains('＠') {
            return self.empty_result();
        }
        self.extract(s, |r| r == Rule::federated_mention || r == Rule::username)
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
    /// Does NOT include federated mentions. Use `extract_entities_with_indices_federated`
    /// to include Mastodon-style @user@domain mentions.
    fn extract_entities_with_indices(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| {
            r == Rule::url
                || r == Rule::hashtag
                || r == Rule::cashtag
                || r == Rule::list
                || r == Rule::username
        })
    }

    /// Extract all entities from the text, including federated mentions.
    /// This includes Usernames, Lists, Hashtags, Cashtags, URLs, and Mastodon-style
    /// federated mentions (@user@domain.tld).
    fn extract_entities_with_indices_federated(&self, s: &'a str) -> Self::T {
        self.extract(s, |r| {
            r == Rule::url
                || r == Rule::hashtag
                || r == Rule::cashtag
                || r == Rule::list
                || r == Rule::username
                || r == Rule::federated_mention
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
                    Rule::federated_mention => {
                        Some(Entity::new(Type::FEDERATEDMENTION, s, start, end))
                    }
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
            UnprocessedEntity::FullPestPair(pair) => {
                let s = pair.as_str();
                match pair.as_rule() {
                    FullPestRule::hashtag => Some(Entity::new(
                        Type::HASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    FullPestRule::cashtag => Some(Entity::new(
                        Type::CASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    FullPestRule::username => Some(Entity::new(
                        Type::MENTION,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    FullPestRule::federated_mention => {
                        Some(Entity::new(Type::FEDERATEDMENTION, s, start, end))
                    }
                    FullPestRule::list => {
                        let mut list_iter = pair.into_inner();
                        let listname = list_iter.find(|p| p.as_rule() == FullPestRule::listname);
                        let list_slug = list_iter.find(|p| p.as_rule() == FullPestRule::list_slug);
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
            UnprocessedEntity::NomEntity(entity) => {
                let s = entity.value;
                match entity.entity_type {
                    NomEntityType::Url | NomEntityType::UrlWithoutProtocol => {
                        Some(Entity::new(Type::URL, s, start, end))
                    }
                    NomEntityType::Hashtag => Some(Entity::new(
                        Type::HASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    NomEntityType::Cashtag => Some(Entity::new(
                        Type::CASHTAG,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    NomEntityType::Username => Some(Entity::new(
                        Type::MENTION,
                        &s[calculate_offset(s)..],
                        start,
                        end,
                    )),
                    NomEntityType::List => {
                        // For list entities, we need to extract the username and list_slug
                        // The value contains "@user/list", we need to split it
                        // list_slug includes the leading "/" per conformance tests
                        if let Some(slash_pos) = s.find('/') {
                            let name = &s[..slash_pos];
                            let list_slug = &s[slash_pos..]; // include the "/"
                            Some(Entity::new_list(
                                Type::MENTION,
                                &name[calculate_offset(name)..],
                                list_slug,
                                start,
                                end,
                            ))
                        } else {
                            None
                        }
                    }
                    NomEntityType::FederatedMention => {
                        // Federated mentions keep the full value including the @user@domain
                        Some(Entity::new(Type::FEDERATEDMENTION, s, start, end))
                    }
                    NomEntityType::Emoji | NomEntityType::InvalidChar => None,
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
    parser_backend: ParserBackend,
}

impl Extractor {
    /// Create a new extractor with the default TLD matcher (External/phf).
    pub fn new() -> Extractor {
        Extractor {
            extract_url_without_protocol: true,
            parser_backend: ParserBackend::default(),
        }
    }

    /// Create a new extractor with the specified TLD matcher.
    pub fn with_parser_backend(parser_backend: ParserBackend) -> Extractor {
        Extractor {
            extract_url_without_protocol: true,
            parser_backend,
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
        // Use optimized path for Nom backend - skip Entity creation entirely
        if self.parser_backend == ParserBackend::Nom {
            // Early exit if no dollar sign present
            if !s.contains('$') {
                return Vec::new();
            }
            let nom_entities = nom_parser::parse_cashtags_only(s);
            return nom_entities
                .into_iter()
                .map(|e| {
                    // Strip the $ prefix
                    String::from(&e.value[1..])
                })
                .collect();
        }

        self.extract_cashtags_with_indices(s)
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
    }

    /// Extract all usernames from the text. The same
    /// as [Extract::extract_mentioned_screennames_with_indices], but included for compatibility.
    pub fn extract_mentioned_screennames(&self, s: &str) -> Vec<String> {
        // Use optimized path for Nom backend - skip Entity creation entirely
        if self.parser_backend == ParserBackend::Nom {
            // Early exit if no at sign present
            if !s.contains('@') && !s.contains('＠') {
                return Vec::new();
            }
            let nom_entities = nom_parser::parse_mentions_only(s);
            return nom_entities
                .into_iter()
                .filter(|e| e.entity_type == NomEntityType::Username)
                .map(|e| {
                    // Strip the @ prefix
                    let value = &e.value[calculate_offset(e.value)..];
                    String::from(value)
                })
                .collect();
        }

        self.extract_mentioned_screennames_with_indices(s)
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
    }

    /// Extract all federated mentions from the text (Mastodon-style @user@domain.tld).
    pub fn extract_federated_mentions(&self, s: &str) -> Vec<String> {
        self.extract_federated_mentions_with_indices(s)
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

    fn get_parser_backend(&self) -> ParserBackend {
        self.parser_backend
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

    /// Optimized mention extraction using specialized parser.
    /// Only available when using Nom backend.
    fn extract_mentioned_screennames_with_indices(&self, s: &'a str) -> Vec<Entity<'a>> {
        // Early exit if no at sign present
        if !s.contains('@') && !s.contains('＠') {
            return Vec::new();
        }

        // Use optimized parser for Nom backend
        if self.parser_backend == ParserBackend::Nom {
            let nom_entities = nom_parser::parse_mentions_only(s);
            let mut entities = Vec::with_capacity(nom_entities.len());
            let mut iter = s.char_indices().peekable();
            let mut start_index: i32 = 0;

            for entity in nom_entities {
                // Only include usernames, not lists or federated mentions
                if entity.entity_type != NomEntityType::Username {
                    continue;
                }
                // Calculate UTF-16 indices
                start_index += self.scan(iter.by_ref(), entity.start);
                let end_index = start_index + self.scan(iter.by_ref(), entity.end);
                // Strip the @ prefix from the value
                let value = &entity.value[calculate_offset(entity.value)..];
                entities.push(Entity::new(Type::MENTION, value, start_index, end_index));
                start_index = end_index;
            }
            return entities;
        }

        // Fall back to generic extraction for other backends
        self.extract(s, |r| r == Rule::username)
    }

    /// Optimized cashtag extraction using specialized parser.
    /// Only available when using Nom backend.
    fn extract_cashtags_with_indices(&self, s: &'a str) -> Vec<Entity<'a>> {
        // Early exit if no dollar sign present
        if !s.contains('$') {
            return Vec::new();
        }

        // Use optimized parser for Nom backend
        if self.parser_backend == ParserBackend::Nom {
            let nom_entities = nom_parser::parse_cashtags_only(s);
            let mut entities = Vec::with_capacity(nom_entities.len());
            let mut iter = s.char_indices().peekable();
            let mut start_index: i32 = 0;

            for entity in nom_entities {
                // Calculate UTF-16 indices
                start_index += self.scan(iter.by_ref(), entity.start);
                let end_index = start_index + self.scan(iter.by_ref(), entity.end);
                // Strip the $ prefix from the value
                let value = &entity.value[1..];
                entities.push(Entity::new(Type::CASHTAG, value, start_index, end_index));
                start_index = end_index;
            }
            return entities;
        }

        // Fall back to generic extraction for other backends
        self.extract(s, |r| r == Rule::cashtag)
    }
}

/**
 * An [Extract] implementation that extracts entities and provides [TwitterTextParseResults] validation data.
 */
pub struct ValidatingExtractor<'a> {
    extract_url_without_protocol: bool,
    parser_backend: ParserBackend,
    config: &'a Configuration,
    ld: LengthData,
}

impl<'a> ValidatingExtractor<'a> {
    /// Create a new Extractor with the default TLD matcher (External/phf).
    /// [ValidatingExtractor::prep_input] must be called prior to extract.
    pub fn new(configuration: &Configuration) -> ValidatingExtractor<'_> {
        ValidatingExtractor {
            extract_url_without_protocol: true,
            parser_backend: ParserBackend::default(),
            config: configuration,
            ld: LengthData::empty(),
        }
    }

    /// Create a new Extractor with the specified TLD matcher.
    /// [ValidatingExtractor::prep_input] must be called prior to extract.
    pub fn with_parser_backend(
        configuration: &Configuration,
        parser_backend: ParserBackend,
    ) -> ValidatingExtractor<'_> {
        ValidatingExtractor {
            extract_url_without_protocol: true,
            parser_backend,
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
            parser_backend: ParserBackend::default(),
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
    pub fn new_with_nfc_input_and_parser_backend(
        configuration: &'a Configuration,
        s: &str,
        parser_backend: ParserBackend,
    ) -> ValidatingExtractor<'a> {
        let (length, length_utf8) = calculate_length(s);
        ValidatingExtractor {
            extract_url_without_protocol: true,
            parser_backend,
            config: configuration,
            ld: LengthData {
                normalized_length: length,
                normalized_length_utf8: length_utf8,
                original_length: length,
                original_length_utf8: length_utf8,
            },
        }
    }

    /// Extract all federated mentions from the text (Mastodon-style @user@domain.tld).
    /// Returns both regular mentions (@user) and federated mentions (@user@domain).
    pub fn extract_federated_mentions(&self, s: &'a str) -> Vec<String> {
        self.extract_federated_mentions_with_indices(s)
            .entities
            .iter()
            .map(|entity| String::from(entity.get_value()))
            .collect()
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

    fn get_parser_backend(&self) -> ParserBackend {
        self.parser_backend
    }

    fn extract(&self, s: &'a str, r_match: RuleMatch) -> Self::T {
        self.extract_impl(s, r_match)
    }

    // Override to skip early exit - ValidatingExtractor must always scan full text
    // to calculate weighted length, even when no URLs are present.
    fn extract_urls_with_indices(&self, s: &'a str) -> Self::T {
        if self.get_extract_url_without_protocol() {
            self.extract(s, |r| r == Rule::url || r == Rule::url_without_protocol)
        } else {
            self.extract(s, |r| r == Rule::url)
        }
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
    /// Cached weight for fast path (code points 0-4351), or None if no fast path available
    fast_path_weight: Option<i32>,
    config: &'a Configuration,
}

impl<'a> TextMetrics<'a> {
    fn new(config: &Configuration, normalized_length: i32) -> TextMetrics<'_> {
        // Pre-compute fast path weight if first range starts at 0
        let fast_path_weight = config.ranges.first().and_then(|r| {
            if r.range.start() == 0 {
                Some(r.weight)
            } else {
                None
            }
        });
        TextMetrics {
            is_valid: true,
            weighted_count: 0,
            offset: 0,
            valid_offset: 0,
            normalized_length,
            scaled_max_weighted_tweet_length: config.max_weighted_tweet_length * config.scale,
            fast_path_weight,
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
            // Fast path: use cached weight for code points 0-4351 (ASCII, Latin-1, common scripts)
            let char_weight = if let Some(weight) = self.fast_path_weight {
                if code_point <= 4351 {
                    weight
                } else {
                    self.weight_for_code_point(code_point)
                }
            } else {
                // No fast path (v1 config or unusual ranges) - use default weight
                self.config.default_weight
            };
            self.weighted_count += char_weight;
            self.add_char(c);
        }
    }

    #[cold]
    #[inline(never)]
    fn weight_for_code_point(&self, code_point: i32) -> i32 {
        for range in self.config.ranges.iter() {
            if range.contains(code_point) {
                return range.weight;
            }
        }
        self.config.default_weight
    }

    fn scan(&mut self, iter: &mut Peekable<CharIndices>, limit: usize, action: TrackAction) -> i32 {
        let mut offset: i32 = 0;

        match action {
            TrackAction::Text => {
                while let Some(&(pos, c)) = iter.peek() {
                    if pos >= limit {
                        break;
                    }
                    iter.next();
                    offset += as_i32(c.len_utf16());
                    self.track_text(c);
                }
            }
            TrackAction::Emoji => {
                while let Some(&(pos, c)) = iter.peek() {
                    if pos >= limit {
                        break;
                    }
                    iter.next();
                    offset += as_i32(c.len_utf16());
                    self.track_emoji(c);
                }
            }
            TrackAction::Url => {
                while let Some(&(pos, c)) = iter.peek() {
                    if pos >= limit {
                        break;
                    }
                    iter.next();
                    offset += as_i32(c.len_utf16());
                }
                self.track_url(offset);
            }
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
    FullPestPair(FullPestPair<'a>),
    NomEntity(NomEntity<'a>),
}

impl<'a> UnprocessedEntity<'a> {
    fn start(&self) -> usize {
        match self {
            UnprocessedEntity::UrlSpan(span) => span.start(),
            UnprocessedEntity::Pair(pair) => pair.as_span().start(),
            UnprocessedEntity::FullPestPair(pair) => pair.as_span().start(),
            UnprocessedEntity::NomEntity(entity) => entity.start,
        }
    }

    fn end(&self) -> usize {
        match self {
            UnprocessedEntity::UrlSpan(span) => span.end(),
            UnprocessedEntity::Pair(pair) => pair.as_span().end(),
            UnprocessedEntity::FullPestPair(pair) => pair.as_span().end(),
            UnprocessedEntity::NomEntity(entity) => entity.end,
        }
    }

    fn as_rule(&self) -> Rule {
        match self {
            UnprocessedEntity::UrlSpan(_span) => Rule::url,
            UnprocessedEntity::Pair(pair) => pair.as_rule(),
            // Convert FullPestRule to Rule - they have the same variant names
            UnprocessedEntity::FullPestPair(pair) => full_pest_rule_to_rule(pair.as_rule()),
            // Convert NomEntityType to Rule
            UnprocessedEntity::NomEntity(entity) => nom_entity_type_to_rule(entity.entity_type),
        }
    }
}

/// Convert a FullPestRule to the equivalent Rule.
/// Both enums have the same variant names, just generated from different grammars.
fn full_pest_rule_to_rule(r: FullPestRule) -> Rule {
    match r {
        FullPestRule::url => Rule::url,
        FullPestRule::url_without_protocol => Rule::url_without_protocol,
        FullPestRule::hashtag => Rule::hashtag,
        FullPestRule::cashtag => Rule::cashtag,
        FullPestRule::username => Rule::username,
        FullPestRule::list => Rule::list,
        FullPestRule::listname => Rule::listname,
        FullPestRule::list_slug => Rule::list_slug,
        FullPestRule::invalid_char => Rule::invalid_char,
        FullPestRule::emoji => Rule::emoji,
        FullPestRule::federated_mention => Rule::federated_mention,
        _ => Rule::tweet, // fallback for rules we don't use directly
    }
}

/// Convert a RuleMatch function to work with FullPestRule.
fn convert_rule_match(r_match: RuleMatch) -> impl Fn(FullPestRule) -> bool {
    move |r: FullPestRule| {
        let equivalent_rule = full_pest_rule_to_rule(r);
        r_match(equivalent_rule)
    }
}

/// Convert a NomEntityType to the equivalent Rule.
fn nom_entity_type_to_rule(t: NomEntityType) -> Rule {
    match t {
        NomEntityType::Url => Rule::url,
        NomEntityType::UrlWithoutProtocol => Rule::url_without_protocol,
        NomEntityType::Hashtag => Rule::hashtag,
        NomEntityType::Cashtag => Rule::cashtag,
        NomEntityType::Username => Rule::username,
        NomEntityType::List => Rule::list,
        NomEntityType::FederatedMention => Rule::federated_mention,
        NomEntityType::Emoji => Rule::emoji,
        NomEntityType::InvalidChar => Rule::invalid_char,
    }
}

/// Validates a URL parsed with the full Pest grammar.
/// Since the grammar already validated the TLD, we only need to check punycode validity.
fn validate_url_full_pest(p: &FullPestPair) -> bool {
    let original = p.as_str();
    match p.clone().into_inner().find(|pair| {
        let r = pair.as_rule();
        r == FullPestRule::host || r == FullPestRule::tco_domain || r == FullPestRule::uwp_domain
    }) {
        Some(pair) => valid_punycode_full_pest(original, &pair),
        None => false,
    }
}

/// Validates punycode for a domain parsed with the full Pest grammar.
fn valid_punycode_full_pest(original: &str, domain: &FullPestPair) -> bool {
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
            domain.as_rule() != FullPestRule::uwp_domain,
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
/// The `parser_backend` parameter controls how TLDs are validated:
/// - `ParserBackend::Pest`: Trust the Pest grammar's TLD matching (no external validation)
/// - `ParserBackend::External`: Use phf lookup for O(1) TLD validation
fn validate_url(p: Pair, requires_exact_tld: bool, parser_backend: ParserBackend) -> Option<usize> {
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
            if parser_backend == ParserBackend::Pest {
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
                        if is_valid_tld_case_insensitive(tld) {
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

/// Validates a URL parsed by the nom parser.
/// Returns Some(trim_bytes) if valid (0 means no trimming needed),
/// or None if the URL is invalid.
fn validate_url_nom(entity: &NomEntity, requires_exact_tld: bool) -> Option<usize> {
    let original = entity.value;

    // Get the host/domain portion using the stored positions
    let (host_start, host_end) = match (entity.host_start, entity.host_end) {
        (Some(hs), Some(he)) => (hs - entity.start, he - entity.start),
        _ => return None,
    };

    // Bounds check
    if host_end > original.len() || host_start > host_end {
        return None;
    }

    let domain = &original[host_start..host_end];

    // For t.co, skip TLD validation - it's hardcoded
    if domain == "t.co" {
        return if valid_punycode_str(original, domain) {
            Some(0)
        } else {
            None
        };
    }

    // For URLs without protocol, check for script mixing in domain labels
    if requires_exact_tld {
        let valid_domain_end = find_valid_domain_end(domain);
        let domain_trim = domain.len() - valid_domain_end;

        if domain_trim > 0 {
            let trimmed_domain = &domain[..valid_domain_end];
            if let Some(last_dot) = trimmed_domain.rfind('.') {
                let tld = &trimmed_domain[last_dot + 1..];
                if is_valid_tld_case_insensitive(tld) {
                    let after_domain = original.len() - host_end;
                    let total_trim = domain_trim + after_domain;
                    return Some(total_trim);
                }
            }
            return None;
        }
    }

    // Validate TLD by finding the valid boundary
    match find_valid_tld_boundary(domain, requires_exact_tld) {
        Some(valid_domain_len) => {
            let domain_trim = domain.len() - valid_domain_len;

            if domain_trim == 0 {
                // Full domain is valid
                if valid_punycode_str(original, domain) {
                    Some(0)
                } else {
                    None
                }
            } else {
                // Need to trim domain - also trim everything after the domain
                let after_domain = original.len() - host_end;
                let total_trim = domain_trim + after_domain;
                Some(total_trim)
            }
        }
        None => None,
    }
}

/// Validate punycode for a domain string (used by nom parser validation).
fn valid_punycode_str(original: &str, domain: &str) -> bool {
    let uts46 = Uts46::new();

    let result = uts46.to_ascii(
        domain.as_bytes(),
        AsciiDenyList::EMPTY,
        Hyphens::Allow,
        DnsLength::Verify,
    );

    match result {
        Ok(s) => {
            // Check that the ASCII form isn't too long for the URL
            let has_protocol = original.starts_with("http://") || original.starts_with("https://");
            length_check(original, domain, &s, has_protocol)
        }
        Err(_) => false,
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

                if !effective_segment.is_empty() && is_valid_tld_case_insensitive(effective_segment)
                {
                    let end_pos = dot_pos + 1 + effective_segment.len();
                    #[cfg(test)]
                    eprintln!(
                        "find_valid_tld_boundary: found valid TLD '{}' at script boundary, position {}",
                        effective_segment, end_pos
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

        // Check if this segment is a valid TLD (case-insensitive, no heap allocation)
        if is_valid_tld_case_insensitive(segment) {
            // Check if the segment before this TLD contains underscores
            // If so, this is not a valid URL (domain_segment can't have underscores)
            let before_dot = &domain[..dot_pos];
            let prev_segment = before_dot.rsplit('.').next().unwrap_or(before_dot);
            if prev_segment.contains('_') {
                // The segment before the TLD has an underscore - skip this TLD
                #[cfg(test)]
                eprintln!(
                    "find_valid_tld_boundary: rejecting TLD '{}' because prev segment '{}' contains underscore",
                    segment, prev_segment
                );
                continue;
            }

            let end_pos = dot_pos + 1 + segment.len();
            #[cfg(test)]
            eprintln!(
                "find_valid_tld_boundary: found valid TLD '{}' at position {}",
                segment, end_pos
            );
            return Some(end_pos);
        }

        // Check if a valid TLD is a prefix of this segment followed by hyphen
        // This handles cases like "domain.com-that-you..." where "com" is valid
        // but "com-that-you..." is not.
        if let Some(hyphen_pos) = segment.find('-') {
            let before_hyphen = &segment[..hyphen_pos];
            if is_valid_tld_case_insensitive(before_hyphen) {
                // Also check if the segment before this TLD contains underscores
                let before_dot = &domain[..dot_pos];
                let prev_segment = before_dot.rsplit('.').next().unwrap_or(before_dot);
                if prev_segment.contains('_') {
                    #[cfg(test)]
                    eprintln!(
                        "find_valid_tld_boundary: rejecting TLD '{}' (before hyphen) because prev segment '{}' contains underscore",
                        before_hyphen, prev_segment
                    );
                    continue;
                }

                let end_pos = dot_pos + 1 + before_hyphen.len();
                #[cfg(test)]
                eprintln!(
                    "find_valid_tld_boundary: found valid TLD '{}' before hyphen at position {}",
                    before_hyphen, end_pos
                );
                return Some(end_pos);
            }
        }

        // For URLs without protocol (requires_exact_tld=true), we need prefix matching
        // for Unicode TLDs like みんな from みんなです.
        // For URLs with protocol, we don't do prefix matching (grammar already determined the TLD).
        if requires_exact_tld && !segment.is_ascii() {
            // Check if a valid Unicode TLD is a prefix of this segment
            // We try progressively shorter prefixes
            for (char_idx, _) in segment.char_indices().skip(2) {
                let prefix = &segment[..char_idx];
                if is_valid_tld_case_insensitive(prefix) {
                    let end_pos = dot_pos + 1 + prefix.len();
                    #[cfg(test)]
                    eprintln!(
                        "find_valid_tld_boundary: found valid Unicode TLD prefix '{}' at position {}",
                        prefix, end_pos
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

    // Federated mention tests (Mastodon-style @user@domain.tld)
    // These tests run on all backends that support federated mentions.
    fn federated_mention_backends() -> Vec<ParserBackend> {
        vec![
            ParserBackend::Pest,
            ParserBackend::External,
            ParserBackend::Nom,
        ]
    }

    #[test]
    fn test_federated_mention_empty_string() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted = extractor.extract_federated_mentions("");
            assert_eq!(0, extracted.len(), "Failed for {:?}", validator);
        }
    }

    #[test]
    fn test_federated_mention_no_at_signs() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted = extractor.extract_federated_mentions("a string without at signs");
            assert_eq!(0, extracted.len(), "Failed for {:?}", validator);
        }
    }

    #[test]
    fn test_federated_mention_simple() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted = extractor.extract_federated_mentions("@user@domain.tld");
            assert_eq!(
                vec!["@user@domain.tld"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_federated_mention_in_text() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted =
                extractor.extract_federated_mentions("hello @user@mastodon.social world");
            assert_eq!(
                vec!["@user@mastodon.social"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_federated_mention_with_indices() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted = extractor
                .extract_federated_mentions_with_indices("hello @user@mastodon.social world");
            assert_eq!(1, extracted.len(), "Failed for {:?}", validator);
            assert_eq!(
                "@user@mastodon.social", extracted[0].value,
                "Failed for {:?}",
                validator
            );
            assert_eq!(6, extracted[0].start, "Failed for {:?}", validator);
            assert_eq!(27, extracted[0].end, "Failed for {:?}", validator);
        }
    }

    #[test]
    fn test_federated_mention_complex_username() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            // Username with dots and hyphens
            let extracted = extractor.extract_federated_mentions("@user.name-test@domain.tld");
            assert_eq!(
                vec!["@user.name-test@domain.tld"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_federated_mention_complex_domain() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            // Domain with subdomains
            let extracted = extractor.extract_federated_mentions("@user@sub.domain.example.com");
            assert_eq!(
                vec!["@user@sub.domain.example.com"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_federated_mention_multiple() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted =
                extractor.extract_federated_mentions("@user1@domain1.com and @user2@domain2.org");
            assert_eq!(
                vec!["@user1@domain1.com", "@user2@domain2.org"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_federated_mention_with_underscore() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted = extractor.extract_federated_mentions("@user_name@domain.tld");
            assert_eq!(
                vec!["@user_name@domain.tld"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_federated_mention_fullwidth_at() {
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            // Full-width @ (U+FF20) as prefix
            let extracted = extractor.extract_federated_mentions("＠user@domain.tld");
            assert_eq!(
                vec!["＠user@domain.tld"],
                extracted,
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_local_mention_included_in_federated() {
        // extract_federated_mentions now returns both regular and federated mentions
        // (matching Mastodon's behavior)
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let extracted = extractor.extract_federated_mentions("@localuser");
            assert_eq!(1, extracted.len(), "Failed for {:?}", validator);
            assert_eq!("localuser", extracted[0], "Failed for {:?}", validator);
        }
    }

    #[test]
    fn test_mentions_with_federated_style_text() {
        // extract_mentions should ignore federated-style mentions (@user@domain)
        // Only regular mentions are extracted
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let text = "test @sayrer @user1@domain1.com and @user2@domain2.org";
            let mentions = extractor.extract_mentions_or_lists_with_indices(text);
            // Only @sayrer is extracted; @user1@domain1.com and @user2@domain2.org are ignored
            assert_eq!(1, mentions.len(), "Failed for {:?}", validator);
            assert_eq!("sayrer", mentions[0].value, "Failed for {:?}", validator);
            assert_eq!(5, mentions[0].start, "Failed for {:?}", validator);
            assert_eq!(12, mentions[0].end, "Failed for {:?}", validator);
        }
    }

    #[test]
    fn test_extract_federated_mentions_mixed_text() {
        // extract_federated_mentions returns both regular and federated mentions
        // (matching Mastodon's behavior)
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let text = "test @sayrer @user1@domain1.com and @user2@domain2.org";
            let federated = extractor.extract_federated_mentions(text);
            // Should get all three: @sayrer (regular) and the two federated ones
            assert_eq!(3, federated.len(), "Failed for {:?}", validator);
            assert_eq!("sayrer", federated[0], "Failed for {:?}", validator);
            assert_eq!(
                "@user1@domain1.com", federated[1],
                "Failed for {:?}",
                validator
            );
            assert_eq!(
                "@user2@domain2.org", federated[2],
                "Failed for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_extract_entities_excludes_federated_mentions() {
        // extract_entities_with_indices should return all entity types EXCEPT federated mentions
        // This matches Twitter's behavior where federated mentions are a Mastodon extension
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let text = "Check https://example.com @user @user/list @fed@mastodon.social #tag $CASH";

            let entities = extractor.extract_entities_with_indices(text);

            // Should have 5 entities: URL, mention, list, hashtag, cashtag (no federated mention)
            assert_eq!(
                5,
                entities.len(),
                "Expected 5 entities for {:?}, got {:?}",
                validator,
                entities
            );

            // Verify each entity type is present (list is also Type::MENTION)
            let url = entities.iter().find(|e| e.t == Type::URL).unwrap();
            assert_eq!(
                "https://example.com", url.value,
                "Failed for {:?}",
                validator
            );

            // Find the two mentions (regular and list)
            let mentions: Vec<_> = entities.iter().filter(|e| e.t == Type::MENTION).collect();
            assert_eq!(2, mentions.len(), "Expected 2 mentions for {:?}", validator);

            // Regular mention (empty list_slug)
            let regular_mention = mentions.iter().find(|e| e.list_slug.is_empty()).unwrap();
            assert_eq!("user", regular_mention.value, "Failed for {:?}", validator);

            // List mention (non-empty list_slug)
            let list_mention = mentions.iter().find(|e| !e.list_slug.is_empty()).unwrap();
            assert_eq!("user", list_mention.value, "Failed for {:?}", validator);
            assert_eq!(
                "/list", list_mention.list_slug,
                "Failed for {:?}",
                validator
            );

            let hashtag = entities.iter().find(|e| e.t == Type::HASHTAG).unwrap();
            assert_eq!("tag", hashtag.value, "Failed for {:?}", validator);

            let cashtag = entities.iter().find(|e| e.t == Type::CASHTAG).unwrap();
            assert_eq!("CASH", cashtag.value, "Failed for {:?}", validator);

            // Verify federated mention is NOT present
            assert!(
                !entities.iter().any(|e| e.t == Type::FEDERATEDMENTION),
                "FEDERATEDMENTION should not be in extract_entities_with_indices for {:?}",
                validator
            );
        }
    }

    #[test]
    fn test_validating_extractor_federated_mentions() {
        // Test that ValidatingExtractor can extract federated mentions
        // and returns correct parse results
        let config = Configuration::default();
        let text = "Hello @local and @user@mastodon.social!";

        let extractor = ValidatingExtractor::new_with_nfc_input(&config, text);
        let mentions = extractor.extract_federated_mentions(text);

        // Should extract both regular and federated mentions
        assert_eq!(2, mentions.len());
        assert_eq!("local", mentions[0]);
        assert_eq!("@user@mastodon.social", mentions[1]);

        // Also test with_indices to verify parse results
        let result = extractor.extract_federated_mentions_with_indices(text);

        // Verify parse results are calculated correctly
        assert!(result.parse_results.is_valid);
        assert_eq!(39, result.parse_results.weighted_length); // All ASCII, 1 weight each

        // Verify entities
        assert_eq!(2, result.entities.len());
        assert_eq!(Type::MENTION, result.entities[0].t);
        assert_eq!("local", result.entities[0].value);
        assert_eq!(Type::FEDERATEDMENTION, result.entities[1].t);
        assert_eq!("@user@mastodon.social", result.entities[1].value);
    }

    #[test]
    fn test_extract_entities_with_indices_federated() {
        // Test that extract_entities_with_indices_federated includes all entity types
        // including federated mentions
        for validator in federated_mention_backends() {
            let extractor = Extractor::with_parser_backend(validator);
            let text = "Check https://example.com @user @user/list @fed@mastodon.social #tag $CASH";

            let entities = extractor.extract_entities_with_indices_federated(text);

            // Should have 6 entities: URL, mention, list, federated mention, hashtag, cashtag
            assert_eq!(
                6,
                entities.len(),
                "Expected 6 entities for {:?}, got {:?}",
                validator,
                entities
            );

            // Verify each entity type is present
            let url = entities.iter().find(|e| e.t == Type::URL).unwrap();
            assert_eq!(
                "https://example.com", url.value,
                "Failed for {:?}",
                validator
            );

            // Find mentions (regular and list)
            let mentions: Vec<_> = entities.iter().filter(|e| e.t == Type::MENTION).collect();
            assert_eq!(2, mentions.len(), "Expected 2 mentions for {:?}", validator);

            // Federated mention IS present
            let federated = entities
                .iter()
                .find(|e| e.t == Type::FEDERATEDMENTION)
                .unwrap();
            assert_eq!(
                "@fed@mastodon.social", federated.value,
                "Failed for {:?}",
                validator
            );

            let hashtag = entities.iter().find(|e| e.t == Type::HASHTAG).unwrap();
            assert_eq!("tag", hashtag.value, "Failed for {:?}", validator);

            let cashtag = entities.iter().find(|e| e.t == Type::CASHTAG).unwrap();
            assert_eq!("CASH", cashtag.value, "Failed for {:?}", validator);
        }
    }

    #[test]
    fn test_validating_extractor_entities_federated() {
        // Test ValidatingExtractor with extract_entities_with_indices_federated
        let config = Configuration::default();
        let text = "Check https://example.com @user @fed@mastodon.social #tag $CASH";

        let extractor = ValidatingExtractor::new_with_nfc_input(&config, text);
        let result = extractor.extract_entities_with_indices_federated(text);

        // Verify parse results
        assert!(result.parse_results.is_valid);

        // Should have 5 entities: URL, mention, federated mention, hashtag, cashtag
        assert_eq!(5, result.entities.len());

        // Verify federated mention is included
        let federated = result
            .entities
            .iter()
            .find(|e| e.t == Type::FEDERATEDMENTION);
        assert!(federated.is_some());
        assert_eq!("@fed@mastodon.social", federated.unwrap().value);
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

    // ParserBackend backend tests

    #[test]
    fn test_parser_backend_external_validates_tlds() {
        // External backend should reject invalid TLDs
        let extractor = Extractor::with_parser_backend(ParserBackend::External);

        // Valid TLD should be extracted
        let urls = extractor.extract_urls("Check out http://example.com/path");
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "http://example.com/path");

        // URL with valid punycode TLD
        let urls = extractor.extract_urls("See http://xn--80abe5aohbnkjb.xn--p1ai/");
        assert_eq!(urls.len(), 1);
    }

    #[test]
    fn test_parser_backend_pest_trusts_grammar() {
        // Pest backend trusts whatever the grammar matches
        let extractor = Extractor::with_parser_backend(ParserBackend::Pest);

        // With the permissive grammar, this will also match
        let urls = extractor.extract_urls("Check out http://example.com/path");
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], "http://example.com/path");
    }

    #[test]
    fn test_parser_backend_default_is_nom() {
        // Default should be Nom
        assert_eq!(ParserBackend::default(), ParserBackend::Nom);

        // New extractor should use Nom by default
        let extractor = Extractor::new();
        assert_eq!(extractor.get_parser_backend(), ParserBackend::Nom);
    }

    #[test]
    fn test_all_backends_produce_same_results() {
        // Test that all three backends produce identical results
        let external = Extractor::with_parser_backend(ParserBackend::External);
        let pest = Extractor::with_parser_backend(ParserBackend::Pest);
        let nom = Extractor::with_parser_backend(ParserBackend::Nom);

        let test_cases = [
            "Check out http://example.com/path",
            "Visit https://twitter.com and https://github.com",
            "See http://xn--80abe5aohbnkjb.xn--p1ai/",
            "URL without protocol: example.com/page",
            "Japanese TLD: http://example.みんな/path",
            "Multiple: foo.co.jp and bar.co.uk are valid",
            "@mention and #hashtag with http://test.org",
            // Hyphenated TLD case
            "text http://domain.com-that-you-should-have-put-a-space-after",
            // Underscore in subdomain (valid)
            "test http://sub_domain-dash.twitter.com",
            // Underscore before TLD (invalid - should NOT extract)
            "text http://domain-dash_2314352345_dfasd.foo-cow_4352.com",
            // Leading dash in subdomain (invalid)
            "test http://-leadingdash.twitter.com",
            // CJK characters surrounding URLs
            "http://twitter.com/これは日本語です。example.com中国語http://t.co/abcde한국twitter.comテストexample2.comテストhttp://twitter.com/abcde",
            // URLs in hashtag or mention (should NOT extract)
            "#test.com @test.com #http://test.com @http://test.com #t.co/abcde @t.co/abcde",
            // Domain followed by Japanese characters
            "example.comてすとですtwitter.みんなです",
            // URL preceded by $ (should NOT extract)
            "$http://twitter.com $twitter.com $http://t.co/abcde $t.co/abcde $t.co $TVI.CA $RBS.CA",
            // Email addresses (should NOT extract)
            "john.doe.gov@mail.com",
            "john.doe.jp@mail.com",
            // URLs with trailing punctuation (should extract URL without punctuation)
            "text http://example.com.",
            "text http://example.com?",
            "text http://example.com!",
        ];

        for text in test_cases {
            let external_urls = external.extract_urls(text);
            let pest_urls = pest.extract_urls(text);
            let nom_urls = nom.extract_urls(text);

            assert_eq!(
                external_urls, pest_urls,
                "External vs Pest differ on: {}\nExternal: {:?}\nPest: {:?}",
                text, external_urls, pest_urls
            );
            assert_eq!(
                external_urls, nom_urls,
                "External vs Nom differ on: {}\nExternal: {:?}\nNom: {:?}",
                text, external_urls, nom_urls
            );
        }
    }

    #[test]
    fn test_emoji_weighting_v2_v3() {
        // Test emoji weighting with v2 and v3 configs
        let config_v2 = twitter_text_config::config_v2();
        let config_v3 = twitter_text_config::config_v3();
        let text = "H🐱☺👨‍👩‍👧‍👦";

        eprintln!("Text: {:?}", text);
        eprintln!(
            "emoji_parsing_enabled v2: {}",
            config_v2.emoji_parsing_enabled
        );
        eprintln!(
            "emoji_parsing_enabled v3: {}",
            config_v3.emoji_parsing_enabled
        );

        let ext_v2 =
            crate::parse_with_parser_backend(text, config_v2, false, ParserBackend::External);
        let nom_v2 = crate::parse_with_parser_backend(text, config_v2, false, ParserBackend::Nom);

        eprintln!("\nV2 config:");
        eprintln!("  External: weighted_length = {}", ext_v2.weighted_length);
        eprintln!("  Nom:      weighted_length = {}", nom_v2.weighted_length);

        let ext_v3 =
            crate::parse_with_parser_backend(text, config_v3, false, ParserBackend::External);
        let nom_v3 = crate::parse_with_parser_backend(text, config_v3, false, ParserBackend::Nom);

        eprintln!("\nV3 config:");
        eprintln!("  External: weighted_length = {}", ext_v3.weighted_length);
        eprintln!("  Nom:      weighted_length = {}", nom_v3.weighted_length);

        assert_eq!(
            ext_v2.weighted_length, nom_v2.weighted_length,
            "V2 mismatch"
        );
        assert_eq!(
            ext_v3.weighted_length, nom_v3.weighted_length,
            "V3 mismatch"
        );
    }

    #[test]
    fn test_star_emoji_with_variation_selector() {
        // Test that ⭐️ (U+2B50 + U+FE0F) is counted as a single emoji
        // This matches Old JS twitter-text behavior where weighted_length = 72
        let config = twitter_text_config::config_v3();
        let text = "🔥🔥🔥 This is amazing! 💯💯💯 Best day ever! 🚀🚀🚀 To the moon! 🌙✨⭐️";

        let pest = crate::parse_with_parser_backend(text, config, false, ParserBackend::Pest);
        let ext = crate::parse_with_parser_backend(text, config, false, ParserBackend::External);
        let nom = crate::parse_with_parser_backend(text, config, false, ParserBackend::Nom);

        // Expected: 72 (matching Old JS behavior)
        // - 12 emoji (🔥🔥🔥💯💯💯🚀🚀🚀🌙✨⭐️) each count as 1
        // - Text and spaces weighted by character weight
        assert_eq!(
            pest.weighted_length, 72,
            "Pest weighted_length should be 72"
        );
        assert_eq!(
            ext.weighted_length, 72,
            "External weighted_length should be 72"
        );
        assert_eq!(nom.weighted_length, 72, "Nom weighted_length should be 72");
    }

    #[test]
    fn test_pest_backend_extracts_punycode_urls() {
        let extractor = Extractor::with_parser_backend(ParserBackend::Pest);

        // Punycode URL should be extracted
        let urls = extractor.extract_urls("See http://xn--80abe5aohbnkjb.xn--p1ai/");
        assert_eq!(urls.len(), 1, "Pest backend should extract punycode URLs");
        assert_eq!(urls[0], "http://xn--80abe5aohbnkjb.xn--p1ai/");
    }

    #[test]
    fn test_nom_hyphenated_tld_debug() {
        let text = "text http://domain-dash_2314352345_dfasd.foo-cow_4352.com";
        eprintln!("Input: {}", text);

        // First, let's see what Pest parses
        match TwitterTextParser::parse(Rule::tweet, text) {
            Ok(p) => {
                for pair in p.flatten() {
                    let r = pair.as_rule();
                    if r == Rule::url || r == Rule::host || r == Rule::url_without_protocol {
                        eprintln!("Pest {:?}: '{}'", r, pair.as_str());
                    }
                }
            }
            Err(e) => eprintln!("Pest parse error: {}", e),
        }
        eprintln!("");

        // Test with Nom parser
        let nom_entities = nom_parser::parse_tweet(text);
        eprintln!("Nom entities: {:?}", nom_entities);

        for entity in &nom_entities {
            eprintln!(
                "Entity: {:?} value='{}' host_start={:?} host_end={:?}",
                entity.entity_type, entity.value, entity.host_start, entity.host_end
            );

            if entity.entity_type == NomEntityType::Url {
                if let (Some(hs), Some(he)) = (entity.host_start, entity.host_end) {
                    let host_in_value_start = hs - entity.start;
                    let host_in_value_end = he - entity.start;
                    if host_in_value_end <= entity.value.len() {
                        let domain = &entity.value[host_in_value_start..host_in_value_end];
                        eprintln!("Domain extracted: '{}'", domain);

                        let boundary = find_valid_tld_boundary(domain, false);
                        eprintln!("find_valid_tld_boundary result: {:?}", boundary);

                        let trim_result = validate_url_nom(entity, false);
                        eprintln!("validate_url_nom result: {:?}", trim_result);
                    }
                }
            }
        }

        // Now test the full extractor
        let extractor = Extractor::with_parser_backend(ParserBackend::Nom);
        let urls = extractor.extract_urls(text);
        eprintln!("Extracted URLs: {:?}", urls);

        // Compare with External
        let external = Extractor::with_parser_backend(ParserBackend::External);
        let external_urls = external.extract_urls(text);
        eprintln!("External URLs: {:?}", external_urls);
    }

    #[test]
    fn test_emojis_crate_lookup() {
        // Simple emoji
        assert!(emojis::get("😀").is_some(), "Simple emoji should be found");

        // Family emoji (ZWJ sequence)
        assert!(emojis::get("👨‍👩‍👧‍👦").is_some(), "Family emoji should be found");

        // Skin tone emoji
        assert!(
            emojis::get("👋🏽").is_some(),
            "Skin tone emoji should be found"
        );

        // Flag
        assert!(emojis::get("🇺🇸").is_some(), "Flag emoji should be found");

        // Not an emoji
        assert!(
            emojis::get("hello").is_none(),
            "Text should not be found as emoji"
        );
        assert!(
            emojis::get("a").is_none(),
            "Single letter should not be found as emoji"
        );

        // Test star emoji with and without variation selector
        // The emojis crate doesn't find star+FE0F directly, but is_valid_emoji handles it
        let star_with_vs = "\u{2b50}\u{fe0f}"; // ⭐️
        let plain_star = "\u{2b50}"; // ⭐
        assert!(
            emojis::get(plain_star).is_some(),
            "Plain star should be found"
        );
        assert!(
            emojis::get(star_with_vs).is_none(),
            "Star+FE0F not found directly by emojis crate"
        );
        assert!(
            is_valid_emoji(star_with_vs),
            "Star+FE0F should be valid via is_valid_emoji"
        );
        assert!(
            is_valid_emoji(plain_star),
            "Plain star should be valid via is_valid_emoji"
        );
    }
}
