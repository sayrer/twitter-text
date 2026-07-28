// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use std::char;
use std::io;
use std::io::BufRead;
use std::u32;

use pest::iterators::Pair;
use pest::Parser;
use twitter_text::ParserBackend;
use twitter_text_parser::twitter_text::full_pest::Rule as FullPestRule;
use twitter_text_parser::twitter_text::full_pest::TwitterTextFullPestParser;
use twitter_text_parser::twitter_text::Rule;
use twitter_text_parser::twitter_text::TwitterTextParser;

const EMOJI_TXT: &str = include_str!("unicode18-emoji-test.txt");

/// Base code points added in Emoji 18.0 that the `emojis` crate does not know
/// about yet. 0.9.0 is the newest release and its data stops at Emoji 17.0, so
/// `is_valid_emoji` in the extractor rejects any sequence built from these.
///
/// This list exists only to keep the weighting test honest while 18.0 is a
/// draft; `test_pending_emoji_18_still_unsupported` fails once the `emojis`
/// crate catches up, at which point delete both this constant and that test.
const PENDING_EMOJI_18: &[char] = &[
    '\u{1faeb}', // cracking face
    '\u{1faf9}', // leftwards thumb sign
    '\u{1fafa}', // rightwards thumb sign
    '\u{1facc}', // monarch butterfly
    '\u{1fadd}', // pickle
    '\u{1f6d9}', // lighthouse
    '\u{1fa8b}', // meteor
    '\u{1fa8c}', // eraser
    '\u{1fa8d}', // net with handle
];

fn is_pending_emoji_18(s: &str) -> bool {
    s.chars().any(|c| PENDING_EMOJI_18.contains(&c))
}

fn build_emoji_strings() -> Vec<String> {
    let f = io::BufReader::new(EMOJI_TXT.as_bytes());
    f.lines()
        .filter_map(|result| result.ok())
        .filter(|s| !s.starts_with("#"))
        .filter(|s| !(s.trim().len() == 0))
        .map(|s| {
            let mut test_str = String::from("test|");
            match s.find(";") {
                Some(ss) => {
                    let code: String = s.chars().take(ss).collect();
                    let codes = code.trim().split(" ");
                    let chars: Vec<char> = codes
                        .map(|sss| {
                            let digit = u32::from_str_radix(sss, 16).unwrap();
                            char::from_u32(digit).unwrap()
                        })
                        .collect();

                    for c in chars {
                        test_str.push(c);
                    }
                    test_str.push_str("|test");
                }
                None => {}
            }

            test_str
        })
        .collect()
}

#[test]
fn test_emoji_parsing() {
    let emoji_strings = build_emoji_strings();

    let mut failures: Vec<String> = Vec::new();
    for s in emoji_strings {
        let tree = TwitterTextParser::parse(Rule::tweet, &s).expect("Expected successful parse.");
        let tokens: Vec<Pair<Rule>> = tree.flatten().collect();
        let len = tokens.len();
        if len != 3 {
            failures.push(format!("Expected 3 tokens, got {} for: {:?}", len, s));
        }
    }

    if !failures.is_empty() {
        for f in &failures[..std::cmp::min(20, failures.len())] {
            eprintln!("{}", f);
        }
        panic!(
            "{} emoji failed to parse correctly (showing first 20)",
            failures.len()
        );
    }
}

#[test]
fn test_emoji_parsing_full_pest() {
    let emoji_strings = build_emoji_strings();

    let mut failures: Vec<String> = Vec::new();
    for s in emoji_strings {
        // Use the full Pest parser which has the complete emoji grammar
        let tree = TwitterTextFullPestParser::parse(FullPestRule::tweet, &s)
            .expect("Expected successful parse.");
        let tokens: Vec<Pair<FullPestRule>> = tree.flatten().collect();
        let len = tokens.len();
        if len != 3 {
            failures.push(format!("Expected 3 tokens, got {} for: {:?}", len, s));
        }
    }

    if !failures.is_empty() {
        for f in &failures[..std::cmp::min(20, failures.len())] {
            eprintln!("{}", f);
        }
        panic!(
            "{} emoji failed to parse correctly with full_pest parser (showing first 20)",
            failures.len()
        );
    }
}

/// Bare, fully-qualified emoji sequences from the test data.
///
/// Only fully-qualified rows are used: minimally-qualified and unqualified
/// forms are missing variation selectors, so backends may legitimately weigh
/// them differently.
fn build_fully_qualified_sequences() -> Vec<String> {
    let f = io::BufReader::new(EMOJI_TXT.as_bytes());
    f.lines()
        .filter_map(|result| result.ok())
        .filter(|s| !s.starts_with("#"))
        .filter(|s| s.contains("; fully-qualified"))
        .filter_map(|s| {
            let semi = s.find(";")?;
            let code: String = s.chars().take(semi).collect();
            code.trim()
                .split(" ")
                .map(|c| u32::from_str_radix(c, 16).ok().and_then(char::from_u32))
                .collect::<Option<String>>()
        })
        .collect()
}

fn weighted_lengths(s: &str) -> (i32, i32, i32) {
    let config = twitter_text_config::config_v3();
    let pest = twitter_text::parse_with_parser_backend(s, config, false, ParserBackend::Pest);
    let external =
        twitter_text::parse_with_parser_backend(s, config, false, ParserBackend::External);
    let nom = twitter_text::parse_with_parser_backend(s, config, false, ParserBackend::Nom);
    (
        pest.weighted_length,
        external.weighted_length,
        nom.weighted_length,
    )
}

/// Every parser backend must agree on the weighted length of an emoji.
///
/// Tokenization alone is not enough to prove an emoji is handled: the Pest
/// backend weighs straight from the grammar, while the External and Nom
/// backends additionally run each match through `is_valid_emoji`, which
/// delegates to the `emojis` crate. When the grammar accepts a sequence the
/// crate does not know, those backends fall back to counting it as individual
/// characters and the weights diverge -- a two code point sequence weighs 4
/// instead of 2. `test_emoji_parsing` cannot see that, because both backends
/// still produce one token.
#[test]
fn test_emoji_weighting_backends_agree() {
    let mut failures: Vec<String> = Vec::new();

    for s in build_fully_qualified_sequences() {
        if is_pending_emoji_18(&s) {
            continue;
        }
        let (pest, external, nom) = weighted_lengths(&s);
        if pest != external || pest != nom {
            failures.push(format!(
                "weighted_length disagreement for {:?} ({} code points): pest={}, external={}, nom={}",
                s,
                s.chars().count(),
                pest,
                external,
                nom
            ));
        }
    }

    if !failures.is_empty() {
        for f in &failures[..std::cmp::min(20, failures.len())] {
            eprintln!("{}", f);
        }
        panic!(
            "{} emoji weighed inconsistently across backends (showing first 20)",
            failures.len()
        );
    }
}

/// Guards the `PENDING_EMOJI_18` allowance so it cannot outlive its purpose.
///
/// Every multi code point sequence built from a pending base is expected to
/// *still* disagree across backends. Once the `emojis` crate ships Emoji 18.0
/// data these will agree, this test will fail, and the correct response is to
/// delete `PENDING_EMOJI_18`, the skip in
/// `test_emoji_weighting_backends_agree`, and this test.
#[test]
fn test_pending_emoji_18_still_unsupported() {
    let pending: Vec<String> = build_fully_qualified_sequences()
        .into_iter()
        .filter(|s| is_pending_emoji_18(s) && s.chars().count() > 1)
        .collect();

    assert!(
        !pending.is_empty(),
        "No pending Emoji 18.0 sequences found in the test data. If the data no \
         longer contains them, delete PENDING_EMOJI_18 and this test."
    );

    let now_supported: Vec<&String> = pending
        .iter()
        .filter(|s| {
            let (pest, external, nom) = weighted_lengths(s);
            pest == external && pest == nom
        })
        .collect();

    assert!(
        now_supported.is_empty(),
        "The emojis crate now recognizes {} of {} pending Emoji 18.0 sequences \
         (e.g. {:?}). Delete PENDING_EMOJI_18, the skip in \
         test_emoji_weighting_backends_agree, and this test.",
        now_supported.len(),
        pending.len(),
        now_supported[0]
    );
}
