use pyo3::prelude::*;
use twitter_text::validator::{Validator as RustValidator, MAX_TWEET_LENGTH};

#[pyclass]
pub struct Validator {
    inner: RustValidator,
}

#[pymethods]
impl Validator {
    #[new]
    fn new() -> Self {
        Validator {
            inner: RustValidator::new(),
        }
    }

    fn is_valid_tweet(&self, text: &str) -> bool {
        self.inner.is_valid_tweet(text)
    }

    fn is_valid_username(&self, text: &str) -> bool {
        self.inner.is_valid_username(text)
    }

    fn is_valid_list(&self, text: &str) -> bool {
        self.inner.is_valid_list(text)
    }

    fn is_valid_hashtag(&self, text: &str) -> bool {
        self.inner.is_valid_hashtag(text)
    }

    fn is_valid_url(&self, text: &str) -> bool {
        self.inner.is_valid_url(text)
    }

    fn is_valid_url_without_protocol(&self, text: &str) -> bool {
        self.inner.is_valid_url_without_protocol(text)
    }

    fn get_max_tweet_length(&self) -> i32 {
        MAX_TWEET_LENGTH
    }

    fn get_short_url_length(&self) -> i32 {
        self.inner.get_short_url_length()
    }

    fn set_short_url_length(&mut self, length: i32) {
        self.inner.set_short_url_length(length);
    }

    fn get_short_url_length_https(&self) -> i32 {
        self.inner.get_short_url_length_https()
    }

    fn set_short_url_length_https(&mut self, length: i32) {
        self.inner.set_short_url_length_https(length);
    }
}
