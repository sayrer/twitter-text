use twitter_text::validator::{Validator as RustValidator, MAX_TWEET_LENGTH};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Validator {
    inner: RustValidator,
}

#[wasm_bindgen]
impl Validator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Validator {
        Validator {
            inner: RustValidator::new(),
        }
    }

    #[wasm_bindgen(js_name = "isValidTweet")]
    pub fn is_valid_tweet(&self, text: &str) -> bool {
        self.inner.is_valid_tweet(text)
    }

    #[wasm_bindgen(js_name = "isValidUsername")]
    pub fn is_valid_username(&self, text: &str) -> bool {
        self.inner.is_valid_username(text)
    }

    #[wasm_bindgen(js_name = "isValidList")]
    pub fn is_valid_list(&self, text: &str) -> bool {
        self.inner.is_valid_list(text)
    }

    #[wasm_bindgen(js_name = "isValidHashtag")]
    pub fn is_valid_hashtag(&self, text: &str) -> bool {
        self.inner.is_valid_hashtag(text)
    }

    #[wasm_bindgen(js_name = "isValidUrl")]
    pub fn is_valid_url(&self, text: &str) -> bool {
        self.inner.is_valid_url(text)
    }

    #[wasm_bindgen(js_name = "isValidUrlWithoutProtocol")]
    pub fn is_valid_url_without_protocol(&self, text: &str) -> bool {
        self.inner.is_valid_url_without_protocol(text)
    }

    #[wasm_bindgen(js_name = "getMaxTweetLength")]
    pub fn get_max_tweet_length(&self) -> i32 {
        MAX_TWEET_LENGTH
    }

    #[wasm_bindgen(js_name = "getShortUrlLength")]
    pub fn get_short_url_length(&self) -> i32 {
        self.inner.get_short_url_length()
    }

    #[wasm_bindgen(js_name = "setShortUrlLength")]
    pub fn set_short_url_length(&mut self, length: i32) {
        self.inner.set_short_url_length(length);
    }

    #[wasm_bindgen(js_name = "getShortUrlLengthHttps")]
    pub fn get_short_url_length_https(&self) -> i32 {
        self.inner.get_short_url_length_https()
    }

    #[wasm_bindgen(js_name = "setShortUrlLengthHttps")]
    pub fn set_short_url_length_https(&mut self, length: i32) {
        self.inner.set_short_url_length_https(length);
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}
