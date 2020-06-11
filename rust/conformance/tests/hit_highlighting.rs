// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use serde_derive::{Serialize, Deserialize};
use twitter_text::hit_highlighter::HitHighlighter;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assertion {
    pub description: String,
    pub text: String,
    pub hits: Vec<(usize, usize)>,
    pub expected: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tests {
    pub plain_text: Vec<Assertion>,
    pub with_links: Vec<Assertion>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub tests: Tests
}

const MANIFEST_YML: &str = include_str!("hit_highlighting.yml");

#[test]
fn hit_highlighting() {
    let manifest: Manifest = serde_yaml::from_str(MANIFEST_YML).expect("Error parsing yaml");

    for assertion in  manifest.tests.plain_text {
        let highlighter = HitHighlighter::new();
        let text = highlighter.highlight(&assertion.text, assertion.hits);
        assert_eq!(text, assertion.expected, "{}", assertion.description);
    }

    for assertion in  manifest.tests.with_links {
        let highlighter = HitHighlighter::new();
        let text = highlighter.highlight(&assertion.text, assertion.hits);
        assert_eq!(text, assertion.expected, "{}", assertion.description);
    }
}
