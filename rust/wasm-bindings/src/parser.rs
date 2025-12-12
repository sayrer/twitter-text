use crate::configuration::TwitterTextConfiguration;
use twitter_text::TwitterTextParseResults;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Range {
    start: i32,
    end: i32,
}

#[wasm_bindgen]
impl Range {
    #[wasm_bindgen(getter)]
    pub fn start(&self) -> i32 {
        self.start
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> i32 {
        self.end
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct TwitterTextParseResult {
    weighted_length: i32,
    permillage: i32,
    is_valid: bool,
    display_text_range_start: i32,
    display_text_range_end: i32,
    valid_text_range_start: i32,
    valid_text_range_end: i32,
}

#[wasm_bindgen]
impl TwitterTextParseResult {
    #[wasm_bindgen(getter, js_name = "weightedLength")]
    pub fn weighted_length(&self) -> i32 {
        self.weighted_length
    }

    #[wasm_bindgen(getter)]
    pub fn permillage(&self) -> i32 {
        self.permillage
    }

    #[wasm_bindgen(getter, js_name = "isValid")]
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    #[wasm_bindgen(getter, js_name = "displayTextRange")]
    pub fn display_text_range(&self) -> Range {
        Range {
            start: self.display_text_range_start,
            end: self.display_text_range_end,
        }
    }

    #[wasm_bindgen(getter, js_name = "validTextRange")]
    pub fn valid_text_range(&self) -> Range {
        Range {
            start: self.valid_text_range_start,
            end: self.valid_text_range_end,
        }
    }
}

impl From<TwitterTextParseResults> for TwitterTextParseResult {
    fn from(results: TwitterTextParseResults) -> Self {
        TwitterTextParseResult {
            weighted_length: results.weighted_length,
            permillage: results.permillage,
            is_valid: results.is_valid,
            display_text_range_start: results.display_text_range.start(),
            display_text_range_end: results.display_text_range.end(),
            valid_text_range_start: results.valid_text_range.start(),
            valid_text_range_end: results.valid_text_range.end(),
        }
    }
}

#[wasm_bindgen]
pub struct TwitterTextParser {
    config: twitter_text_config::Configuration,
}

#[wasm_bindgen]
impl TwitterTextParser {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &TwitterTextConfiguration) -> TwitterTextParser {
        TwitterTextParser {
            config: config.inner().clone(),
        }
    }

    #[wasm_bindgen(js_name = "parseTweet")]
    pub fn parse_tweet(&self, text: &str) -> TwitterTextParseResult {
        let results = twitter_text::parse(text, &self.config, true);
        results.into()
    }
}

/// Parse a tweet with the default v3 configuration
#[wasm_bindgen(js_name = "parseTweet")]
pub fn parse_tweet(text: &str) -> TwitterTextParseResult {
    let config = twitter_text_config::config_v3();
    let results = twitter_text::parse(text, config, true);
    results.into()
}

/// Parse a tweet with a specific configuration
#[wasm_bindgen(js_name = "parseTweetWithConfig")]
pub fn parse_tweet_with_config(
    text: &str,
    config: &TwitterTextConfiguration,
) -> TwitterTextParseResult {
    let results = twitter_text::parse(text, config.inner(), true);
    results.into()
}
