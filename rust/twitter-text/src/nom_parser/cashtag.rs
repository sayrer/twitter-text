// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Cashtag parsing for the nom parser.
//!
//! Cashtags are stock symbols:
//! - A prefix ($)
//! - 1-6 ASCII letters
//! - Optionally: a dot or underscore followed by 1-2 more ASCII letters
//! - Must not be followed by a digit or letter

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{opt, recognize},
    multi::many_m_n,
    sequence::tuple,
    IResult,
};

/// Match the $ prefix.
fn cashtag_prefix(input: &str) -> IResult<&str, &str> {
    tag("$")(input)
}

/// Match a single ASCII alphabetic character.
fn ascii_alpha(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_alphabetic())(input)
}

/// Parse a cashtag, returning the matched string slice.
/// Pattern: $SYMBOL or $SYMBOL.XX or $SYMBOL_XX
pub fn parse_cashtag(input: &str) -> IResult<&str, &str> {
    // Use recognize to get the full matched string
    let (remaining, matched) = recognize(tuple((
        cashtag_prefix,
        many_m_n(1, 6, ascii_alpha),
        opt(tuple((
            alt((tag("."), tag("_"))),
            many_m_n(1, 2, ascii_alpha),
        ))),
    )))(input)?;

    // Verify not followed by digit or letter
    if let Some(next_char) = remaining.chars().next() {
        if next_char.is_ascii_alphanumeric() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
    }

    Ok((remaining, matched))
}

/// Check if the character before a potential cashtag is valid.
/// Cashtags should be preceded by whitespace or start of string.
pub fn valid_cashtag_predecessor(c: char) -> bool {
    c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cashtag() {
        let result = parse_cashtag("$AAPL");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "$AAPL");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_cashtag_with_dot() {
        let result = parse_cashtag("$BRK.A");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "$BRK.A");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_cashtag_with_underscore() {
        let result = parse_cashtag("$TEST_A");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "$TEST_A");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_cashtag_lowercase() {
        // Cashtags are case-insensitive at parse time
        let result = parse_cashtag("$aapl");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "$aapl");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_cashtag_too_long() {
        // More than 6 chars should not all be consumed
        let result = parse_cashtag("$ABCDEFG");
        assert!(result.is_err()); // Followed by G
    }

    #[test]
    fn test_cashtag_followed_by_number() {
        // Cashtags can't be followed by digits
        let result = parse_cashtag("$AAPL1");
        assert!(result.is_err());
    }

    #[test]
    fn test_cashtag_with_space() {
        let result = parse_cashtag("$AAPL is great");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "$AAPL");
        assert_eq!(remaining, " is great");
    }
}
