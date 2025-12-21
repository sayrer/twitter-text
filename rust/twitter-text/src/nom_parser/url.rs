// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! URL parsing for the nom parser.
//!
//! URLs can have two forms:
//! 1. With protocol: http:// or https:// followed by host, path, query, fragment
//! 2. Without protocol: domain.tld followed by optional path, query, fragment
//!
//! TLD validation is done externally via phf lookup.

use nom::IResult;

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

/// Parse the http:// or https:// protocol prefix using direct byte scanning.
fn protocol(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Check for "http" (case insensitive)
    if bytes.len() < 7 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let h = bytes[0];
    let t = bytes[1];
    let t2 = bytes[2];
    let p = bytes[3];

    if !((h == b'h' || h == b'H')
        && (t == b't' || t == b'T')
        && (t2 == b't' || t2 == b'T')
        && (p == b'p' || p == b'P'))
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    // Check for optional 's' and "://"
    let (proto_len, remaining_start) = if bytes[4] == b's' || bytes[4] == b'S' {
        // https://
        if bytes.len() < 8 || bytes[5] != b':' || bytes[6] != b'/' || bytes[7] != b'/' {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Tag,
            )));
        }
        (8, 8)
    } else if bytes[4] == b':' && bytes.len() >= 7 && bytes[5] == b'/' && bytes[6] == b'/' {
        // http://
        (7, 7)
    } else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };

    Ok((&input[remaining_start..], &input[..proto_len]))
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

/// Match a punycode domain segment (xn--...) using direct byte scanning.
/// Punycode labels start with "xn--" (case insensitive) followed by alphanumerics and hyphens.
fn punycode_segment(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Check for "xn--" prefix (case insensitive)
    if bytes.len() < 5 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    let x = bytes[0];
    let n = bytes[1];

    if !((x == b'x' || x == b'X')
        && (n == b'n' || n == b'N')
        && bytes[2] == b'-'
        && bytes[3] == b'-')
    {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }

    // Must have at least one alphanumeric/hyphen after xn--
    let mut end_pos = 4;
    while end_pos < bytes.len() {
        let b = bytes[end_pos];
        if b.is_ascii_alphanumeric() || b == b'-' {
            end_pos += 1;
        } else {
            break;
        }
    }

    if end_pos == 4 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Check if a byte is an ASCII domain character (alphanumeric).
#[inline(always)]
fn is_ascii_domain_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric()
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

    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Fast path: check if first byte is ASCII
    let first_byte = bytes[0];
    if first_byte < 128 {
        // ASCII path - much faster
        if !is_ascii_domain_byte(first_byte) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Satisfy,
            )));
        }

        let mut end_pos = 1;
        let mut i = 1;

        while i < bytes.len() {
            let b = bytes[i];
            if b >= 128 {
                // Non-ASCII: fall back to char-based parsing for rest
                // SAFETY: end_pos is a valid UTF-8 boundary (ASCII chars are 1 byte)
                return domain_segment_continue_unicode(input, end_pos);
            }

            if b == b'-' || b == b'_' {
                // Must be followed by domain char
                if i + 1 < bytes.len() {
                    let next = bytes[i + 1];
                    if next >= 128 {
                        // Non-ASCII after hyphen - fall back
                        return domain_segment_continue_unicode(input, end_pos);
                    }
                    if is_ascii_domain_byte(next) {
                        end_pos = i + 2;
                        i += 2;
                        continue;
                    }
                }
                // Hyphen/underscore not followed by domain char - stop before it
                break;
            } else if is_ascii_domain_byte(b) {
                end_pos = i + 1;
                i += 1;
            } else {
                break;
            }
        }

        Ok((&input[end_pos..], &input[..end_pos]))
    } else {
        // Non-ASCII first char - use full Unicode path
        domain_segment_unicode(input)
    }
}

/// Domain segment parsing for non-ASCII input (full Unicode support).
fn domain_segment_unicode(input: &str) -> IResult<&str, &str> {
    let mut chars = input.chars();

    // Must start with a domain char
    let first = chars.next().ok_or_else(|| {
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

    // Continue matching
    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        if c == '-' || c == '_' {
            // Must be followed by domain_char
            if let Some(next) = chars.next() {
                if is_domain_char(next) {
                    end_pos += c.len_utf8() + next.len_utf8();
                    continue;
                }
            }
            break;
        } else if is_domain_char(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Continue domain segment parsing after switching from ASCII to Unicode.
fn domain_segment_continue_unicode(input: &str, start_pos: usize) -> IResult<&str, &str> {
    let mut end_pos = start_pos;
    let mut chars = input[start_pos..].chars();

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        if c == '-' || c == '_' {
            // Must be followed by domain_char
            if let Some(next) = chars.next() {
                if is_domain_char(next) {
                    end_pos += c.len_utf8() + next.len_utf8();
                    continue;
                }
            }
            break;
        } else if is_domain_char(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Check if a byte is a valid ASCII UWP domain character.
#[inline(always)]
fn is_ascii_uwp_domain_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric()
}

/// Match a UWP domain segment.
fn uwp_domain_segment(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Fast path: check if first byte is ASCII
    let first_byte = bytes[0];
    if first_byte < 128 {
        // ASCII path
        if !is_ascii_uwp_domain_byte(first_byte) {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Satisfy,
            )));
        }

        let mut end_pos = 1;
        let mut i = 1;

        while i < bytes.len() {
            let b = bytes[i];
            if b >= 128 {
                // Non-ASCII: check if it's a latin accent and continue
                return uwp_domain_segment_continue_unicode(input, end_pos);
            }

            if b == b'-' || b == b'_' {
                // Must be followed by uwp_domain char
                if i + 1 < bytes.len() {
                    let next = bytes[i + 1];
                    if next >= 128 {
                        // Non-ASCII after hyphen - fall back
                        return uwp_domain_segment_continue_unicode(input, end_pos);
                    }
                    if is_ascii_uwp_domain_byte(next) {
                        end_pos = i + 2;
                        i += 2;
                        continue;
                    }
                }
                // Hyphen/underscore not followed by domain char - stop before it
                break;
            } else if is_ascii_uwp_domain_byte(b) {
                end_pos = i + 1;
                i += 1;
            } else {
                break;
            }
        }

        Ok((&input[end_pos..], &input[..end_pos]))
    } else {
        // Non-ASCII first char - use full Unicode path
        uwp_domain_segment_unicode(input)
    }
}

/// UWP domain segment parsing for non-ASCII input.
fn uwp_domain_segment_unicode(input: &str) -> IResult<&str, &str> {
    let mut chars = input.chars();

    // Must start with a UWP domain char
    let first = chars.next().ok_or_else(|| {
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

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        if c == '-' || c == '_' {
            if let Some(next) = chars.next() {
                if is_uwp_domain_char(next) {
                    end_pos += c.len_utf8() + next.len_utf8();
                    continue;
                }
            }
            break;
        } else if is_uwp_domain_char(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Continue UWP domain segment parsing after switching from ASCII to Unicode.
fn uwp_domain_segment_continue_unicode(input: &str, start_pos: usize) -> IResult<&str, &str> {
    let mut end_pos = start_pos;
    let mut chars = input[start_pos..].chars();

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        if c == '-' || c == '_' {
            if let Some(next) = chars.next() {
                if is_uwp_domain_char(next) {
                    end_pos += c.len_utf8() + next.len_utf8();
                    continue;
                }
            }
            break;
        } else if is_uwp_domain_char(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Parse a domain: subdomain(s) + TLD.
/// Returns the matched domain and the TLD portion for validation.
///
/// The segment immediately before the TLD (the "domain segment") must NOT contain underscores.
/// Subdomains (earlier segments) may contain underscores.
///
/// Valid: test_underscore.twitter.com (underscore in subdomain)
/// Invalid: foo.bar_baz.com (underscore in domain segment before TLD)
fn domain(input: &str) -> IResult<&str, (&str, &str)> {
    let start = input;

    // Match one or more segments separated by dots, ending with a TLD
    let mut remaining = input;
    let mut prev_segment: Option<&str> = None;

    loop {
        // Try to match a segment
        let (after_segment, segment) = domain_segment(remaining)?;

        // Check for a dot
        if after_segment.as_bytes().first() == Some(&b'.') {
            let after_dot = &after_segment[1..];
            // Peek ahead: is there a valid segment after the dot?
            // If not, this dot is trailing punctuation, not part of the domain
            if domain_segment(after_dot).is_err() {
                // No valid segment after dot - treat current segment as TLD
                // The previous segment (before current) is the domain segment - check for underscores
                if let Some(prev) = prev_segment {
                    if prev.contains('_') {
                        return Err(nom::Err::Error(nom::error::Error::new(
                            input,
                            nom::error::ErrorKind::Verify,
                        )));
                    }
                }
                let consumed = start.len() - after_segment.len();
                let domain_str = &start[..consumed];
                return Ok((after_segment, (domain_str, segment)));
            }
            prev_segment = Some(segment);
            remaining = after_dot;
        } else {
            // No more dots - this segment should be the TLD
            // The previous segment is the domain segment - check for underscores
            if let Some(prev) = prev_segment {
                if prev.contains('_') {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Verify,
                    )));
                }
            }
            let consumed = start.len() - after_segment.len();
            let domain_str = &start[..consumed];
            return Ok((after_segment, (domain_str, segment)));
        }
    }
}

/// Parse t.co domain specifically using direct byte scanning.
fn tco_domain(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();
    if bytes.len() >= 4
        && bytes[0] == b't'
        && bytes[1] == b'.'
        && bytes[2] == b'c'
        && bytes[3] == b'o'
    {
        Ok((&input[4..], &input[..4]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

/// Parse a t.co URL using direct byte scanning.
fn tco_url(input: &str) -> IResult<&str, (&str, usize, usize)> {
    let start = input;

    let (remaining, _) = protocol(input)?;
    let host_start = start.len() - remaining.len();

    let (remaining, _) = tco_domain(remaining)?;
    let host_end = start.len() - remaining.len();

    // Optional path: / followed by up to 40 alphanumeric chars
    let bytes = remaining.as_bytes();
    let remaining = if !bytes.is_empty() && bytes[0] == b'/' {
        let mut path_len = 0;
        let after_slash = &bytes[1..];
        for &b in after_slash.iter().take(41) {
            if b.is_ascii_alphanumeric() {
                path_len += 1;
            } else {
                break;
            }
        }
        if path_len > 40 {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }
        &remaining[1 + path_len..]
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

/// Check if input looks like an IPv4 address (starts with digits followed by dots).
/// This is used to prevent invalid IPs from being parsed as domains.
fn looks_like_ipv4(input: &str) -> bool {
    let bytes = input.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_digit() {
        return false;
    }
    // Check for pattern: digit(s).digit(s).digit(s).digit(s)
    let mut dots = 0;
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_digit() {
            i += 1;
        } else if b == b'.' {
            dots += 1;
            i += 1;
        } else {
            break;
        }
    }
    // If we have exactly 3 dots and only digits between them, it looks like an IP
    dots == 3
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

    // Try IPv6 first
    if let Ok(result) = ip_literal(input) {
        return Ok(result);
    }

    // Check if this looks like an IPv4 address
    if looks_like_ipv4(input) {
        // If it looks like an IP, it MUST be a valid IP - don't fall back to domain
        return ipv4_address(input);
    }

    // Otherwise, parse as domain
    domain(input).map(|(r, (d, _))| (r, d))
}

/// Parse IPv4 address using direct byte scanning.
fn ipv4_address(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();
    let mut pos = 0;

    for octet_num in 0..4 {
        // Parse decimal octet (0-255)
        if pos >= bytes.len() || !bytes[pos].is_ascii_digit() {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )));
        }

        let octet_start = pos;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }

        let octet_len = pos - octet_start;
        if octet_len == 0 || octet_len > 3 {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Digit,
            )));
        }

        // Validate octet value (0-255)
        let octet_str = &input[octet_start..pos];
        let octet_val: u16 = octet_str.parse().unwrap_or(256);
        if octet_val > 255 {
            return Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Verify,
            )));
        }

        // Check for dot separator (except after last octet)
        if octet_num < 3 {
            if pos >= bytes.len() || bytes[pos] != b'.' {
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Char,
                )));
            }
            pos += 1;
        }
    }

    Ok((&input[pos..], &input[..pos]))
}

/// Parse IPv6 literal [address] using direct byte scanning.
fn ip_literal(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    if bytes.is_empty() || bytes[0] != b'[' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    // Find closing bracket, matching hex digits, colons, and dots
    let mut pos = 1;
    while pos < bytes.len() {
        let b = bytes[pos];
        if b == b']' {
            if pos == 1 {
                // Empty brackets
                return Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Satisfy,
                )));
            }
            return Ok((&input[pos + 1..], &input[..pos + 1]));
        } else if b.is_ascii_hexdigit() || b == b':' || b == b'.' {
            pos += 1;
        } else {
            break;
        }
    }

    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Char,
    )))
}

/// Parse port number using direct byte scanning.
fn port(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Must start with ':'
    if bytes.is_empty() || bytes[0] != b':' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    // Must have at least one non-zero digit
    if bytes.len() < 2 || !matches!(bytes[1], b'1'..=b'9') {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    // Consume remaining digits
    let mut pos = 2;
    while pos < bytes.len() && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    Ok((&input[pos..], &input[..pos]))
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

/// Check if an ASCII byte is valid in a URL path (excluding parens).
#[inline(always)]
fn is_ascii_path_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric()
        || matches!(
            b,
            b'=' | b'_'
                | b'-'
                | b'+'
                | b'/'
                | b'!'
                | b'*'
                | b'\''
                | b';'
                | b':'
                | b','
                | b'.'
                | b'$'
                | b'%'
                | b'['
                | b']'
                | b'~'
                | b'|'
                | b'&'
                | b'@'
        )
}

/// Parse URL path.
fn path(input: &str) -> IResult<&str, &str> {
    // Path must start with /
    let bytes = input.as_bytes();
    if bytes.is_empty() || bytes[0] != b'/' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    let mut end_pos = 1; // Start after the initial /
    let mut paren_depth: i32 = 0;
    let mut i = 1;

    // Fast ASCII path
    while i < bytes.len() {
        let b = bytes[i];

        if b >= 128 {
            // Non-ASCII: fall back to char-based parsing
            return path_continue_unicode(input, end_pos, paren_depth);
        }

        if b == b'(' {
            paren_depth += 1;
            end_pos = i + 1;
            i += 1;
        } else if b == b')' {
            if paren_depth > 0 {
                paren_depth -= 1;
                end_pos = i + 1;
                i += 1;
            } else {
                // Unbalanced ) - stop before it
                break;
            }
        } else if is_ascii_path_byte(b) {
            end_pos = i + 1;
            i += 1;
        } else {
            break;
        }
    }

    // Handle unbalanced parens
    if paren_depth > 0 {
        end_pos = fixup_unbalanced_parens(input, end_pos);
    }

    let matched = &input[..end_pos];

    // Trim trailing punctuation that shouldn't be part of the URL
    let trimmed = matched.trim_end_matches(|c: char| is_path_punctuation(c) && c != '/');

    Ok((&input[trimmed.len()..], trimmed))
}

/// Continue path parsing after encountering non-ASCII.
fn path_continue_unicode(
    input: &str,
    start_pos: usize,
    mut paren_depth: i32,
) -> IResult<&str, &str> {
    let mut end_pos = start_pos;
    let mut chars = input[start_pos..].chars();

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        if c == '(' {
            paren_depth += 1;
            end_pos += 1;
        } else if c == ')' {
            if paren_depth > 0 {
                paren_depth -= 1;
                end_pos += 1;
            } else {
                break;
            }
        } else if is_path_char(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    // Handle unbalanced parens
    if paren_depth > 0 {
        end_pos = fixup_unbalanced_parens(input, end_pos);
    }

    let matched = &input[..end_pos];
    let trimmed = matched.trim_end_matches(|c: char| is_path_punctuation(c) && c != '/');

    Ok((&input[trimmed.len()..], trimmed))
}

/// Fix up end_pos when we have unbalanced open parens.
fn fixup_unbalanced_parens(input: &str, end_pos: usize) -> usize {
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

    last_balanced_end
}

/// Parse query string.
/// Grammar: query = _{ "?" ~ query_char+ | "?" ~ &"#" }
/// Where: query_char = _{ query_end_char | query_punctuation_char+ ~ &query_end_char }
fn query(input: &str) -> IResult<&str, &str> {
    // Must start with ?
    let bytes = input.as_bytes();
    if bytes.is_empty() || bytes[0] != b'?' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }
    let after_q = &input[1..];

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

/// Check if a byte is valid in a fragment.
#[inline(always)]
fn is_fragment_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric()
        || matches!(
            b,
            b'-' | b'_'
                | b'&'
                | b'='
                | b'/'
                | b'+'
                | b'#'
                | b'!'
                | b'?'
                | b'*'
                | b'\''
                | b'('
                | b')'
                | b';'
                | b':'
                | b'$'
                | b'%'
                | b'['
                | b']'
                | b'.'
                | b'~'
                | b'|'
                | b'@'
                | b','
        )
}

/// Parse fragment using direct byte scanning.
fn fragment(input: &str) -> IResult<&str, &str> {
    let bytes = input.as_bytes();

    // Must start with #
    if bytes.is_empty() || bytes[0] != b'#' {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Char,
        )));
    }

    // Consume valid fragment characters
    let mut pos = 1;
    while pos < bytes.len() && is_fragment_byte(bytes[pos]) {
        pos += 1;
    }

    Ok((&input[pos..], &input[..pos]))
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

/// Parse a Unicode TLD segment (for TLDs like みんな) using direct char iteration.
fn unicode_tld_segment(input: &str) -> IResult<&str, &str> {
    let mut end_pos = 0;

    for c in input.chars() {
        if is_unicode_tld_char(c) {
            end_pos += c.len_utf8();
        } else {
            break;
        }
    }

    if end_pos == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Satisfy,
        )));
    }

    Ok((&input[end_pos..], &input[..end_pos]))
}

/// Parse a URL without protocol.
fn uwp_domain_and_tld(input: &str) -> IResult<&str, (&str, &str)> {
    let start = input;

    // Check for a dot before space/EOF (early rejection)
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
                if after_segment.as_bytes().first() == Some(&b'.') {
                    remaining = &after_segment[1..];
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

    #[test]
    fn test_url_underscore_in_subdomain() {
        // Valid: underscore in subdomain (not adjacent to TLD)
        let result = parse_url("http://test_underscore.twitter.com");
        assert!(result.is_ok());
        let (remaining, (matched, _, _)) = result.unwrap();
        assert_eq!(matched, "http://test_underscore.twitter.com");
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_url_invalid_underscore_adjacent_to_tld() {
        // Invalid: underscore in the segment immediately before TLD
        let result = parse_url("http://domain-dash_2314352345_dfasd.foo-cow_4352.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_url_invalid_ip_octet() {
        // Invalid: 256 is not a valid IP octet (must be 0-255)
        let result = parse_url("http://256.1.2.3/");
        assert!(result.is_err());
    }
}
