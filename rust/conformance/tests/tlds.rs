// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use serde_derive::{Deserialize, Serialize};
use twitter_text::extractor::Extractor;
use twitter_text::ExternalValidator;

/// Returns all ExternalValidator variants for testing both backends.
fn all_external_validators() -> [ExternalValidator; 2] {
    [ExternalValidator::External, ExternalValidator::Pest]
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

fn tld_check(assertions: &[Assertion], external_validator: ExternalValidator) {
    let extractor = Extractor::with_external_validator(external_validator);
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
    for external_validator in all_external_validators() {
        let manifest: Manifest = serde_yaml_ng::from_str(MANIFEST_YML).expect("Error parsing yaml");
        tld_check(&manifest.tests.country, external_validator);
        tld_check(&manifest.tests.generic, external_validator);
    }
}
