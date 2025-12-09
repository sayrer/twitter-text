// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use serde_derive::{Deserialize, Serialize};
use twitter_text::extractor::Extractor;
use twitter_text::TldMatcher;

/// Returns all TldMatcher variants for testing both backends.
fn all_tld_matchers() -> [TldMatcher; 2] {
    [TldMatcher::External, TldMatcher::Pest]
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assertion {
    pub description: String,
    pub text: String,
    pub expected: Vec<serde_yaml_ng::Value>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tests {
    pub country: Vec<Assertion>,
    pub generic: Vec<Assertion>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub tests: Tests,
}

const MANIFEST_YML: &str = include_str!("tlds.yml");

fn tld_check(assertions: &[Assertion], tld_matcher: TldMatcher) {
    let extractor = Extractor::with_tld_matcher(tld_matcher);
    for assertion in assertions {
        let url_text = extractor.extract_urls(&assertion.text);
        assert!(
            !url_text.is_empty(),
            "No URLs extracted from '{}' - expected {:?} - {}",
            assertion.text,
            assertion.expected,
            assertion.description
        );
        assert_eq!(
            url_text[0],
            assertion.expected[0].as_str().unwrap(),
            "{}",
            assertion.description
        );
    }
}

#[test]
fn tlds() {
    for tld_matcher in all_tld_matchers() {
        let manifest: Manifest = serde_yaml_ng::from_str(MANIFEST_YML).expect("Error parsing yaml");
        tld_check(&manifest.tests.country, tld_matcher);
        tld_check(&manifest.tests.generic, tld_matcher);
    }
}
