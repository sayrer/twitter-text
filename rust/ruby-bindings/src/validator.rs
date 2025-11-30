use std::cell::Cell;
use twitter_text::validator::{Validator as RustValidator, MAX_TWEET_LENGTH};

#[magnus::wrap(class = "Twittertext::Validator", free_immediately, size)]
pub struct Validator {
    short_url_length: Cell<i32>,
    short_url_length_https: Cell<i32>,
}

impl Validator {
    fn new() -> Self {
        Validator {
            short_url_length: Cell::new(23),
            short_url_length_https: Cell::new(23),
        }
    }

    pub fn ruby_new() -> Self {
        Self::new()
    }

    pub fn get_max_tweet_length(&self) -> i32 {
        MAX_TWEET_LENGTH as i32
    }

    pub fn get_short_url_length(&self) -> i32 {
        self.short_url_length.get()
    }

    pub fn set_short_url_length(&self, length: i32) {
        self.short_url_length.set(length);
    }

    pub fn get_short_url_length_https(&self) -> i32 {
        self.short_url_length_https.get()
    }

    pub fn set_short_url_length_https(&self, length: i32) {
        self.short_url_length_https.set(length);
    }

    pub fn is_valid_tweet(&self, text: String) -> bool {
        let validator = RustValidator::new();
        validator.is_valid_tweet(&text)
    }

    pub fn is_valid_username(&self, text: String) -> bool {
        let validator = RustValidator::new();
        validator.is_valid_username(&text)
    }

    pub fn is_valid_list(&self, text: String) -> bool {
        let validator = RustValidator::new();
        validator.is_valid_list(&text)
    }

    pub fn is_valid_hashtag(&self, text: String) -> bool {
        let validator = RustValidator::new();
        validator.is_valid_hashtag(&text)
    }

    pub fn is_valid_url(&self, text: String) -> bool {
        let validator = RustValidator::new();
        validator.is_valid_url(&text)
    }

    pub fn is_valid_url_without_protocol(&self, text: String) -> bool {
        let validator = RustValidator::new();
        validator.is_valid_url_without_protocol(&text)
    }
}
