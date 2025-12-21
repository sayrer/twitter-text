// Copyright 2025 Robert Sayre
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

/// Get the last character of a string slice efficiently.
/// For ASCII-only strings this is O(1), otherwise O(n) in the worst case.
#[inline]
fn last_char(s: &str) -> Option<char> {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return None;
    }
    let last_byte = bytes[bytes.len() - 1];
    if last_byte < 128 {
        // ASCII - O(1)
        Some(last_byte as char)
    } else {
        // Non-ASCII - need to find char boundary
        s.chars().last()
    }
}

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
    let mut prev_char: Option<char> = None;

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
                prev_char = Some(b as char);
                pos += 1;
                continue;
            }

            let remaining = &input[pos..];
            let current_char = b as char;

            // Try to match an entity at this position
            if let Some((entity, consumed)) = try_parse_entity(input, remaining, pos, prev_char) {
                entities.push(entity);
                // Update prev_char to last char of consumed entity
                prev_char = last_char(&input[pos..pos + consumed]);
                pos += consumed;
            } else {
                // Handle special ASCII cases that failed entity parsing
                if b == b'$' {
                    // $ followed by URL-like content should be skipped
                    let after_dollar = &remaining[1..];
                    let skip = skip_invalid_cashtag_url(after_dollar);
                    prev_char = last_char(&input[pos..pos + 1 + skip]);
                    pos += 1 + skip;
                } else if b == b'@' {
                    // @ followed by URL-like content (email) should skip
                    let after_at = &remaining[1..];
                    let skip = skip_email_domain(after_at);
                    prev_char = last_char(&input[pos..pos + 1 + skip]);
                    pos += 1 + skip;
                } else {
                    // Regular ASCII char, just skip
                    prev_char = Some(current_char);
                    pos += 1;
                }
            }
        } else {
            // Non-ASCII: need to decode UTF-8
            let remaining = &input[pos..];
            let c = remaining.chars().next().unwrap();
            let char_len = c.len_utf8();

            // Try to match an entity at this position
            if let Some((entity, consumed)) = try_parse_entity(input, remaining, pos, prev_char) {
                entities.push(entity);
                prev_char = last_char(&input[pos..pos + consumed]);
                pos += consumed;
            } else if common::is_invalid_char(c) {
                entities.push(NomEntity::new(
                    NomEntityType::InvalidChar,
                    &remaining[..char_len],
                    pos,
                    pos + char_len,
                ));
                prev_char = Some(c);
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
                    prev_char = last_char(matched);
                    pos += consumed;
                } else {
                    prev_char = Some(c);
                    pos += char_len;
                }
            } else if c == '\u{ff20}' {
                // Fullwidth @ - check for email domain
                let after_at = &remaining[char_len..];
                let skip = skip_email_domain(after_at);
                prev_char = last_char(&input[pos..pos + char_len + skip]);
                pos += char_len + skip;
            } else {
                prev_char = Some(c);
                pos += char_len;
            }
        }
    }

    entities
}

/// Parse a tweet and return only mention entities (usernames and lists).
/// This is optimized for the case where only mentions are needed.
/// Uses memchr to skip directly to @ characters, avoiding all URL/hashtag parsing.
pub fn parse_mentions_only(input: &str) -> Vec<NomEntity<'_>> {
    let mut entities = Vec::with_capacity(8);
    let bytes = input.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        // Use memchr to find the next @ character (SIMD-accelerated)
        let remaining_bytes = &bytes[pos..];

        // Find ASCII @ first
        let at_pos = memchr::memchr(b'@', remaining_bytes);

        // Also need to find fullwidth @ (U+FF20 = 0xEF 0xBC 0xA0 in UTF-8)
        // We look for 0xEF which starts the sequence
        let fullwidth_pos = find_fullwidth_at(remaining_bytes);

        // Take the earlier of the two
        let next_at = match (at_pos, fullwidth_pos) {
            (Some(a), Some(f)) => Some(a.min(f)),
            (Some(a), None) => Some(a),
            (None, Some(f)) => Some(f),
            (None, None) => None,
        };

        let at_offset = match next_at {
            Some(offset) => offset,
            None => break, // No more @ in the text
        };

        pos += at_offset;

        // Check predecessor - mentions shouldn't be preceded by certain characters
        // Exception: RT@ pattern
        if pos > 0 {
            let prev_byte = bytes[pos - 1];
            // Check for invalid ASCII prefix characters
            let is_invalid_prefix = prev_byte.is_ascii_alphanumeric()
                || prev_byte == b'_'
                || prev_byte == b'@'
                || prev_byte == b'!'
                || prev_byte == b'#'
                || prev_byte == b'$'
                || prev_byte == b'%'
                || prev_byte == b'&'
                || prev_byte == b'*'
                || prev_byte == b'='
                || prev_byte == b'/';

            if is_invalid_prefix {
                // Check for RT exception
                if !is_rt_prefix_before(input, pos) {
                    pos += 1;
                    continue;
                }
            }

            // Also check for fullwidth @ (U+FF20) as predecessor
            // UTF-8: 0xEF 0xBC 0xA0
            if pos >= 3
                && bytes[pos - 3] == 0xEF
                && bytes[pos - 2] == 0xBC
                && bytes[pos - 1] == 0xA0
            {
                pos += 1;
                continue;
            }
        }

        let remaining = &input[pos..];

        // Try to parse a mention (username or list)
        if let Ok((after, (matched, mention_type))) = mention::parse_any_mention(remaining) {
            // Check for invalid suffix (@ or latin_accent or :// or -)
            let has_invalid_suffix = if let Some(next_char) = after.chars().next() {
                next_char == '@'
                    || next_char == '\u{ff20}'
                    || next_char == '-'
                    || url::is_latin_accent(next_char)
                    || after.starts_with("://")
            } else {
                false
            };

            if !has_invalid_suffix {
                let consumed = matched.len();
                let entity_type = match mention_type {
                    mention::MentionType::Federated => NomEntityType::FederatedMention,
                    mention::MentionType::List(slug_start) => {
                        entities.push(NomEntity::new_list(
                            matched,
                            pos,
                            pos + consumed,
                            pos + slug_start,
                        ));
                        pos += consumed;
                        continue;
                    }
                    mention::MentionType::Username => NomEntityType::Username,
                };
                entities.push(NomEntity::new(entity_type, matched, pos, pos + consumed));
                pos += consumed;
                continue;
            }
        }

        // Failed to parse - skip this @ and continue
        pos += 1;
    }

    entities
}

/// Parse a tweet and return only cashtag entities.
/// This is optimized for the case where only cashtags are needed.
/// Uses memchr to skip directly to $ characters, avoiding all URL/mention/hashtag parsing.
pub fn parse_cashtags_only(input: &str) -> Vec<NomEntity<'_>> {
    let mut entities = Vec::with_capacity(4);
    let bytes = input.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        // Use memchr to find the next $ character (SIMD-accelerated)
        let remaining_bytes = &bytes[pos..];

        let dollar_offset = match memchr::memchr(b'$', remaining_bytes) {
            Some(offset) => offset,
            None => break, // No more $ in the text
        };

        pos += dollar_offset;

        // Check predecessor - cashtags must be preceded by whitespace or start of input
        if pos > 0 {
            let prev_byte = bytes[pos - 1];
            // Fast check for common ASCII whitespace
            let is_space =
                prev_byte == b' ' || prev_byte == b'\t' || prev_byte == b'\n' || prev_byte == b'\r';
            if !is_space {
                // For non-ASCII, need to check if it's Unicode whitespace
                // But cashtags are very restrictive - only ASCII whitespace really matters
                pos += 1;
                continue;
            }
        }

        let remaining = &input[pos..];

        // Try to parse a cashtag
        if let Ok((_, matched)) = cashtag::parse_cashtag(remaining) {
            let consumed = matched.len();
            entities.push(NomEntity::new(
                NomEntityType::Cashtag,
                matched,
                pos,
                pos + consumed,
            ));
            pos += consumed;
            continue;
        }

        // Failed to parse - skip this $ and continue
        pos += 1;
    }

    entities
}

/// Find fullwidth @ (U+FF20) in a byte slice.
/// U+FF20 is encoded as 0xEF 0xBC 0xA0 in UTF-8.
#[inline]
fn find_fullwidth_at(bytes: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i + 2 < bytes.len() {
        if let Some(offset) = memchr::memchr(0xEF, &bytes[i..]) {
            let pos = i + offset;
            if pos + 2 < bytes.len() && bytes[pos + 1] == 0xBC && bytes[pos + 2] == 0xA0 {
                return Some(pos);
            }
            i = pos + 1;
        } else {
            break;
        }
    }
    None
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
/// * `prev_char` - The character immediately before this position (if any)
fn try_parse_entity<'a>(
    full_input: &'a str,
    input: &'a str,
    offset: usize,
    prev_char: Option<char>,
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

    // Check based on first character to avoid unnecessary parsing attempts
    match first_char {
        // @ or fullwidth @
        '@' | '\u{ff20}' => {
            // Check for invalid prefix - mentions shouldn't be preceded by alphanumeric, @, or certain punctuation
            // EXCEPT for the special "RT" prefix case (legacy retweet syntax)
            // Also check for = and / which are invalid for federated mentions
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
                    || pc == '*'
                    || pc == '='
                    || pc == '/';

                if is_invalid_prefix {
                    // Check for RT prefix exception: "RT@" or "RT:@" preceded by space/SOI
                    let is_rt_prefix = is_rt_prefix_before(full_input, offset);
                    if !is_rt_prefix {
                        return None;
                    }
                }
            }

            // Try federated mention, list, or username (in that order for longest match)
            if let Ok((remaining, (matched, mention_type))) = mention::parse_any_mention(input) {
                // For federated mentions, we don't check invalid suffix (it's already parsed correctly)
                // For username/list, check for invalid suffix
                match mention_type {
                    mention::MentionType::Federated => {
                        let consumed = matched.len();
                        let entity = NomEntity::new(
                            NomEntityType::FederatedMention,
                            matched,
                            offset,
                            offset + consumed,
                        );
                        return Some((entity, consumed));
                    }
                    mention::MentionType::List(slug_start) => {
                        // Check for invalid suffix (@ or latin_accent or :// or -)
                        if let Some(next_char) = remaining.chars().next() {
                            if next_char == '@'
                                || next_char == '\u{ff20}'
                                || next_char == '-'
                                || url::is_latin_accent(next_char)
                                || remaining.starts_with("://")
                            {
                                return None;
                            }
                        }
                        let consumed = matched.len();
                        let entity = NomEntity::new_list(
                            matched,
                            offset,
                            offset + consumed,
                            offset + slug_start,
                        );
                        return Some((entity, consumed));
                    }
                    mention::MentionType::Username => {
                        // Check for invalid suffix (@ or latin_accent or :// or -)
                        if let Some(next_char) = remaining.chars().next() {
                            if next_char == '@'
                                || next_char == '\u{ff20}'
                                || next_char == '-'
                                || url::is_latin_accent(next_char)
                                || remaining.starts_with("://")
                            {
                                return None;
                            }
                        }
                        let consumed = matched.len();
                        let entity = NomEntity::new(
                            NomEntityType::Username,
                            matched,
                            offset,
                            offset + consumed,
                        );
                        return Some((entity, consumed));
                    }
                }
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
                let before_bytes = before.as_bytes();

                // Fast path: scan backwards for ASCII whitespace or non-ASCII
                // Most tweets are ASCII, so this is usually very fast
                let mut last_delim = 0;
                for (i, &b) in before_bytes.iter().enumerate().rev() {
                    if b == b' ' || b == b'\n' || b == b'\r' || b == b'\t' {
                        last_delim = i + 1;
                        break;
                    }
                    if b >= 128 {
                        // Found non-ASCII - need to check for Unicode delimiters
                        // Find the start of this UTF-8 char and scan from there
                        let mut start = i;
                        while start > 0 && (before_bytes[start] & 0xC0) == 0x80 {
                            start -= 1;
                        }
                        let suffix = &before[start..];
                        if let Some((j, c)) = suffix
                            .char_indices()
                            .rev()
                            .find(|(_, c)| common::is_space(*c) || is_url_delimiter(*c))
                        {
                            last_delim = start + j + c.len_utf8();
                        }
                        break;
                    }
                }

                let since_delim = &before[last_delim..];
                // If there's "://" followed by anything (like "http://-foo"), skip this
                // Use memchr to find ':' then check for "//" - faster than contains()
                let since_bytes = since_delim.as_bytes();
                if let Some(colon_pos) = memchr::memchr(b':', since_bytes) {
                    if since_bytes.get(colon_pos + 1..colon_pos + 3) == Some(b"//") {
                        return None;
                    }
                }
            }

            // Check if there's a dot before the next space (early rejection)
            let has_dot = {
                let bytes = input.as_bytes();
                let mut found = false;
                for &b in bytes {
                    if b == b'.' {
                        found = true;
                        break;
                    }
                    // Check for space (common ASCII spaces)
                    if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
                        break;
                    }
                    // Non-ASCII might be Unicode space, check char
                    if b >= 128 {
                        // Fall back to char check for non-ASCII
                        for c in input.chars() {
                            if c == '.' {
                                found = true;
                                break;
                            }
                            if common::is_space(c) {
                                break;
                            }
                        }
                        break;
                    }
                }
                found
            };
            if has_dot {
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
