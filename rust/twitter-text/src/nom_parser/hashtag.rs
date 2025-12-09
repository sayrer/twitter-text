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

use nom::IResult;

/// Check if a character is a hashtag letter or mark.
/// Corresponds to hashtag_letter_or_mark in the Pest grammar.
#[inline]
fn is_hashtag_letter_or_mark(c: char) -> bool {
    use unicode_categories::UnicodeCategories;
    c.is_letter() || c.is_mark()
}

/// Check if a character is a hashtag special character.
/// These are allowed in hashtags but don't count as "letters".
#[inline]
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

/// Parse a hashtag, returning the matched string slice.
/// Returns the full hashtag including the # prefix.
pub fn parse_hashtag(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Check for hashtag prefix
    let prefix_len = if bytes.first() == Some(&b'#') {
        1
    } else if input.starts_with('\u{ff03}') {
        '\u{ff03}'.len_utf8()
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    let after_prefix = &input[prefix_len..];

    // Check we don't start with variation selector or enclosing keycap
    if after_prefix.starts_with('\u{fe0f}') || after_prefix.starts_with('\u{20e3}') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Not,
        )));
    }

    // Parse hashtag text - must contain at least one letter/mark
    let mut end_pos = prefix_len;
    let mut has_letter = false;

    for c in after_prefix.chars() {
        if is_hashtag_letter_or_mark(c) {
            has_letter = true;
            end_pos += c.len_utf8();
        } else if is_hashtag_special(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    // Must have at least one letter/mark
    if !has_letter {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    Ok((&input[end_pos..], &input[..end_pos]))
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
