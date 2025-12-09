// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Hashtag parsing for the nom parser.
//!
//! Hashtags consist of:
//! - A prefix (# or ＃)
//! - One or more hashtag_text characters (letters, marks, digits, underscore, special chars)
//!
//! Hashtags must contain at least one letter/mark character.

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::{not, peek, recognize},
    multi::many1,
    sequence::tuple,
    IResult,
};

/// Check if a character is a hashtag letter or mark.
/// Corresponds to hashtag_letter_or_mark in the Pest grammar.
fn is_hashtag_letter_or_mark(c: char) -> bool {
    use unicode_categories::UnicodeCategories;
    c.is_letter() || c.is_mark()
}

/// Check if a character is a hashtag special character.
/// These are allowed in hashtags but don't count as "letters".
fn is_hashtag_special(c: char) -> bool {
    use unicode_categories::UnicodeCategories;
    c.is_number_decimal_digit()
        || c == '_'
        || matches!(
            c,
            '\u{200c}' // ZWNJ
            | '\u{200d}' // ZWJ
            | '\u{a67e}' // Cyrillic Kavyka
            | '\u{05be}' // Hebrew Maqaf
            | '\u{05f3}' // Hebrew Geresh
            | '\u{05f4}' // Hebrew Gershayim
            | '\u{ff5e}' // Fullwidth Tilde
            | '\u{301c}' // Wave Dash
            | '\u{309b}' // Katakana-Hiragana Voiced Sound Mark
            | '\u{309c}' // Katakana-Hiragana Semi-Voiced Sound Mark
            | '\u{30a0}' // Katakana-Hiragana Double Hyphen
            | '\u{30fb}' // Katakana Middle Dot
            | '\u{3003}' // Ditto Mark
            | '\u{0f0b}' // Tibetan Mark Intersyllabic Tsheg
            | '\u{0f0c}' // Tibetan Mark Delimiter Tsheg Bstar
            | '\u{00b7}' // Middle Dot
        )
}

/// Match the hashtag prefix (# or ＃).
fn hashtag_prefix(input: &str) -> IResult<&str, &str> {
    alt((tag("#"), tag("\u{ff03}")))(input)
}

/// Match a hashtag letter or mark character.
fn hashtag_letter_or_mark(input: &str) -> IResult<&str, char> {
    satisfy(is_hashtag_letter_or_mark)(input)
}

/// Match a hashtag special character.
fn hashtag_special(input: &str) -> IResult<&str, char> {
    satisfy(is_hashtag_special)(input)
}

/// Match hashtag text: must contain at least one letter/mark.
/// Pattern: (special* letter_or_mark+ special*)+
fn hashtag_text(input: &str) -> IResult<&str, &str> {
    // First, ensure we don't start with variation selector or enclosing keycap
    let (input, _) = not(peek(alt((tag("\u{fe0f}"), tag("\u{20e3}")))))(input)?;

    // Match: (special* letter_or_mark+ special*)+
    recognize(many1(tuple((
        recognize(nom::multi::many0(hashtag_special)),
        recognize(many1(hashtag_letter_or_mark)),
        recognize(nom::multi::many0(hashtag_special)),
    ))))(input)
}

/// Parse a hashtag, returning the matched string slice.
/// Returns the full hashtag including the # prefix.
pub fn parse_hashtag(input: &str) -> IResult<&str, &str> {
    recognize(tuple((hashtag_prefix, hashtag_text)))(input)
}

/// Check if the character before a potential hashtag is valid.
/// Hashtags should not be preceded by &, or a letter/mark.
/// Exception: variation selectors (FE0E, FE0F) are allowed.
pub fn valid_hashtag_predecessor(c: char) -> bool {
    // Variation selectors are allowed before hashtags
    if c == '\u{fe0e}' || c == '\u{fe0f}' {
        return true;
    }
    c != '&' && !is_hashtag_letter_or_mark(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hashtag() {
        let result = parse_hashtag("#hello");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "#hello");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_hashtag_with_numbers() {
        let result = parse_hashtag("#hello123");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "#hello123");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_hashtag_with_underscore() {
        let result = parse_hashtag("#hello_world");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "#hello_world");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_fullwidth_hashtag() {
        let result = parse_hashtag("\u{ff03}hello");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "\u{ff03}hello");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_hashtag_unicode() {
        let result = parse_hashtag("#日本語");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "#日本語");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_hashtag_only_numbers_fails() {
        // Hashtags must contain at least one letter
        let result = parse_hashtag("#123");
        assert!(result.is_err());
    }

    #[test]
    fn test_hashtag_stops_at_space() {
        let result = parse_hashtag("#hello world");
        assert!(result.is_ok());
        let (remaining, matched) = result.unwrap();
        assert_eq!(matched, "#hello");
        assert_eq!(remaining, " world");
    }
}
