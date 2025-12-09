use twitter_text_config::Configuration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TwitterTextConfiguration {
    inner: Configuration,
}

#[wasm_bindgen]
impl TwitterTextConfiguration {
    #[wasm_bindgen(js_name = "configV1")]
    pub fn config_v1() -> TwitterTextConfiguration {
        TwitterTextConfiguration {
            inner: twitter_text_config::config_v1().clone(),
        }
    }

    #[wasm_bindgen(js_name = "configV2")]
    pub fn config_v2() -> TwitterTextConfiguration {
        TwitterTextConfiguration {
            inner: twitter_text_config::config_v2().clone(),
        }
    }

    #[wasm_bindgen(js_name = "configV3")]
    pub fn config_v3() -> TwitterTextConfiguration {
        TwitterTextConfiguration {
            inner: twitter_text_config::config_v3().clone(),
        }
    }

    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json: &str) -> Result<TwitterTextConfiguration, JsValue> {
        match serde_json::from_str::<Configuration>(json) {
            Ok(config) => Ok(TwitterTextConfiguration { inner: config }),
            Err(e) => Err(JsValue::from_str(&format!("Failed to parse config: {}", e))),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn version(&self) -> i32 {
        self.inner.version
    }

    #[wasm_bindgen(getter, js_name = "maxWeightedTweetLength")]
    pub fn max_weighted_tweet_length(&self) -> i32 {
        self.inner.max_weighted_tweet_length
    }

    #[wasm_bindgen(getter)]
    pub fn scale(&self) -> i32 {
        self.inner.scale
    }

    #[wasm_bindgen(getter, js_name = "defaultWeight")]
    pub fn default_weight(&self) -> i32 {
        self.inner.default_weight
    }

    #[wasm_bindgen(getter, js_name = "transformedUrlLength")]
    pub fn transformed_url_length(&self) -> i32 {
        self.inner.transformed_url_length
    }

    #[wasm_bindgen(getter, js_name = "emojiParsingEnabled")]
    pub fn emoji_parsing_enabled(&self) -> bool {
        self.inner.emoji_parsing_enabled
    }
}

impl TwitterTextConfiguration {
    pub fn inner(&self) -> &Configuration {
        &self.inner
    }
}
