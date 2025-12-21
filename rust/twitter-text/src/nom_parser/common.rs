// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Common character classes and utilities for the nom parser.
//!
//! These correspond to the character class rules in twitter_text.pest.

use nom::{branch::alt, bytes::complete::tag, character::complete::satisfy, IResult};

/// Match a single ASCII alphanumeric character.
pub fn alphanumeric(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_alphanumeric())(input)
}

/// Match a single ASCII alphabetic character.
pub fn ascii_alpha(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_alphabetic())(input)
}

/// Match a single ASCII digit.
pub fn ascii_digit(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_digit())(input)
}

/// Match a single hex digit.
pub fn hex_digit(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_hexdigit())(input)
}

/// Match a Latin accent character.
/// Corresponds to latin_accent in the Pest grammar.
pub fn latin_accent(input: &str) -> IResult<&str, char> {
    satisfy(|c| {
        matches!(c,
            '\u{00c0}'..='\u{00d6}' |
            '\u{00d8}'..='\u{00f6}' |
            '\u{00f8}'..='\u{00ff}' |
            '\u{0100}'..='\u{024f}' |
            '\u{0253}'..='\u{0254}' |
            '\u{0256}'..='\u{0257}' |
            '\u{0259}' |
            '\u{025b}' |
            '\u{0263}' |
            '\u{0268}' |
            '\u{026f}' |
            '\u{0272}' |
            '\u{0289}' |
            '\u{028b}' |
            '\u{02bb}' |
            '\u{0300}'..='\u{036f}' |
            '\u{1e00}'..='\u{1eff}'
        )
    })(input)
}

/// Match a Cyrillic character.
pub fn cyrillic_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| matches!(c, '\u{0400}'..='\u{04ff}'))(input)
}

/// Match a space character (Unicode spaces).
pub fn space_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| {
        matches!(c,
            '\u{0020}' |  // Space
            '\u{0085}' |  // Next Line
            '\u{00A0}' |  // No-Break Space
            '\u{1680}' |  // Ogham Space Mark
            '\u{180E}' |  // Mongolian Vowel Separator
            '\u{2028}' |  // Line Separator
            '\u{2029}' |  // Paragraph Separator
            '\u{202F}' |  // Narrow No-Break Space
            '\u{205F}' |  // Medium Mathematical Space
            '\u{3000}' |  // Ideographic Space
            '\u{0009}'..='\u{000D}' |  // Tab, LF, VT, FF, CR
            '\u{2000}'..='\u{200A}'    // Various spaces
        )
    })(input)
}

/// Match a punctuation character.
pub fn punctuation_char(input: &str) -> IResult<&str, char> {
    satisfy(|c| {
        matches!(
            c,
            '-' | '_'
                | '!'
                | '"'
                | '#'
                | '$'
                | '%'
                | '&'
                | '\''
                | '('
                | ')'
                | '*'
                | '+'
                | ','
                | '.'
                | '/'
                | '\\'
                | ':'
                | ';'
                | '<'
                | '='
                | '>'
                | '?'
                | '@'
                | '['
                | ']'
                | '^'
                | '`'
                | '{'
                | '|'
                | '}'
                | '~'
        )
    })(input)
}

/// Check if a character is a space.
pub fn is_space(c: char) -> bool {
    matches!(c,
        '\u{0020}' | '\u{0085}' | '\u{00A0}' | '\u{1680}' | '\u{180E}' |
        '\u{2028}' | '\u{2029}' | '\u{202F}' | '\u{205F}' | '\u{3000}' |
        '\u{0009}'..='\u{000D}' | '\u{2000}'..='\u{200A}'
    )
}

/// Check if a character is punctuation.
pub fn is_punctuation(c: char) -> bool {
    matches!(
        c,
        '-' | '_'
            | '!'
            | '"'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '+'
            | ','
            | '.'
            | '/'
            | '\\'
            | ':'
            | ';'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | ']'
            | '^'
            | '`'
            | '{'
            | '|'
            | '}'
            | '~'
    ) || c.is_ascii_punctuation()
}

/// Check if a character is an invalid tweet character.
pub fn is_invalid_char(c: char) -> bool {
    matches!(c, '\u{fffe}' | '\u{feff}' | '\u{ffff}')
}

/// Match an invalid character.
pub fn invalid_char(input: &str) -> IResult<&str, char> {
    satisfy(is_invalid_char)(input)
}

/// Match the @ prefix (regular or fullwidth).
pub fn at_prefix(input: &str) -> IResult<&str, &str> {
    alt((tag("@"), tag("\u{ff20}")))(input) // @ or ＠
}

/// Match the # prefix (regular or fullwidth).
pub fn hash_prefix(input: &str) -> IResult<&str, &str> {
    alt((tag("#"), tag("\u{ff03}")))(input) // # or ＃
}

/// Match the $ prefix.
pub fn dollar_prefix(input: &str) -> IResult<&str, &str> {
    tag("$")(input)
}
