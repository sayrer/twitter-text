// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! URL parsing for the nom parser.
//!
//! URLs can have two forms:
//! 1. With protocol: http:// or https:// followed by host, path, query, fragment
//! 2. Without protocol: domain.tld followed by optional path, query, fragment
//!
//! TLD validation is done externally via phf lookup.

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while, take_while1},
    character::complete::{char, satisfy},
    combinator::{opt, recognize},
    sequence::{delimited, preceded, tuple},
    IResult,
};

use super::common;

/// Check if a character is an invalid TLD suffix.
/// TLDs cannot be immediately followed by alphanumeric or @.
/// From grammar: invalid_tld_suffix = _{ 'a'..'z' | '0'..'9' | "@" }
fn is_invalid_tld_suffix(c: char) -> bool {
    c.is_ascii_lowercase()
        || c.is_ascii_uppercase()
        || c.is_ascii_digit()
        || c == '@'
        || c == '\u{ff20}'
}

/// Parse the http:// or https:// protocol prefix.
fn protocol(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag_no_case("http"),
        opt(tag_no_case("s")),
        tag("://"),
    )))(input)
}

/// Check if a character is a Latin accent character.
pub fn is_latin_accent(c: char) -> bool {
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
}

/// Check if a character is a Cyrillic character.
fn is_cyrillic(c: char) -> bool {
    matches!(c, '\u{0400}'..='\u{04ff}')
}

/// Check if a character is a "non-punctuation" character.
/// This corresponds to `non_punctuation` in the Pest grammar:
/// any character that is not punctuation, space, or an invalid char.
/// This allows CJK and other Unicode characters in domains.
fn is_non_punctuation(c: char) -> bool {
    !common::is_space(c) && !common::is_punctuation(c) && !common::is_invalid_char(c)
}

/// Check if a character is a domain character.
/// This is permissive and includes CJK, Cyrillic, Arabic, etc.
fn is_domain_char(c: char) -> bool {
    is_non_punctuation(c)
}

/// Check if a character is valid in a UWP (URL without protocol) domain.
fn is_uwp_domain_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || is_latin_accent(c)
}

/// Match a punycode domain segment (xn--...).
/// Punycode labels start with "xn--" (case insensitive) followed by alphanumerics and hyphens.
fn punycode_segment(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        tag_no_case("xn--"),
        take_while1(|c: char| c.is_ascii_alphanumeric() || c == '-'),
    )))(input)
}

/// Match a domain segment (label between dots).
/// Allows domain chars with hyphens and underscores between them.
/// The Pest grammar allows: subdomain_char = (domain_char ~ "_"? ~ domain_char) | domain_char
/// And domain_char allows hyphens between non_punctuation chars.
/// Punycode segments (xn--...) are handled specially.
fn domain_segment(input: &str) -> IResult<&str, &str> {
    // Try punycode first
    if let Ok(result) = punycode_segment(input) {
        return Ok(result);
    }

    // Regular domain segment - manual loop to avoid many0 allocation
    let mut chars = input.char_indices();

    // Must start with a domain char
    let (_, first) = chars.next().ok_or_else(|| {
        nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        ))
    })?;
    if !is_domain_char(first) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    let mut end_pos = first.len_utf8();

    // Continue matching: (hyphen/underscore + domain_char) or domain_char
    while let Some((pos, c)) = chars.next() {
        if c == '-' || c == '_' {
            // Must be followed by domain_char
            if let Some((_, next)) = chars.next() {
                if is_domain_char(next) {
                    end_pos = pos + c.len_utf8() + next.len_utf8();
                    continue;
                }
            }
            // Hyphen/underscore not followed by domain_char - stop before it
            break;
        } else if is_domain_char(c) {
            end_pos = pos + c.len_utf8();
        } else {
            break;
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Match a UWP domain segment.
fn uwp_domain_segment(input: &str) -> IResult<&str, &str> {
    // Manual loop to avoid many0 allocation
    let mut chars = input.char_indices();

    // Must start with a UWP domain char
    let (_, first) = chars.next().ok_or_else(|| {
        nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        ))
    })?;
    if !is_uwp_domain_char(first) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    let mut end_pos = first.len_utf8();

    // Continue matching: (hyphen/underscore + uwp_domain_char) or uwp_domain_char
    while let Some((pos, c)) = chars.next() {
        if c == '-' || c == '_' {
            // Must be followed by uwp_domain_char
            if let Some((_, next)) = chars.next() {
                if is_uwp_domain_char(next) {
                    end_pos = pos + c.len_utf8() + next.len_utf8();
                    continue;
                }
            }
            // Hyphen/underscore not followed by uwp_domain_char - stop before it
            break;
        } else if is_uwp_domain_char(c) {
            end_pos = pos + c.len_utf8();
        } else {
            break;
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Parse a domain: subdomain(s) + TLD.
/// Returns the matched domain and the TLD portion for validation.
fn domain(input: &str) -> IResult<&str, (&str, &str)> {
    let start = input;

    // Match one or more segments separated by dots, ending with a TLD
    let mut remaining = input;

    loop {
        // Try to match a segment
        let (after_segment, segment) = domain_segment(remaining)?;

        // Check for a dot
        if let Ok((after_dot, _)) = char::<&str, nom::error::Error<&str>>('.')(after_segment) {
            // Peek ahead: is there a valid segment after the dot?
            // If not, this dot is trailing punctuation, not part of the domain
            if domain_segment(after_dot).is_err() {
                // No valid segment after dot - treat current segment as TLD
                let consumed = start.len() - after_segment.len();
                let domain_str = &start[..consumed];
                return Ok((after_segment, (domain_str, segment)));
            }
            remaining = after_dot;
        } else {
            // No more dots - this segment should be the TLD
            let consumed = start.len() - after_segment.len();
            let domain_str = &start[..consumed];
            return Ok((after_segment, (domain_str, segment)));
        }
    }
}

/// Parse t.co domain specifically.
fn tco_domain(input: &str) -> IResult<&str, &str> {
    tag("t.co")(input)
}

/// Parse a t.co URL.
fn tco_url(input: &str) -> IResult<&str, (&str, usize, usize)> {
    let start = input;

    let (remaining, _) = protocol(input)?;
    let host_start = start.len() - remaining.len();

    let (remaining, _) = tco_domain(remaining)?;
    let host_end = start.len() - remaining.len();

    // Optional path: / followed by up to 40 alphanumeric chars
    let remaining =
        if let Ok((after_slash, _)) = char::<&str, nom::error::Error<&str>>('/')(remaining) {
            let (after_path, _) = take_while(|c: char| c.is_ascii_alphanumeric())(after_slash)?;
            // Check the path length (1-40 chars)
            let path_len = after_slash.len() - after_path.len();
            if path_len > 40 {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Verify,
                )));
            }
            after_path
        } else {
            remaining
        };

    // Optional query and fragment
    let remaining = if let Ok((after_query, _)) = query(remaining) {
        if let Ok((after_fragment, _)) = fragment(after_query) {
            after_fragment
        } else {
            after_query
        }
    } else {
        remaining
    };

    let consumed = start.len() - remaining.len();
    Ok((remaining, (&start[..consumed], host_start, host_end)))
}

/// Parse a normal URL (with protocol, not t.co).
fn normal_url(input: &str) -> IResult<&str, (&str, usize, usize)> {
    let start = input;

    let (remaining, _) = protocol(input)?;

    // Optional userinfo
    let remaining = if let Ok((after_userinfo, _)) = userinfo(remaining) {
        after_userinfo
    } else {
        remaining
    };

    let host_start = start.len() - remaining.len();

    // Host: domain, IPv4, or IPv6
    let (remaining, _) = host(remaining)?;
    let host_end = start.len() - remaining.len();

    // Optional port
    let remaining = if let Ok((after_port, _)) = port(remaining) {
        after_port
    } else {
        remaining
    };

    // Optional path
    let remaining = if let Ok((after_path, _)) = path(remaining) {
        after_path
    } else {
        remaining
    };

    // Optional query
    let remaining = if let Ok((after_query, _)) = query(remaining) {
        after_query
    } else {
        remaining
    };

    // Optional fragment
    let remaining = if let Ok((after_fragment, _)) = fragment(remaining) {
        after_fragment
    } else {
        remaining
    };

    let consumed = start.len() - remaining.len();
    Ok((remaining, (&start[..consumed], host_start, host_end)))
}

/// Check if a character is valid in userinfo (before the @).
fn is_userinfo_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "-._~:".contains(c) || is_cyrillic(c) || "!$&'()*+,;=".contains(c)
}

/// Parse userinfo (user:pass@).
fn userinfo(input: &str) -> IResult<&str, &str> {
    // Find the @ sign, validating characters along the way
    let mut chars = input.char_indices().peekable();

    while let Some((pos, c)) = chars.next() {
        if c == '@' {
            // Found the @, include it in the match
            return Ok((&input[pos + 1..], &input[..pos + 1]));
        } else if c == '%' {
            // Check for percent-encoded: %XX
            let h1 = chars.next();
            let h2 = chars.next();
            match (h1, h2) {
                (Some((_, c1)), Some((_, c2)))
                    if c1.is_ascii_hexdigit() && c2.is_ascii_hexdigit() =>
                {
                    // Valid percent-encoded, continue
                }
                _ => {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )));
                }
            }
        } else if is_userinfo_char(c) {
            // Valid userinfo char, continue
        } else {
            // Invalid character before finding @
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
    }

    // No @ found
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Char,
    )))
}

/// Parse host (domain or IP).
fn host(input: &str) -> IResult<&str, &str> {
    // Check for t.co first and reject it (handled by tco_url)
    if input.starts_with("t.co") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    alt((
        // Try IPv6 first
        recognize(ip_literal),
        // Then IPv4
        recognize(ipv4_address),
        // Then domain
        |i| domain(i).map(|(r, (d, _))| (r, d)),
    ))(input)
}

/// Parse IPv4 address.
fn ipv4_address(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        dec_octet,
        char('.'),
        dec_octet,
        char('.'),
        dec_octet,
        char('.'),
        dec_octet,
    )))(input)
}

/// Parse a decimal octet (0-255).
fn dec_octet(input: &str) -> IResult<&str, &str> {
    alt((
        // 250-255
        recognize(tuple((tag("25"), satisfy(|c| ('0'..='5').contains(&c))))),
        // 200-249
        recognize(tuple((
            char('2'),
            satisfy(|c| ('0'..='4').contains(&c)),
            satisfy(|c| c.is_ascii_digit()),
        ))),
        // 100-199
        recognize(tuple((
            char('1'),
            satisfy(|c| c.is_ascii_digit()),
            satisfy(|c| c.is_ascii_digit()),
        ))),
        // 10-99
        recognize(tuple((
            satisfy(|c| ('1'..='9').contains(&c)),
            satisfy(|c| c.is_ascii_digit()),
        ))),
        // 0-9
        recognize(satisfy(|c| c.is_ascii_digit())),
    ))(input)
}

/// Parse IPv6 literal [address].
fn ip_literal(input: &str) -> IResult<&str, &str> {
    recognize(delimited(char('['), ipv6_address, char(']')))(input)
}

/// Parse IPv6 address (simplified - matches hex groups separated by colons).
fn ipv6_address(input: &str) -> IResult<&str, &str> {
    // Simplified: just match hex digits and colons, with possible :: compression
    recognize(take_while1(|c: char| {
        c.is_ascii_hexdigit() || c == ':' || c == '.'
    }))(input)
}

/// Parse port number.
fn port(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        char(':'),
        satisfy(|c| ('1'..='9').contains(&c)),
        take_while(|c: char| c.is_ascii_digit()),
    )))(input)
}

/// Check if a character is valid at the end of a URL path.
fn is_url_path_end(c: char) -> bool {
    c.is_ascii_alphanumeric() || "=_-+".contains(c) || is_cyrillic(c) || is_latin_accent(c)
}

/// Check if a character is path punctuation.
fn is_path_punctuation(c: char) -> bool {
    "!*';:,.$%[]~|&@\u{2013}".contains(c)
}

/// Check if a character is valid in a URL path (excluding parens which are handled specially).
fn is_path_char(c: char) -> bool {
    c != '(' && c != ')' && (is_url_path_end(c) || is_path_punctuation(c) || c == '/')
}

/// Parse URL path.
fn path(input: &str) -> IResult<&str, &str> {
    // Path must start with /
    if !input.starts_with('/') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    let mut end_pos = 1; // Start after the initial /
    let mut chars = input[1..].char_indices().peekable();
    let mut paren_depth: i32 = 0;

    while let Some((pos, c)) = chars.next() {
        let abs_pos = pos + 1; // Adjust for skipped initial /

        if c == '(' {
            paren_depth += 1;
            end_pos = abs_pos + 1;
        } else if c == ')' {
            if paren_depth > 0 {
                paren_depth -= 1;
                end_pos = abs_pos + 1;
            } else {
                // Unbalanced ) - stop before it
                break;
            }
        } else if is_path_char(c) {
            end_pos = abs_pos + c.len_utf8();
        } else {
            break;
        }
    }

    // If we have unbalanced open parens, we need to backtrack to exclude them
    // and everything after the last balanced position
    if paren_depth > 0 {
        // Find the position of the first unbalanced open paren
        let mut depth: i32 = 0;
        let mut last_balanced_end = 1; // After the initial /

        for (pos, c) in input[1..].char_indices() {
            let abs_pos = pos + 1;
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
            }

            if depth == 0 {
                if is_path_char(c) || c == ')' {
                    last_balanced_end = abs_pos + c.len_utf8();
                }
            }

            if abs_pos + c.len_utf8() >= end_pos {
                break;
            }
        }

        end_pos = last_balanced_end;
    }

    let matched = &input[..end_pos];

    // Trim trailing punctuation that shouldn't be part of the URL
    let trimmed = matched.trim_end_matches(|c: char| is_path_punctuation(c) && c != '/');

    Ok((&input[trimmed.len()..], trimmed))
}

/// Parse query string.
/// Grammar: query = _{ "?" ~ query_char+ | "?" ~ &"#" }
/// Where: query_char = _{ query_end_char | query_punctuation_char+ ~ &query_end_char }
fn query(input: &str) -> IResult<&str, &str> {
    // Must start with ?
    let (after_q, _) = char::<&str, nom::error::Error<&str>>('?')(input)?;

    // Check if followed by # (valid: ?#fragment)
    if after_q.starts_with('#') {
        return Ok((after_q, &input[..1])); // Just the "?"
    }

    // Parse query chars: end chars are always valid, punctuation only if followed by end char
    let mut remaining = after_q;
    let mut last_valid_end = 0usize; // Position of last valid end (after end_char)

    loop {
        if let Some(c) = remaining.chars().next() {
            if is_query_end_char(c) {
                remaining = &remaining[c.len_utf8()..];
                last_valid_end = after_q.len() - remaining.len();
            } else if is_query_punctuation(c) {
                // Punctuation is tentatively consumed, but only valid if followed by end_char
                remaining = &remaining[c.len_utf8()..];
                // Continue consuming punctuation
            } else {
                // Not a query char - stop
                break;
            }
        } else {
            break;
        }
    }

    // Must have at least one valid query char
    if last_valid_end == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    // Return up to the last valid end position
    let consumed = 1 + last_valid_end; // +1 for the ?
    Ok((&input[consumed..], &input[..consumed]))
}

fn is_query_end_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || "-_&=/+".contains(c)
}

fn is_query_punctuation(c: char) -> bool {
    "!?*'();:$%[].~|@,".contains(c)
}

/// Parse fragment.
fn fragment(input: &str) -> IResult<&str, &str> {
    recognize(preceded(
        char('#'),
        take_while(|c: char| {
            c.is_ascii_alphanumeric() || "-_&=/+#".contains(c) || is_query_punctuation(c)
        }),
    ))(input)
}

/// Check if a character can be part of a Unicode TLD.
fn is_unicode_tld_char(c: char) -> bool {
    // Allow Unicode letters (for TLDs like みんな, 中国, etc.)
    // but not spaces, punctuation, or control characters
    !common::is_space(c)
        && !common::is_punctuation(c)
        && !c.is_ascii()
        && !common::is_invalid_char(c)
}

/// Parse a Unicode TLD segment (for TLDs like みんな).
fn unicode_tld_segment(input: &str) -> IResult<&str, &str> {
    take_while1(is_unicode_tld_char)(input)
}

/// Parse a URL without protocol.
fn uwp_domain_and_tld(input: &str) -> IResult<&str, (&str, &str)> {
    let start = input;

    // Check for a dot before space/EOF (early rejection)
    let has_dot = input
        .chars()
        .take_while(|c| !common::is_space(*c))
        .any(|c| c == '.');
    if !has_dot {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Verify,
        )));
    }

    // Match segments separated by dots
    let mut remaining = input;

    loop {
        // Try to match a segment (ASCII first, then Unicode for TLD)
        let segment_result = uwp_domain_segment(remaining);

        match segment_result {
            Ok((after_segment, _)) => {
                // Check for a dot
                if let Ok((after_dot, _)) =
                    char::<&str, nom::error::Error<&str>>('.')(after_segment)
                {
                    remaining = after_dot;
                } else {
                    // No more dots - this segment is the TLD
                    // Check for invalid TLD suffix (alphanumeric or @)
                    if let Some(next_char) = after_segment.chars().next() {
                        if is_invalid_tld_suffix(next_char) {
                            return Err(nom::Err::Error(nom::error::Error::new(
                                input,
                                nom::error::ErrorKind::Verify,
                            )));
                        }
                    }

                    let consumed = start.len() - after_segment.len();
                    let domain_str = &start[..consumed];

                    // Find the TLD (everything after the last dot)
                    if let Some(dot_idx) = domain_str.rfind('.') {
                        let tld_str = &domain_str[dot_idx + 1..];
                        return Ok((after_segment, (domain_str, tld_str)));
                    } else {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Verify,
                        )));
                    }
                }
            }
            Err(_) => {
                // ASCII segment failed - try Unicode TLD segment
                // This handles cases like "twitter.みんなです" where みんな is the TLD
                if let Ok((after_unicode, _)) = unicode_tld_segment(remaining) {
                    // Check for invalid TLD suffix (alphanumeric or @)
                    if let Some(next_char) = after_unicode.chars().next() {
                        if is_invalid_tld_suffix(next_char) {
                            return Err(nom::Err::Error(nom::error::Error::new(
                                input,
                                nom::error::ErrorKind::Verify,
                            )));
                        }
                    }

                    let consumed = start.len() - after_unicode.len();
                    let domain_str = &start[..consumed];

                    if let Some(dot_idx) = domain_str.rfind('.') {
                        let tld_str = &domain_str[dot_idx + 1..];
                        return Ok((after_unicode, (domain_str, tld_str)));
                    }
                }

                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Verify,
                )));
            }
        }
    }
}

/// Parse a URL without protocol (UWP).
/// Returns (matched_url, host_start, host_end).
pub fn parse_url_without_protocol(input: &str) -> IResult<&str, (&str, usize, usize)> {
    let start = input;

    // Parse domain and TLD
    let (remaining, (_domain_str, _tld)) = uwp_domain_and_tld(input)?;
    let host_end = start.len() - remaining.len();

    // Optional port
    let remaining = if let Ok((after_port, _)) = port(remaining) {
        after_port
    } else {
        remaining
    };

    // Optional path
    let remaining = if let Ok((after_path, _)) = path(remaining) {
        after_path
    } else {
        remaining
    };

    // Optional query
    let remaining = if let Ok((after_query, _)) = query(remaining) {
        after_query
    } else {
        remaining
    };

    // Optional fragment
    let remaining = if let Ok((after_fragment, _)) = fragment(remaining) {
        after_fragment
    } else {
        remaining
    };

    let consumed = start.len() - remaining.len();
    Ok((remaining, (&start[..consumed], 0, host_end)))
}

/// Parse a full URL (with protocol).
/// Returns (matched_url, host_start, host_end).
pub fn parse_url(input: &str) -> IResult<&str, (&str, usize, usize)> {
    // Try t.co first
    if let Ok(result) = tco_url(input) {
        return Ok(result);
    }

    // Then normal URL
    normal_url(input)
}

/// Check if the character before a potential URL is valid.
pub fn valid_url_predecessor(c: char) -> bool {
    // URLs should not be preceded by @, #, or $
    c != '@' && c != '#' && c != '$' && c != '\u{ff20}' && c != '\u{ff03}'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol() {
        assert!(protocol("http://").is_ok());
        assert!(protocol("https://").is_ok());
        assert!(protocol("HTTP://").is_ok());
        assert!(protocol("HTTPS://").is_ok());
        assert!(protocol("ftp://").is_err());
    }

    #[test]
    fn test_simple_url() {
        let result = parse_url("http://example.com");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "http://example.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_url_with_path() {
        let result = parse_url("https://example.com/path/to/page");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "https://example.com/path/to/page");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_url_with_query() {
        let result = parse_url("http://example.com?foo=bar");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "http://example.com?foo=bar");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_tco_url() {
        let result = parse_url("https://t.co/abc123");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "https://t.co/abc123");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_url_without_protocol() {
        let result = parse_url_without_protocol("example.com");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "example.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_url_without_protocol_with_path() {
        let result = parse_url_without_protocol("example.com/path");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "example.com/path");
        assert_eq!(remaining, "");
    }
}
