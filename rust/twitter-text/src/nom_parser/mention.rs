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

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::satisfy,
    combinator::recognize,
    sequence::tuple,
    IResult,
};

/// Match the @ prefix (regular or fullwidth).
fn at_prefix(input: &str) -> IResult<&str, &str> {
    alt((tag("@"), tag("\u{ff20}")))(input)
}

/// Check if a character is a valid username character (alphanumeric or underscore).
#[inline]
fn is_username_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Match the username text (1-20 valid characters).
fn username_text(input: &str) -> IResult<&str, &str> {
    take_while_m_n(1, 20, is_username_char)(input)
}

/// Parse a username/mention, returning the matched string slice.
/// Pattern: @username (1-20 chars)
pub fn parse_username(input: &str) -> IResult<&str, &str> {
    recognize(tuple((at_prefix, username_text)))(input)
}

/// Check if a character is a valid list slug character (alphanumeric, underscore, or hyphen).
#[inline]
fn is_list_slug_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || c == '-'
}

/// Match the list slug text (starts with letter, then 0-24 more chars, total 1-25).
fn list_slug_text(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        satisfy(|c| c.is_ascii_alphabetic()),
        take_while_m_n(0, 24, is_list_slug_char),
    )))(input)
}

/// Parse the list slug portion (including the /).
fn list_slug(input: &str) -> IResult<&str, &str> {
    recognize(tuple((tag("/"), list_slug_text)))(input)
}

/// Parse a list, returning the matched string slice and the position where the slug starts.
/// Pattern: @username/list-name
/// Returns (remaining, (full_match, slug_start_offset))
pub fn parse_list(input: &str) -> IResult<&str, (&str, usize)> {
    let (remaining, (prefix, username, slug)) =
        tuple((at_prefix, username_text, list_slug))(input)?;

    let full_len = prefix.len() + username.len() + slug.len();
    let slug_start = prefix.len() + username.len() + 1; // +1 for the /

    // Compute the full match slice
    let full_match = &input[..full_len];

    Ok((remaining, (full_match, slug_start)))
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

/// Check if a character is a valid federated username character (alphanumeric or underscore).
#[inline]
fn is_federated_username_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Check if a character is a federated separator (dot or hyphen).
#[inline]
fn is_federated_separator(c: char) -> bool {
    c == '.' || c == '-'
}

/// Match a federated username: alphanumeric/underscore chars with dots/hyphens between segments.
/// Pattern: segment (separator+ segment)*
fn federated_username(input: &str) -> IResult<&str, &str> {
    // Must start with at least one username char
    let (remaining, first_segment) = take_while1(is_federated_username_char)(input)?;

    // Continue consuming (separator+ segment)* without allocating
    let mut end_pos = first_segment.len();
    let mut rest = remaining;

    loop {
        // Try to match one or more separators
        match take_while1::<_, _, nom::error::Error<&str>>(is_federated_separator)(rest) {
            Ok((after_sep, sep)) => {
                // Must be followed by a segment
                match take_while1::<_, _, nom::error::Error<&str>>(is_federated_username_char)(
                    after_sep,
                ) {
                    Ok((after_segment, segment)) => {
                        end_pos += sep.len() + segment.len();
                        rest = after_segment;
                    }
                    Err(_) => break,
                }
            }
            Err(_) => break,
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Check if a character is a valid federated domain character (alphanumeric or underscore).
#[inline]
fn is_federated_domain_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Match a federated domain: alphanumeric/underscore chars with dots/hyphens between segments.
/// Pattern: segment (separator+ segment)*
fn federated_domain(input: &str) -> IResult<&str, &str> {
    // Must start with at least one domain char
    let (remaining, first_segment) = take_while1(is_federated_domain_char)(input)?;

    // Continue consuming (separator+ segment)* without allocating
    let mut end_pos = first_segment.len();
    let mut rest = remaining;

    loop {
        // Try to match one or more separators
        match take_while1::<_, _, nom::error::Error<&str>>(is_federated_separator)(rest) {
            Ok((after_sep, sep)) => {
                // Must be followed by a segment
                match take_while1::<_, _, nom::error::Error<&str>>(is_federated_domain_char)(
                    after_sep,
                ) {
                    Ok((after_segment, segment)) => {
                        end_pos += sep.len() + segment.len();
                        rest = after_segment;
                    }
                    Err(_) => break,
                }
            }
            Err(_) => break,
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Parse a federated mention (@username@domain).
/// Returns the full matched string including both @ symbols.
pub fn parse_federated_mention(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        at_prefix,
        federated_username,
        tag("@"),
        federated_domain,
    )))(input)
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
