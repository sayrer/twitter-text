use magnus::Error;
use twitter_text_config::Configuration as RustConfiguration;

#[magnus::wrap(class = "Twittertext::Configuration", free_immediately, size)]
pub struct TwitterTextConfiguration {
    inner: RustConfiguration,
}

impl TwitterTextConfiguration {
    pub fn ruby_new() -> Self {
        TwitterTextConfiguration {
            inner: RustConfiguration::default(),
        }
    }

    pub fn configuration_from_path(path: String) -> Result<Self, Error> {
        use std::path::PathBuf;
        let path_buf = PathBuf::from(path);
        let config = RustConfiguration::configuration_from_path(&path_buf);
        Ok(TwitterTextConfiguration { inner: config })
    }

    pub fn inner(&self) -> &RustConfiguration {
        &self.inner
    }
}
