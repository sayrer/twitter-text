// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Mention and list parsing for the nom parser.
//!
//! Usernames (mentions):
//! - @ or ＠ prefix
//! - 1-20 alphanumeric or underscore characters
//!
//! Lists:
//! - Username followed by /
//! - 1-25 characters (letter followed by alphanumeric, underscore, or hyphen)

use nom::IResult;

/// Check if a byte is a valid username character (alphanumeric or underscore).
#[inline(always)]
fn is_username_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Check if a byte is a valid list slug character (alphanumeric, underscore, or hyphen).
#[inline(always)]
fn is_list_slug_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

/// Parse a username/mention using direct byte scanning.
/// Pattern: @username (1-20 chars)
pub fn parse_username(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Check for @ prefix (regular or fullwidth)
    let prefix_len = if bytes.first() == Some(&b'@') {
        1
    } else if input.starts_with('\u{ff20}') {
        '\u{ff20}'.len_utf8()
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    // Scan username chars (1-20)
    let after_prefix = &bytes[prefix_len..];
    let mut username_len = 0;

    for &b in after_prefix.iter().take(20) {
        if is_username_byte(b) {
            username_len += 1;
        } else {
            break;
        }
    }

    if username_len == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    let total_len = prefix_len + username_len;
    Ok((&input[total_len..], &input[..total_len]))
}

/// Parse a list using direct byte scanning.
/// Pattern: @username/list-name
/// Returns (remaining, (full_match, slug_start_offset))
pub fn parse_list(input: &str) -> IResult<&str, (&str, usize)> {
    let bytes = input.as_bytes();

    // Check for @ prefix
    let prefix_len = if bytes.first() == Some(&b'@') {
        1
    } else if input.starts_with('\u{ff20}') {
        '\u{ff20}'.len_utf8()
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    // Scan username chars (1-20)
    let after_prefix = &bytes[prefix_len..];
    let mut username_len = 0;

    for &b in after_prefix.iter().take(20) {
        if is_username_byte(b) {
            username_len += 1;
        } else {
            break;
        }
    }

    if username_len == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Check for /
    let slash_pos = prefix_len + username_len;
    if bytes.get(slash_pos) != Some(&b'/') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    // Slug must start with a letter
    let slug_start_pos = slash_pos + 1;
    let slug_first = bytes.get(slug_start_pos);
    if !matches!(slug_first, Some(b) if b.is_ascii_alphabetic()) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Scan slug chars (first letter + 0-24 more, total 1-25)
    let after_slash = &bytes[slug_start_pos..];
    let mut slug_len = 1; // Already counted the first letter

    for &b in after_slash.iter().skip(1).take(24) {
        if is_list_slug_byte(b) {
            slug_len += 1;
        } else {
            break;
        }
    }

    let total_len = slug_start_pos + slug_len;
    let slug_start = prefix_len + username_len + 1; // Position after the /

    Ok((&input[total_len..], (&input[..total_len], slug_start)))
}

/// Try to parse either a list or a username.
/// Returns the matched string and optionally the list slug start position.
pub fn parse_mention_or_list(input: &str) -> IResult<&str, (&str, Option<usize>)> {
    // Try list first (longer match)
    if let Ok((remaining, (full_match, slug_start))) = parse_list(input) {
        return Ok((remaining, (full_match, Some(slug_start))));
    }

    // Fall back to username
    let (remaining, matched) = parse_username(input)?;
    Ok((remaining, (matched, None)))
}

/// Check if the character before a potential mention is valid.
/// Mentions should not be preceded by alphanumeric or underscore.
pub fn valid_mention_predecessor(c: char) -> bool {
    !c.is_ascii_alphanumeric() && c != '_'
}

// Federated mention parsing (Mastodon-style @user@domain.tld)

/// Check if a byte is a valid federated username/domain character (alphanumeric or underscore).
#[inline(always)]
fn is_federated_char_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Check if a byte is a federated separator (dot or hyphen).
#[inline(always)]
fn is_federated_separator_byte(b: u8) -> bool {
    b == b'.' || b == b'-'
}

/// Scan a federated name segment (username or domain): alphanumeric/underscore chars with dots/hyphens between segments.
/// Returns the number of bytes consumed.
#[inline]
fn scan_federated_segment(bytes: &[u8]) -> usize {
    if bytes.is_empty() || !is_federated_char_byte(bytes[0]) {
        return 0;
    }

    let mut end_pos = 1;
    let mut i = 1;

    while i < bytes.len() {
        let b = bytes[i];

        if is_federated_char_byte(b) {
            end_pos = i + 1;
            i += 1;
        } else if is_federated_separator_byte(b) {
            // Separator must be followed by at least one char
            let mut sep_end = i + 1;
            // Consume consecutive separators
            while sep_end < bytes.len() && is_federated_separator_byte(bytes[sep_end]) {
                sep_end += 1;
            }
            // Must be followed by a segment char
            if sep_end < bytes.len() && is_federated_char_byte(bytes[sep_end]) {
                end_pos = sep_end + 1;
                i = sep_end + 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    end_pos
}

/// Parse a federated mention (@username@domain) using direct byte scanning.
/// Returns the full matched string including both @ symbols.
pub fn parse_federated_mention(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Check for @ prefix
    let prefix_len = if bytes.first() == Some(&b'@') {
        1
    } else if input.starts_with('\u{ff20}') {
        '\u{ff20}'.len_utf8()
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    // Scan username
    let username_len = scan_federated_segment(&bytes[prefix_len..]);
    if username_len == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Check for second @
    let second_at_pos = prefix_len + username_len;
    if bytes.get(second_at_pos) != Some(&b'@') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    // Scan domain
    let domain_start = second_at_pos + 1;
    let domain_len = scan_federated_segment(&bytes[domain_start..]);
    if domain_len == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    let total_len = domain_start + domain_len;
    Ok((&input[total_len..], &input[..total_len]))
}

/// Try to parse a federated mention, username, or list.
/// Tries federated mention first (longer match with @domain), then list, then username.
/// Returns the matched string, entity type indicator, and optionally the list slug start position.
pub fn parse_any_mention(input: &str) -> IResult<&str, (&str, MentionType)> {
    // Try federated mention first (has @domain suffix)
    if let Ok((remaining, matched)) = parse_federated_mention(input) {
        return Ok((remaining, (matched, MentionType::Federated)));
    }

    // Try list (has /slug suffix)
    if let Ok((remaining, (matched, slug_start))) = parse_list(input) {
        return Ok((remaining, (matched, MentionType::List(slug_start))));
    }

    // Fall back to username
    let (remaining, matched) = parse_username(input)?;
    Ok((remaining, (matched, MentionType::Username)))
}

/// The type of mention parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MentionType {
    Username,
    List(usize), // Contains slug_start offset
    Federated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_username() {
        let result = parse_username("@user");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_username_with_numbers() {
        let result = parse_username("@user123");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user123");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_username_with_underscore() {
        let result = parse_username("@user_name");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user_name");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_fullwidth_at() {
        let result = parse_username("\u{ff20}user");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "\u{ff20}user");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_username_max_length() {
        let result = parse_username("@12345678901234567890"); // 20 chars
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@12345678901234567890");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_username_over_max_length() {
        let result = parse_username("@123456789012345678901"); // 21 chars
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@12345678901234567890"); // Only 20 chars
        assert_eq!(remaining, "1");
    }

    #[test]
    fn test_simple_list() {
        let result = parse_list("@user/mylist");
        assert!(result.is_ok());
        let (remaining, (matched, slug_start)) = result.unwrap();
        assert_eq!(matched, "@user/mylist");
        assert_eq!(slug_start, 6); // After "@user/"
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_list_with_hyphen() {
        let result = parse_list("@user/my-list");
        assert!(result.is_ok());
        let (remaining, (matched, _)) = result.unwrap();
        assert_eq!(matched, "@user/my-list");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_mention_or_list_with_list() {
        let result = parse_mention_or_list("@user/mylist");
        assert!(result.is_ok());
        let (remaining, (matched, slug_start)) = result.unwrap();
        assert_eq!(matched, "@user/mylist");
        assert!(slug_start.is_some());
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_mention_or_list_with_mention() {
        let result = parse_mention_or_list("@user");
        assert!(result.is_ok());
        let (remaining, (matched, slug_start)) = result.unwrap();
        assert_eq!(matched, "@user");
        assert!(slug_start.is_none());
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_username_stops_at_space() {
        let result = parse_username("@user hello");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user");
        assert_eq!(remaining, " hello");
    }

    // Federated mention tests

    #[test]
    fn test_federated_mention_simple() {
        let result = parse_federated_mention("@user@domain.com");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user@domain.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_federated_mention_complex_domain() {
        let result = parse_federated_mention("@user@sub.domain.org");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user@sub.domain.org");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_federated_mention_with_underscores() {
        let result = parse_federated_mention("@user_name@my_domain.com");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user_name@my_domain.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_federated_mention_with_dots_in_username() {
        let result = parse_federated_mention("@user.name@domain.com");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user.name@domain.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_federated_mention_with_hyphens() {
        let result = parse_federated_mention("@user-name@my-domain.com");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user-name@my-domain.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_federated_mention_fullwidth_at() {
        let result = parse_federated_mention("\u{ff20}user@domain.com");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "\u{ff20}user@domain.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_federated_mention_stops_at_space() {
        let result = parse_federated_mention("@user@domain.com hello");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "@user@domain.com");
        assert_eq!(remaining, " hello");
    }

    #[test]
    fn test_parse_any_mention_federated() {
        let result = parse_any_mention("@user@domain.com");
        assert!(result.is_ok());
        let (remaining, (matched, mention_type)) = result.unwrap();
        assert_eq!(matched, "@user@domain.com");
        assert_eq!(mention_type, MentionType::Federated);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_any_mention_username() {
        let result = parse_any_mention("@user");
        assert!(result.is_ok());
        let (remaining, (matched, mention_type)) = result.unwrap();
        assert_eq!(matched, "@user");
        assert_eq!(mention_type, MentionType::Username);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_any_mention_list() {
        let result = parse_any_mention("@user/mylist");
        assert!(result.is_ok());
        let (remaining, (matched, mention_type)) = result.unwrap();
        assert_eq!(matched, "@user/mylist");
        assert!(matches!(mention_type, MentionType::List(_)));
        assert_eq!(remaining, "");
    }
}
