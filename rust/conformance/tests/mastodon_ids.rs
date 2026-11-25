// Copyright 2026 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Checks Mastodon handles (`@username@domain`) against `mastodon_ids.yml`,
//! following Mastodon's `Account::MENTION_RE`.
//!
//! - `mastodon_ids`: a case is valid when the whole text is extracted as
//!   exactly one federated mention.
//! - `mastodon_mentions`: the federated mentions extracted from running text,
//!   which exercises the lookbehind `(?<![=/[:word:]])` before the `@`.

use serde_derive::{Deserialize, Serialize};
use twitter_text::entity::Type;
use twitter_text::extractor::Extract;
use twitter_text::extractor::Extractor;
use twitter_text::ParserBackend;

fn all_parser_backends() -> [ParserBackend; 3] {
    [
        ParserBackend::External,
        ParserBackend::Pest,
        ParserBackend::Nom,
    ]
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assertion {
    pub description: String,
    pub text: String,
    pub valid: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MentionAssertion {
    pub description: String,
    pub text: String,
    pub expected: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tests {
    pub mastodon_ids: Vec<Assertion>,
    pub mastodon_mentions: Vec<MentionAssertion>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub tests: Tests,
}

fn manifest() -> Manifest {
    serde_yaml_ng::from_str(include_str!("mastodon_ids.yml")).expect("Error parsing yaml")
}

#[test]
fn mastodon_ids() {
    let manifest = manifest();
    for parser_backend in all_parser_backends() {
        let extractor = Extractor::with_parser_backend(parser_backend);
        let mut failures = Vec::new();
        for test in &manifest.tests.mastodon_ids {
            let entities = extractor.extract_federated_mentions_with_indices(&test.text);
            let valid = entities.len() == 1
                && entities[0].get_type() == Type::FEDERATEDMENTION
                && entities[0].get_value() == test.text;
            if valid != test.valid {
                let got: Vec<String> = entities
                    .iter()
                    .map(|e| format!("{:?} {:?}", e.get_type(), e.get_value()))
                    .collect();
                failures.push(format!(
                    "{}: {:?} expected valid={} but extracted {:?}",
                    test.description, test.text, test.valid, got
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{:?} backend failed {} case(s):\n{}",
            parser_backend,
            failures.len(),
            failures.join("\n")
        );
    }
}

#[test]
fn mastodon_mentions() {
    let manifest = manifest();
    for parser_backend in all_parser_backends() {
        let extractor = Extractor::with_parser_backend(parser_backend);
        let mut failures = Vec::new();
        for test in &manifest.tests.mastodon_mentions {
            let entities = extractor.extract_federated_mentions_with_indices(&test.text);
            let federated: Vec<&str> = entities
                .iter()
                .filter(|e| e.get_type() == Type::FEDERATEDMENTION)
                .map(|e| e.get_value())
                .collect();
            if federated != test.expected {
                failures.push(format!(
                    "{}: {:?} expected {:?} but extracted {:?}",
                    test.description, test.text, test.expected, federated
                ));
            }
        }
        assert!(
            failures.is_empty(),
            "{:?} backend failed {} case(s):\n{}",
            parser_backend,
            failures.len(),
            failures.join("\n")
        );
    }
}
