use magnus::{Error, TryConvert};
use std::cell::Cell;
use twitter_text::hit_highlighter::HitHighlighter as RustHitHighlighter;

#[magnus::wrap(class = "Twittertext::Hit", free_immediately, size)]
pub struct Hit {
    start: Cell<i32>,
    end: Cell<i32>,
}

impl Hit {
    pub fn ruby_new() -> Self {
        Hit {
            start: Cell::new(0),
            end: Cell::new(0),
        }
    }

    pub fn get_start(&self) -> i32 {
        self.start.get()
    }

    pub fn set_start(&self, value: i32) {
        self.start.set(value);
    }

    pub fn get_end(&self) -> i32 {
        self.end.get()
    }

    pub fn set_end(&self, value: i32) {
        self.end.set(value);
    }
}

#[magnus::wrap(class = "Twittertext::Hits", free_immediately, size)]
pub struct Hits {
    hits: std::cell::RefCell<Vec<(usize, usize)>>,
}

impl Hits {
    pub fn ruby_new() -> Self {
        Hits {
            hits: std::cell::RefCell::new(Vec::new()),
        }
    }

    pub fn append(&self, hit: &Hit) -> Result<(), Error> {
        self.hits
            .borrow_mut()
            .push((hit.start.get() as usize, hit.end.get() as usize));
        Ok(())
    }

    pub fn to_vec(&self) -> Vec<(usize, usize)> {
        self.hits.borrow().clone()
    }
}

#[magnus::wrap(class = "Twittertext::HitHighlighter", free_immediately, size)]
pub struct HitHighlighter {
    highlight_tag: Cell<String>,
}

impl HitHighlighter {
    pub fn ruby_new(args: &[magnus::Value]) -> Result<Self, magnus::Error> {
        let tag = if args.is_empty() {
            None
        } else {
            Some(TryConvert::try_convert(args[0])?)
        };
        Ok(HitHighlighter {
            highlight_tag: Cell::new(tag.unwrap_or_else(|| "em".to_string())),
        })
    }

    pub fn get_highlight_tag(&self) -> String {
        self.highlight_tag.take()
    }

    pub fn set_highlight_tag(&self, tag: String) {
        self.highlight_tag.set(tag);
    }

    pub fn hit_highlight(&self, text: String, hits: &Hits) -> Result<String, Error> {
        let tag = self.highlight_tag.take();
        let highlighter = RustHitHighlighter::new_with_tag(&tag);
        let result = highlighter.highlight(&text, hits.to_vec());
        self.highlight_tag.set(tag);
        Ok(result)
    }
}
