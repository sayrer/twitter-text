use twitter_text::autolinker::Autolinker as RustAutolinker;

#[magnus::wrap(class = "Twittertext::Autolinker", free_immediately, size)]
pub struct Autolinker {
    _phantom: (),
}

impl Autolinker {
    fn new() -> Self {
        Autolinker { _phantom: () }
    }

    pub fn ruby_new() -> Self {
        Self::new()
    }

    fn to_rust_autolinker<'a>(&self) -> RustAutolinker<'a> {
        RustAutolinker::new(false)
    }

    pub fn autolink(&self, text: String) -> String {
        self.to_rust_autolinker().autolink(&text)
    }

    pub fn autolink_usernames_and_lists(&self, text: String) -> String {
        self.to_rust_autolinker()
            .autolink_usernames_and_lists(&text)
    }

    pub fn autolink_hashtags(&self, text: String) -> String {
        self.to_rust_autolinker().autolink_hashtags(&text)
    }

    pub fn autolink_urls(&self, text: String) -> String {
        self.to_rust_autolinker().autolink_urls(&text)
    }

    pub fn autolink_cashtags(&self, text: String) -> String {
        self.to_rust_autolinker().autolink_cashtags(&text)
    }
}
