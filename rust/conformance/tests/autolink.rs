// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use serde_derive::{Deserialize, Serialize};
use twitter_text::autolinker::Autolinker;
use twitter_text::entity::{Entity, Type};
use twitter_text::ParserBackend;

/// Returns all ParserBackend variants for testing both backends.
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
    pub expected: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct JsonAssertion {
    pub description: String,
    pub text: String,
    pub json: String,
    pub expected: String,
}

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
    for parser_backend in all_parser_backends() {
        let manifest: Manifest = serde_yaml_ng::from_str(MANIFEST_YML).expect("Error parsing yaml");

        for assertion in manifest.tests.usernames {
            let autolinker = Autolinker::with_parser_backend(false, parser_backend);
            let text = autolinker.autolink_usernames_and_lists(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.lists {
            let autolinker = Autolinker::with_parser_backend(false, parser_backend);
            let text = autolinker.autolink_usernames_and_lists(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.hashtags {
            let autolinker = Autolinker::with_parser_backend(false, parser_backend);
            let text = autolinker.autolink_hashtags(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.urls {
            let autolinker = Autolinker::with_parser_backend(false, parser_backend);
            let text = autolinker.autolink_urls(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.cashtags {
            let autolinker = Autolinker::with_parser_backend(false, parser_backend);
            let text = autolinker.autolink_cashtags(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }

        for assertion in manifest.tests.all {
            let autolinker = Autolinker::with_parser_backend(false, parser_backend);
            let text = autolinker.autolink(&assertion.text);
            assert_eq!(text, assertion.expected, "{}", assertion.description);
        }
    } // end for parser_backend

    // JSON entity tests: entities are provided as pre-parsed JSON (Twitter API format)
    let manifest: Manifest = serde_yaml_ng::from_str(MANIFEST_YML).expect("Error parsing yaml");
    for assertion in manifest.tests.json {
        let entities_json: JsonEntities =
            serde_json::from_str(&assertion.json).expect("Error parsing entity JSON");
        let mut entities = Vec::new();
        for m in &entities_json.user_mentions {
            entities.push(Entity {
                t: Type::MENTION,
                start: m.indices[0],
                end: m.indices[1],
                value: &m.screen_name,
                list_slug: "",
                display_url: "",
                expanded_url: "",
            });
        }
        for h in &entities_json.hashtags {
            entities.push(Entity {
                t: Type::HASHTAG,
                start: h.indices[0],
                end: h.indices[1],
                value: &h.text,
                list_slug: "",
                display_url: "",
                expanded_url: "",
            });
        }
        for u in &entities_json.urls {
            entities.push(Entity {
                t: Type::URL,
                start: u.indices[0],
                end: u.indices[1],
                value: &u.url,
                list_slug: "",
                display_url: u.display_url.as_deref().unwrap_or(""),
                expanded_url: u.expanded_url.as_deref().unwrap_or(""),
            });
        }
        entities.sort_by_key(|e| e.start);

        let autolinker = Autolinker::new(false);
        let result = autolinker.autolink_entities(&assertion.text, &entities);
        assert_eq!(result, assertion.expected, "{}", assertion.description);
    }
}

#[derive(Debug, Deserialize)]
struct JsonEntities {
    #[serde(default)]
    hashtags: Vec<JsonHashtag>,
    #[serde(default)]
    urls: Vec<JsonUrl>,
    #[serde(default)]
    user_mentions: Vec<JsonMention>,
}

#[derive(Debug, Deserialize)]
struct JsonHashtag {
    text: String,
    indices: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct JsonUrl {
    url: String,
    expanded_url: Option<String>,
    display_url: Option<String>,
    indices: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct JsonMention {
    screen_name: String,
    indices: Vec<i32>,
}
