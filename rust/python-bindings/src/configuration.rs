use pyo3::prelude::*;
use std::path::PathBuf;
use twitter_text_config::{Configuration, Range, WeightedRange as RustWeightedRange};

#[pyclass]
#[derive(Clone)]
pub struct TwitterTextConfiguration {
    inner: Configuration,
}

#[pyclass]
#[derive(Clone)]
pub struct WeightedRange {
    #[pyo3(get, set)]
    pub range: PyRange,
    #[pyo3(get, set)]
    pub weight: i32,
}

#[pyclass]
#[derive(Clone)]
pub struct PyRange {
    #[pyo3(get, set)]
    pub start: i32,
    #[pyo3(get, set)]
    pub end: i32,
}

impl From<&Range> for PyRange {
    fn from(r: &Range) -> Self {
        PyRange {
            start: r.start(),
            end: r.end(),
        }
    }
}

impl From<&RustWeightedRange> for WeightedRange {
    fn from(wr: &RustWeightedRange) -> Self {
        WeightedRange {
            range: PyRange::from(&wr.range),
            weight: wr.weight,
        }
    }
}

#[pymethods]
impl TwitterTextConfiguration {
    #[new]
    fn new() -> Self {
        TwitterTextConfiguration {
            inner: Configuration::default(),
        }
    }

    #[staticmethod]
    fn configuration_from_path(path: &str) -> PyResult<Self> {
        let pathbuf = PathBuf::from(path);
        Ok(TwitterTextConfiguration {
            inner: Configuration::configuration_from_path(&pathbuf),
        })
    }

    #[staticmethod]
    fn configuration_from_json(json: &str) -> PyResult<Self> {
        Ok(TwitterTextConfiguration {
            inner: Configuration::configuration_from_json(json),
        })
    }

    #[staticmethod]
    fn config_v1() -> Self {
        TwitterTextConfiguration {
            inner: twitter_text_config::config_v1().clone(),
        }
    }

    #[staticmethod]
    fn config_v2() -> Self {
        TwitterTextConfiguration {
            inner: twitter_text_config::config_v2().clone(),
        }
    }

    #[staticmethod]
    fn config_v3() -> Self {
        TwitterTextConfiguration {
            inner: twitter_text_config::config_v3().clone(),
        }
    }

    fn get_version(&self) -> i32 {
        self.inner.version
    }

    fn set_version(&mut self, version: i32) {
        self.inner.version = version;
    }

    fn get_max_weighted_tweet_length(&self) -> i32 {
        self.inner.max_weighted_tweet_length
    }

    fn set_max_weighted_tweet_length(&mut self, length: i32) {
        self.inner.max_weighted_tweet_length = length;
    }

    fn get_scale(&self) -> i32 {
        self.inner.scale
    }

    fn set_scale(&mut self, scale: i32) {
        self.inner.scale = scale;
    }

    fn get_default_weight(&self) -> i32 {
        self.inner.default_weight
    }

    fn set_default_weight(&mut self, weight: i32) {
        self.inner.default_weight = weight;
    }

    fn get_transformed_url_length(&self) -> i32 {
        self.inner.transformed_url_length
    }

    fn set_transformed_url_length(&mut self, length: i32) {
        self.inner.transformed_url_length = length;
    }

    fn get_emoji_parsing_enabled(&self) -> bool {
        self.inner.emoji_parsing_enabled
    }

    fn set_emoji_parsing_enabled(&mut self, enabled: bool) {
        self.inner.emoji_parsing_enabled = enabled;
    }

    fn get_ranges(&self) -> Vec<WeightedRange> {
        self.inner.ranges.iter().map(WeightedRange::from).collect()
    }
}

impl TwitterTextConfiguration {
    pub fn inner(&self) -> &Configuration {
        &self.inner
    }
}
