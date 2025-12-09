// Copyright 2019 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use serde_derive::{Deserialize, Serialize};
use twitter_text::autolinker::Autolinker;
use twitter_text::ExternalValidator;

/// Returns all ExternalValidator variants for testing both backends.
fn all_external_validators() -> [ExternalValidator; 3] {
    [
        ExternalValidator::External,
        ExternalValidator::Pest,
        ExternalValidator::Nom,
    ]
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assertion {
    pub description: String,
    pub text: String,
    pub expected: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonAssertion {
    pub description: String,
    pub text: String,
    pub json: String,
    pub expected: String,
}

// TODO: json
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tests {
    pub usernames: Vec<Assertion>,
    pub lists: Vec<Assertion>,
    pub hashtags: Vec<Assertion>,
    pub urls: Vec<Assertion>,
    pub cashtags: Vec<Assertion>,
    pub all: Vec<Assertion>,
    pub json: Vec<JsonAssertion>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub tests: Tests,
}

const MANIFEST_YML: &str = include_str!("autolink.yml");

#[test]
fn autolink() {
    for external_validator in all_external_validators() {
        let manifest: Manifest = serde_yaml_ng::from_str(MANIFEST_YML).expect("Error parsing yaml");

        for assertion in manifest.tests.usernames {
            let autolinker = Autolinker::with_external_validator(false, external_validator);
            let text = autolinker.autolink_usernames_and_lists(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.lists {
            let autolinker = Autolinker::with_external_validator(false, external_validator);
            let text = autolinker.autolink_usernames_and_lists(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.hashtags {
            let autolinker = Autolinker::with_external_validator(false, external_validator);
            let text = autolinker.autolink_hashtags(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.urls {
            let autolinker = Autolinker::with_external_validator(false, external_validator);
            let text = autolinker.autolink_urls(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.cashtags {
            let autolinker = Autolinker::with_external_validator(false, external_validator);
            let text = autolinker.autolink_cashtags(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.all {
            let autolinker = Autolinker::with_external_validator(false, external_validator);
            let text = autolinker.autolink(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }
    } // end for external_validator
}
