use pyo3::prelude::*;
use twitter_text::hit_highlighter::HitHighlighter as RustHitHighlighter;

#[pyclass]
pub struct HitHighlighter {
    highlight_tag: String,
}

#[pymethods]
impl HitHighlighter {
    #[new]
    fn new() -> Self {
        HitHighlighter {
            highlight_tag: "em".to_string(),
        }
    }

    fn get_highlight_tag(&self) -> &str {
        &self.highlight_tag
    }

    fn set_highlight_tag(&mut self, tag: String) {
        self.highlight_tag = tag;
    }

    fn highlight(&self, text: &str, hits: Vec<(usize, usize)>) -> String {
        let highlighter = RustHitHighlighter::new_with_tag(&self.highlight_tag);
        highlighter.highlight(text, hits)
    }
}
