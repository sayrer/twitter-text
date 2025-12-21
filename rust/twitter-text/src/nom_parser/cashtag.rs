// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Cashtag parsing for the nom parser.
//!
//! Cashtags are stock symbols:
//! - A prefix ($)
//! - 1-6 ASCII letters
//! - Optionally: a dot or underscore followed by 1-2 more ASCII letters
//! - Must not be followed by a digit or letter

use nom::IResult;

/// Parse a cashtag using direct byte scanning.
/// Pattern: $SYMBOL or $SYMBOL.XX or $SYMBOL_XX
pub fn parse_cashtag(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Check for $ prefix
    if bytes.first() != Some(&b'$') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    // Scan 1-6 ASCII letters
    let mut pos = 1;
    let mut symbol_len = 0;

    while pos < bytes.len() && symbol_len < 6 {
        let b = bytes[pos];
        if b.is_ascii_alphabetic() {
            symbol_len += 1;
            pos += 1;
        } else {
            break;
        }
    }

    if symbol_len == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Optional: dot or underscore followed by 1-2 letters
    if pos < bytes.len() && (bytes[pos] == b'.' || bytes[pos] == b'_') {
        let sep_pos = pos;
        pos += 1;

        let mut suffix_len = 0;
        while pos < bytes.len() && suffix_len < 2 {
            let b = bytes[pos];
            if b.is_ascii_alphabetic() {
                suffix_len += 1;
                pos += 1;
            } else {
                break;
            }
        }

        // If no suffix letters, revert to before separator
        if suffix_len == 0 {
            pos = sep_pos;
        }
    }

    // Verify not followed by digit or letter
    if let Some(&next_byte) = bytes.get(pos) {
        if next_byte.is_ascii_alphanumeric() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
    }

    Ok((&input[pos..], &input[..pos]))
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
