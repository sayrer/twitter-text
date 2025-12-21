// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Permissive emoji matching for the nom parser.
//!
//! This module provides a permissive emoji matcher that identifies potential
//! emoji sequences. The actual validation is done externally via the `emojis` crate.

/// Check if a character can start an emoji sequence.
pub fn is_emoji_start(c: char) -> bool {
    matches!(c,
        // Copyright and Registered
        '\u{00a9}' | '\u{00ae}' |
        // Various symbols in the 2xxx-3xxx range
        '\u{203c}'..='\u{3299}' |
        // Main emoji blocks (includes regional indicators)
        '\u{1f000}'..='\u{1faff}' |
        // Keycap base characters
        '#' | '*' | '0'..='9'
    )
}

/// Check if a character is a non-ZWJ emoji modifier/continuation.
/// These can follow an emoji without ZWJ.
fn is_emoji_modifier(c: char) -> bool {
    matches!(c,
        // Skin tone modifiers
        '\u{1f3fb}'..='\u{1f3ff}' |
        // Variation selectors
        '\u{fe0e}' | '\u{fe0f}' |
        // Keycap combining mark
        '\u{20e3}' |
        // Regional indicators (for flags - second one combines with first)
        '\u{1f1e0}'..='\u{1f1ff}' |
        // Tag characters (for subdivision flags like 🏴󠁧󠁢󠁥󠁮󠁧󠁿)
        '\u{e0000}'..='\u{e007f}'
    )
}

/// Try to parse an emoji sequence starting at the current position.
/// Returns the matched emoji string and the number of bytes consumed,
/// or None if no emoji starts here.
pub fn try_parse_emoji(input: &str) -> Option<(&str, usize)> {
    let mut chars = input.chars();
    let first = chars.next()?;

    // Check if this could be the start of an emoji
    if !is_emoji_start(first) {
        return None;
    }

    let mut consumed = first.len_utf8();

    // For keycap sequences (#, *, 0-9), we need to check for FE0F and 20E3
    if matches!(first, '#' | '*' | '0'..='9') {
        // Must be followed by optional FE0F and then 20E3 to be an emoji
        let rest = &input[consumed..];
        let mut rest_chars = rest.chars();

        // Check for optional variation selector
        if let Some(c) = rest_chars.next() {
            if c == '\u{fe0f}' {
                consumed += c.len_utf8();
                // Now must have 20E3
                if let Some(c2) = rest_chars.next() {
                    if c2 == '\u{20e3}' {
                        consumed += c2.len_utf8();
                        return Some((&input[..consumed], consumed));
                    }
                }
                // Had FE0F but not 20E3, not an emoji keycap
                return None;
            } else if c == '\u{20e3}' {
                consumed += c.len_utf8();
                return Some((&input[..consumed], consumed));
            }
        }
        // No continuation, not an emoji
        return None;
    }

    // For other emoji, consume modifiers and ZWJ sequences
    loop {
        let remaining = &input[consumed..];
        let mut remaining_chars = remaining.chars();

        if let Some(c) = remaining_chars.next() {
            if is_emoji_modifier(c) {
                // Modifiers (skin tones, variation selectors, etc.) can follow directly
                consumed += c.len_utf8();
            } else if c == '\u{200d}' {
                // ZWJ must be followed by another emoji
                if let Some(next) = remaining_chars.next() {
                    if is_emoji_start(next) {
                        consumed += c.len_utf8(); // ZWJ
                        consumed += next.len_utf8(); // Next emoji
                    } else {
                        // ZWJ not followed by emoji - stop here
                        break;
                    }
                } else {
                    // ZWJ at end of input - stop
                    break;
                }
            } else {
                // Not a modifier or ZWJ - stop
                break;
            }
        } else {
            break;
        }
    }

    Some((&input[..consumed], consumed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_emoji() {
        let result = try_parse_emoji("😀");
        assert!(result.is_some());
        let (matched, _) = result.unwrap();
        assert_eq!(matched, "😀");
    }

    #[test]
    fn test_emoji_with_skin_tone() {
        let result = try_parse_emoji("👋🏽");
        assert!(result.is_some());
        let (matched, _) = result.unwrap();
        assert_eq!(matched, "👋🏽");
    }

    #[test]
    fn test_flag_emoji() {
        let result = try_parse_emoji("🇺🇸");
        assert!(result.is_some());
        let (matched, _) = result.unwrap();
        assert_eq!(matched, "🇺🇸");
    }

    #[test]
    fn test_keycap_emoji() {
        let result = try_parse_emoji("#\u{fe0f}\u{20e3}");
        assert!(result.is_some());
        let (matched, _) = result.unwrap();
        assert_eq!(matched, "#\u{fe0f}\u{20e3}");
    }

    #[test]
    fn test_not_emoji() {
        let result = try_parse_emoji("hello");
        assert!(result.is_none());
    }

    #[test]
    fn test_hash_not_emoji() {
        // Plain # is not an emoji
        let result = try_parse_emoji("#hashtag");
        assert!(result.is_none());
    }
}
