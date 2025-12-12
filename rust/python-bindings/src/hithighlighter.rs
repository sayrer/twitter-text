use pyo3::prelude::*;
use twitter_text::hit_highlighter::HitHighlighter as RustHitHighlighter;

#[pyclass]
pub struct Hit {
    #[pyo3(get, set)]
    pub start: usize,
    #[pyo3(get, set)]
    pub end: usize,
}

#[pymethods]
impl Hit {
    #[new]
    fn new() -> Self {
        Hit { start: 0, end: 0 }
    }
}

#[pyclass]
pub struct Hits {
    hits: Vec<(usize, usize)>,
}

#[pymethods]
impl Hits {
    #[new]
    fn new() -> Self {
        Hits { hits: Vec::new() }
    }

    fn append(&mut self, hit: PyRef<Hit>) {
        self.hits.push((hit.start, hit.end));
    }
}

#[pyclass]
pub struct HitHighlighter {
    highlight_tag: String,
}

#[pymethods]
impl HitHighlighter {
    #[new]
    fn new(tag: Option<String>) -> Self {
        HitHighlighter {
            highlight_tag: tag.unwrap_or_else(|| "em".to_string()),
        }
    }

    fn get_highlight_tag(&self) -> &str {
        &self.highlight_tag
    }

    fn set_highlight_tag(&mut self, tag: String) {
        self.highlight_tag = tag;
    }

    fn highlight(&self, text: &str, hits: PyRef<Hits>) -> String {
        let highlighter = RustHitHighlighter::new_with_tag(&self.highlight_tag);
        highlighter.highlight(text, hits.hits.clone())
    }
}
