use crate::configuration::TwitterTextConfiguration;
use pyo3::prelude::*;
use twitter_text::TwitterTextParseResults;
use twitter_text_config::Range as ConfigRange;

#[pyclass(get_all, set_all)]
#[derive(Clone)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[pymethods]
impl Range {
    #[new]
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }
}

impl From<ConfigRange> for Range {
    fn from(r: ConfigRange) -> Self {
        Range {
            start: r.start() as usize,
            end: r.end() as usize,
        }
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct TwitterTextParseResult {
    pub weighted_length: i32,
    pub permillage: i32,
    pub is_valid: bool,
    pub display_text_range: Range,
    pub valid_text_range: Range,
}

impl From<TwitterTextParseResults> for TwitterTextParseResult {
    fn from(r: TwitterTextParseResults) -> Self {
        TwitterTextParseResult {
            weighted_length: r.weighted_length,
            permillage: r.permillage,
            is_valid: r.is_valid,
            display_text_range: r.display_text_range.into(),
            valid_text_range: r.valid_text_range.into(),
        }
    }
}

#[pyclass]
pub struct TwitterTextParser;

#[pymethods]
impl TwitterTextParser {
    #[staticmethod]
    fn parse(
        text: &str,
        config: &TwitterTextConfiguration,
        extract_urls: bool,
    ) -> TwitterTextParseResult {
        let results = twitter_text::parse(text, config.inner(), extract_urls);
        results.into()
    }
}
