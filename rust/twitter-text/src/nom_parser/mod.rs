// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Nom-based parser for twitter-text.
//!
//! This module provides a high-performance parser using nom combinators
//! instead of Pest's VM-based interpreter. It translates the permissive
//! grammar from twitter_text.pest, with TLD and emoji validation done
//! externally via phf and the emojis crate.

pub mod cashtag;
pub mod common;
pub mod emoji;
pub mod entity;
pub mod hashtag;
pub mod mention;
pub mod url;

pub use entity::{NomEntity, NomEntityType};

/// Parse a tweet and return all entities found.
///
/// This is the main entry point for the nom parser. It scans through
/// the input text and extracts all entities (URLs, mentions, hashtags,
/// cashtags, emoji, and invalid characters).
///
/// # Arguments
/// * `input` - The tweet text to parse
///
/// # Returns
/// A vector of `NomEntity` structs, sorted by start position.
pub fn parse_tweet(input: &str) -> Vec<NomEntity<'_>> {
    // Pre-allocate for typical tweet entity count
    let mut entities = Vec::with_capacity(32);
    let mut pos = 0;
    let bytes = input.as_bytes();

    while pos < bytes.len() {
        let b = bytes[pos];

        // Fast path: common ASCII characters that can't start entities
        // This covers ~80% of typical tweet content
        if b < 128 {
            // ASCII fast path
            if matches!(
                b,
                b' ' | b'.'
                    | b','
                    | b'!'
                    | b'?'
                    | b'\''
                    | b'"'
                    | b'-'
                    | b'_'
                    | b'\n'
                    | b'\r'
                    | b'\t'
                    | b'0'
                    ..=b'9'
                        | b'('
                        | b')'
                        | b'['
                        | b']'
                        | b'{'
                        | b'}'
                        | b':'
                        | b';'
                        | b'<'
                        | b'>'
                        | b'/'
                        | b'\\'
                        | b'|'
                        | b'`'
                        | b'~'
                        | b'='
                        | b'+'
                        | b'*'
                        | b'&'
                        | b'^'
                        | b'%'
            ) {
                pos += 1;
                continue;
            }

            let remaining = &input[pos..];

            // Try to match an entity at this position
            if let Some((entity, consumed)) = try_parse_entity(input, remaining, pos) {
                entities.push(entity);
                pos += consumed;
            } else {
                // Handle special ASCII cases that failed entity parsing
                if b == b'$' {
                    // $ followed by URL-like content should be skipped
                    let after_dollar = &remaining[1..];
                    let skip = skip_invalid_cashtag_url(after_dollar);
                    pos += 1 + skip;
                } else if b == b'@' {
                    // @ followed by URL-like content (email) should skip
                    let after_at = &remaining[1..];
                    let skip = skip_email_domain(after_at);
                    pos += 1 + skip;
                } else {
                    // Regular ASCII char, just skip
                    pos += 1;
                }
            }
        } else {
            // Non-ASCII: need to decode UTF-8
            let remaining = &input[pos..];
            let c = remaining.chars().next().unwrap();
            let char_len = c.len_utf8();

            // Try to match an entity at this position
            if let Some((entity, consumed)) = try_parse_entity(input, remaining, pos) {
                entities.push(entity);
                pos += consumed;
            } else if common::is_invalid_char(c) {
                entities.push(NomEntity::new(
                    NomEntityType::InvalidChar,
                    &remaining[..char_len],
                    pos,
                    pos + char_len,
                ));
                pos += char_len;
            } else if emoji::is_emoji_start(c) {
                // Try to parse a potential emoji sequence
                if let Some((matched, consumed)) = emoji::try_parse_emoji(remaining) {
                    entities.push(NomEntity::new(
                        NomEntityType::Emoji,
                        matched,
                        pos,
                        pos + consumed,
                    ));
                    pos += consumed;
                } else {
                    pos += char_len;
                }
            } else if c == '\u{ff20}' {
                // Fullwidth @ - check for email domain
                let after_at = &remaining[char_len..];
                let skip = skip_email_domain(after_at);
                pos += char_len + skip;
            } else {
                pos += char_len;
            }
        }
    }

    entities
}

/// When we see a $ that doesn't start a valid cashtag, check if what follows
/// looks like a URL and return how many bytes to skip (not including the $).
/// This prevents extracting partial URLs from things like "$twitter.com" or "$http://t.co".
fn skip_invalid_cashtag_url(input: &str) -> usize {
    // Check for protocol URL: $http://... or $https://...
    if let Ok((_, (matched, _, _))) = url::parse_url(input) {
        return matched.len();
    }

    // Check for URL without protocol: $twitter.com
    if let Ok((_, (matched, _, _))) = url::parse_url_without_protocol(input) {
        return matched.len();
    }

    0
}

/// When we see an @ that's not a valid mention (e.g., in an email address),
/// check if what follows looks like a domain and skip it.
/// This prevents extracting partial URLs like "ail.com" from "user@mail.com".
fn skip_email_domain(input: &str) -> usize {
    // Check for URL without protocol pattern (the domain part of email)
    if let Ok((_, (matched, _, _))) = url::parse_url_without_protocol(input) {
        return matched.len();
    }

    0
}

/// Try to parse an entity at the current position.
/// Returns Some((entity, bytes_consumed)) on success, None otherwise.
///
/// # Arguments
/// * `full_input` - The complete tweet text (for checking preceding characters)
/// * `input` - The remaining text starting at the current position
/// * `offset` - The byte offset into full_input where input starts
fn try_parse_entity<'a>(
    full_input: &'a str,
    input: &'a str,
    offset: usize,
) -> Option<(NomEntity<'a>, usize)> {
    // Try each entity type in priority order
    // URLs should come first to avoid hashtag/mention ambiguity in URLs

    let bytes = input.as_bytes();
    let first_byte = *bytes.first()?;

    // Fast path for ASCII first character
    let first_char = if first_byte < 128 {
        first_byte as char
    } else {
        input.chars().next()?
    };

    // Get the character before this position (if any)
    let prev_char = if offset > 0 {
        full_input[..offset].chars().last()
    } else {
        None
    };

    // Check based on first character to avoid unnecessary parsing attempts
    match first_char {
        // @ or fullwidth @
        '@' | '\u{ff20}' => {
            // Check for invalid prefix - mentions shouldn't be preceded by alphanumeric, @, or certain punctuation
            // EXCEPT for the special "RT" prefix case (legacy retweet syntax)
            if let Some(pc) = prev_char {
                let is_invalid_prefix = pc.is_ascii_alphanumeric()
                    || pc == '_'
                    || pc == '@'
                    || pc == '\u{ff20}'
                    || pc == '!'
                    || pc == '#'
                    || pc == '$'
                    || pc == '%'
                    || pc == '&'
                    || pc == '*';

                if is_invalid_prefix {
                    // Check for RT prefix exception: "RT@" or "RT:@" preceded by space/SOI
                    let is_rt_prefix = is_rt_prefix_before(full_input, offset);
                    if !is_rt_prefix {
                        return None;
                    }
                }
            }

            // Try list first (longer match), then username
            if let Ok((remaining, (matched, slug_start_opt))) =
                mention::parse_mention_or_list(input)
            {
                // Check for invalid suffix (@ or latin_accent or :// or -)
                // If followed by these, don't extract the username
                if let Some(next_char) = remaining.chars().next() {
                    if next_char == '@'
                        || next_char == '\u{ff20}'
                        || next_char == '-'
                        || url::is_latin_accent(next_char)
                        || remaining.starts_with("://")
                    {
                        // Invalid suffix - don't extract
                        return None;
                    }
                }

                let consumed = matched.len();
                let entity = if let Some(slug_start) = slug_start_opt {
                    NomEntity::new_list(matched, offset, offset + consumed, offset + slug_start)
                } else {
                    NomEntity::new(NomEntityType::Username, matched, offset, offset + consumed)
                };
                return Some((entity, consumed));
            }
        }

        // # or fullwidth #
        '#' | '\u{ff03}' => {
            // Check for invalid predecessor - hashtags shouldn't be preceded by & or letter/mark
            if let Some(pc) = prev_char {
                if !hashtag::valid_hashtag_predecessor(pc) {
                    return None;
                }
            }

            // Check for invalid hashtag: # followed by URL protocol
            let after_hash = &input[first_char.len_utf8()..];
            if after_hash.starts_with("http://")
                || after_hash.starts_with("https://")
                || after_hash.starts_with("HTTP://")
                || after_hash.starts_with("HTTPS://")
            {
                return None;
            }

            if let Ok((_, matched)) = hashtag::parse_hashtag(input) {
                let consumed = matched.len();
                let entity =
                    NomEntity::new(NomEntityType::Hashtag, matched, offset, offset + consumed);
                return Some((entity, consumed));
            }
        }

        // $
        '$' => {
            // Cashtags must be preceded by space or start of input
            if let Some(pc) = prev_char {
                if !common::is_space(pc) {
                    return None;
                }
            }

            if let Ok((_, matched)) = cashtag::parse_cashtag(input) {
                let consumed = matched.len();
                let entity =
                    NomEntity::new(NomEntityType::Cashtag, matched, offset, offset + consumed);
                return Some((entity, consumed));
            }
            // If cashtag parse failed but what follows looks like a URL, we need to
            // consume it as an "invalid cashtag" to prevent extracting partial URLs.
            // This handles cases like "$http://twitter.com" or "$twitter.com"
            // The grammar does: invalid_cashtag_start = cashtag_prefix ~ protocol ~ subdomain_char*
            // We return None but the caller (parse_tweet) needs to know to skip more than 1 char.
            // For now, we handle this by returning a dummy entity that gets filtered out.
        }

        // URL with protocol (http:// or https://)
        'h' | 'H' => {
            // URLs must NOT be preceded by @, #, or $ (they would be mentions/hashtags/cashtags)
            if let Some(pc) = prev_char {
                if pc == '@' || pc == '#' || pc == '$' || pc == '\u{ff20}' || pc == '\u{ff03}' {
                    return None;
                }
            }

            if let Ok((_, (matched, host_start, host_end))) = url::parse_url(input) {
                let consumed = matched.len();
                let entity = NomEntity::new_url(
                    NomEntityType::Url,
                    matched,
                    offset,
                    offset + consumed,
                    offset + host_start,
                    offset + host_end,
                );
                return Some((entity, consumed));
            }
        }

        // Potential URL without protocol (starts with alphanumeric)
        c if c.is_ascii_alphanumeric() || url::is_latin_accent(c) => {
            // URLs must NOT be preceded by @, #, or $ (they would be mentions/hashtags/cashtags)
            if let Some(pc) = prev_char {
                if pc == '@' || pc == '#' || pc == '$' || pc == '\u{ff20}' || pc == '\u{ff03}' {
                    return None;
                }
            }

            // Don't extract UWP if we're inside a failed protocol URL context
            // Check if "://" appears before us without intervening delimiter
            // Delimiters include spaces and CJK/non-Latin characters
            if offset >= 3 {
                let before = &full_input[..offset];
                // Find the last delimiter before this position
                // A delimiter is a space or a non-URL character (like CJK)
                let last_delim = before
                    .char_indices()
                    .rev()
                    .find(|(_, c)| common::is_space(*c) || is_url_delimiter(*c))
                    .map(|(i, c)| i + c.len_utf8())
                    .unwrap_or(0);
                let since_delim = &before[last_delim..];
                // If there's "://" followed by anything (like "http://-foo"), skip this
                if since_delim.contains("://") {
                    return None;
                }
            }

            // Check if there's a dot before the next space (early rejection)
            if input
                .chars()
                .take_while(|c| !common::is_space(*c))
                .any(|c| c == '.')
            {
                if let Ok((remaining, (matched, host_start, host_end))) =
                    url::parse_url_without_protocol(input)
                {
                    // Don't extract UWP if followed by @ (it's an email address)
                    if remaining.starts_with('@') || remaining.starts_with('\u{ff20}') {
                        return None;
                    }
                    let consumed = matched.len();
                    let entity = NomEntity::new_url(
                        NomEntityType::UrlWithoutProtocol,
                        matched,
                        offset,
                        offset + consumed,
                        offset + host_start,
                        offset + host_end,
                    );
                    return Some((entity, consumed));
                }
            }
        }

        _ => {}
    }

    None
}

/// Check if a character acts as a URL delimiter (breaks URL context).
/// This includes CJK characters, certain punctuation, etc.
fn is_url_delimiter(c: char) -> bool {
    // CJK Unified Ideographs and related blocks
    matches!(c,
        '\u{3000}'..='\u{303F}' |  // CJK Symbols and Punctuation
        '\u{3040}'..='\u{309F}' |  // Hiragana
        '\u{30A0}'..='\u{30FF}' |  // Katakana
        '\u{4E00}'..='\u{9FFF}' |  // CJK Unified Ideographs
        '\u{AC00}'..='\u{D7AF}' |  // Hangul Syllables
        '\u{FF00}'..='\u{FFEF}'    // Halfwidth and Fullwidth Forms
    )
}

/// Check if the position is preceded by "RT" or "RT:" (case insensitive)
/// where the RT itself is at start of input or preceded by whitespace.
/// This handles legacy retweet syntax like "RT@username" or "RT:@mention".
fn is_rt_prefix_before(full_input: &str, offset: usize) -> bool {
    // We need at least 2 chars before offset for "RT@" or 3 for "RT:@"
    if offset < 2 {
        return false;
    }

    let before = &full_input[..offset];

    // Check for "RT:" immediately before
    if before.ends_with(':') && before.len() >= 3 {
        let rt_check = &before[before.len() - 3..before.len() - 1];
        if rt_check.eq_ignore_ascii_case("RT") {
            // Check if RT is at start or preceded by space
            if before.len() == 3 {
                return true; // "RT:" at start
            }
            let before_rt = before[..before.len() - 3].chars().last();
            if before_rt.map(|c| c.is_whitespace()).unwrap_or(true) {
                return true;
            }
        }
    }

    // Check for "RT" immediately before (no colon)
    let rt_check = &before[before.len() - 2..];
    if rt_check.eq_ignore_ascii_case("RT") {
        // Check if RT is at start or preceded by space
        if before.len() == 2 {
            return true; // "RT" at start
        }
        let before_rt = before[..before.len() - 2].chars().last();
        if before_rt.map(|c| c.is_whitespace()).unwrap_or(true) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let entities = parse_tweet("");
        assert!(entities.is_empty());
    }

    #[test]
    fn test_parse_no_entities() {
        let entities = parse_tweet("Hello world");
        assert!(entities.is_empty());
    }

    #[test]
    fn test_invalid_char() {
        let entities = parse_tweet("Hello\u{fffe}world");
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, NomEntityType::InvalidChar);
    }

    #[test]
    fn test_url_with_trailing_period() {
        let entities = parse_tweet("text http://example.com.");
        eprintln!("Entities: {:?}", entities);
        for e in &entities {
            eprintln!("  Type: {:?}, Value: {:?}", e.entity_type, e.value);
        }
        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].entity_type, NomEntityType::Url);
        assert_eq!(entities[0].value, "http://example.com");
    }
}
