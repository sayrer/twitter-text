use crate::configuration::TwitterTextConfiguration;
use magnus::Error;
use std::cell::Cell;
use twitter_text::TwitterTextParseResults;
use twitter_text_config::Range as ConfigRange;

#[magnus::wrap(class = "Twittertext::ParseRange", free_immediately, size)]
pub struct ParseRange {
    start: Cell<usize>,
    end: Cell<usize>,
}

impl ParseRange {
    pub fn new(start: usize, end: usize) -> Self {
        ParseRange {
            start: Cell::new(start),
            end: Cell::new(end),
        }
    }

    pub fn get_start(&self) -> usize {
        self.start.get()
    }

    pub fn set_start(&self, start: usize) {
        self.start.set(start);
    }

    pub fn get_end(&self) -> usize {
        self.end.get()
    }

    pub fn set_end(&self, end: usize) {
        self.end.set(end);
    }
}

impl From<ConfigRange> for ParseRange {
    fn from(r: ConfigRange) -> Self {
        ParseRange::new(r.start() as usize, r.end() as usize)
    }
}

#[magnus::wrap(class = "Twittertext::TwitterTextParseResult", free_immediately, size)]
pub struct TwitterTextParseResult {
    weighted_length: Cell<i32>,
    permillage: Cell<i32>,
    is_valid: Cell<bool>,
    display_text_range: ParseRange,
    valid_text_range: ParseRange,
}

impl TwitterTextParseResult {
    pub fn get_weighted_length(&self) -> i32 {
        self.weighted_length.get()
    }

    pub fn get_permillage(&self) -> i32 {
        self.permillage.get()
    }

    pub fn get_is_valid(&self) -> bool {
        self.is_valid.get()
    }

    pub fn get_display_text_range(&self) -> ParseRange {
        ParseRange::new(
            self.display_text_range.get_start(),
            self.display_text_range.get_end(),
        )
    }

    pub fn get_valid_text_range(&self) -> ParseRange {
        ParseRange::new(
            self.valid_text_range.get_start(),
            self.valid_text_range.get_end(),
        )
    }
}

impl From<TwitterTextParseResults> for TwitterTextParseResult {
    fn from(r: TwitterTextParseResults) -> Self {
        TwitterTextParseResult {
            weighted_length: Cell::new(r.weighted_length),
            permillage: Cell::new(r.permillage),
            is_valid: Cell::new(r.is_valid),
            display_text_range: r.display_text_range.into(),
            valid_text_range: r.valid_text_range.into(),
        }
    }
}

#[magnus::wrap(class = "Twittertext::TwitterTextParser", free_immediately, size)]
pub struct TwitterTextParser;

impl TwitterTextParser {
    pub fn parse(
        text: String,
        config: &TwitterTextConfiguration,
        extract_urls: bool,
    ) -> Result<TwitterTextParseResult, Error> {
        let results = twitter_text::parse(&text, &config.inner().borrow(), extract_urls);
        Ok(results.into())
    }
}
