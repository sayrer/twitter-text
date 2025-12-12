use magnus::{Error, RArray};
use std::cell::Cell;
use twitter_text_config::{
    Configuration as RustConfiguration, Range, WeightedRange as RustWeightedRange,
};

#[magnus::wrap(
    class = "Twittertext::TwitterTextConfiguration",
    free_immediately,
    size
)]
pub struct TwitterTextConfiguration {
    inner: std::cell::RefCell<RustConfiguration>,
}

#[magnus::wrap(class = "Twittertext::WeightedRange", free_immediately, size)]
pub struct WeightedRange {
    start: Cell<i32>,
    end: Cell<i32>,
    weight: Cell<i32>,
}

impl WeightedRange {
    pub fn new(start: i32, end: i32, weight: i32) -> Self {
        WeightedRange {
            start: Cell::new(start),
            end: Cell::new(end),
            weight: Cell::new(weight),
        }
    }

    pub fn get_range(&self) -> Result<RubyRange, Error> {
        Ok(RubyRange::new(self.start.get(), self.end.get()))
    }

    pub fn get_weight(&self) -> i32 {
        self.weight.get()
    }

    pub fn set_weight(&self, weight: i32) {
        self.weight.set(weight);
    }
}

#[magnus::wrap(class = "Twittertext::Range", free_immediately, size)]
pub struct RubyRange {
    start: Cell<i32>,
    end: Cell<i32>,
}

impl RubyRange {
    pub fn new(start: i32, end: i32) -> Self {
        RubyRange {
            start: Cell::new(start),
            end: Cell::new(end),
        }
    }

    pub fn get_start(&self) -> i32 {
        self.start.get()
    }

    pub fn set_start(&self, start: i32) {
        self.start.set(start);
    }

    pub fn get_end(&self) -> i32 {
        self.end.get()
    }

    pub fn set_end(&self, end: i32) {
        self.end.set(end);
    }
}

impl From<&Range> for RubyRange {
    fn from(r: &Range) -> Self {
        RubyRange::new(r.start(), r.end())
    }
}

impl From<&RustWeightedRange> for WeightedRange {
    fn from(wr: &RustWeightedRange) -> Self {
        WeightedRange {
            start: Cell::new(wr.range.start()),
            end: Cell::new(wr.range.end()),
            weight: Cell::new(wr.weight),
        }
    }
}

impl TwitterTextConfiguration {
    pub fn ruby_new() -> Self {
        TwitterTextConfiguration {
            inner: std::cell::RefCell::new(RustConfiguration::default()),
        }
    }

    pub fn configuration_from_path(path: String) -> Result<Self, Error> {
        use std::path::PathBuf;
        let path_buf = PathBuf::from(path);
        let config = RustConfiguration::configuration_from_path(&path_buf);
        Ok(TwitterTextConfiguration {
            inner: std::cell::RefCell::new(config),
        })
    }

    pub fn configuration_from_json(json: String) -> Result<Self, Error> {
        let config = RustConfiguration::configuration_from_json(&json);
        Ok(TwitterTextConfiguration {
            inner: std::cell::RefCell::new(config),
        })
    }

    pub fn config_v1() -> Self {
        TwitterTextConfiguration {
            inner: std::cell::RefCell::new(twitter_text_config::config_v1().clone()),
        }
    }

    pub fn config_v2() -> Self {
        TwitterTextConfiguration {
            inner: std::cell::RefCell::new(twitter_text_config::config_v2().clone()),
        }
    }

    pub fn config_v3() -> Self {
        TwitterTextConfiguration {
            inner: std::cell::RefCell::new(twitter_text_config::config_v3().clone()),
        }
    }

    pub fn get_version(&self) -> i32 {
        self.inner.borrow().version
    }

    pub fn set_version(&self, version: i32) {
        self.inner.borrow_mut().version = version;
    }

    pub fn get_max_weighted_tweet_length(&self) -> i32 {
        self.inner.borrow().max_weighted_tweet_length
    }

    pub fn set_max_weighted_tweet_length(&self, length: i32) {
        self.inner.borrow_mut().max_weighted_tweet_length = length;
    }

    pub fn get_scale(&self) -> i32 {
        self.inner.borrow().scale
    }

    pub fn set_scale(&self, scale: i32) {
        self.inner.borrow_mut().scale = scale;
    }

    pub fn get_default_weight(&self) -> i32 {
        self.inner.borrow().default_weight
    }

    pub fn set_default_weight(&self, weight: i32) {
        self.inner.borrow_mut().default_weight = weight;
    }

    pub fn get_transformed_url_length(&self) -> i32 {
        self.inner.borrow().transformed_url_length
    }

    pub fn set_transformed_url_length(&self, length: i32) {
        self.inner.borrow_mut().transformed_url_length = length;
    }

    pub fn get_emoji_parsing_enabled(&self) -> bool {
        self.inner.borrow().emoji_parsing_enabled
    }

    pub fn set_emoji_parsing_enabled(&self, enabled: bool) {
        self.inner.borrow_mut().emoji_parsing_enabled = enabled;
    }

    pub fn get_ranges(&self) -> Result<RArray, Error> {
        let array = RArray::new();
        for wr in self.inner.borrow().ranges.iter() {
            let weighted_range = WeightedRange::from(wr);
            array.push(weighted_range)?;
        }
        Ok(array)
    }

    pub fn inner(&self) -> &std::cell::RefCell<RustConfiguration> {
        &self.inner
    }
}
