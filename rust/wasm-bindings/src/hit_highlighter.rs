use js_sys::Array;
use twitter_text::hit_highlighter::HitHighlighter;
use wasm_bindgen::prelude::*;

/// Wrapper for HitHighlighter that provides tweet text highlighting
#[wasm_bindgen(js_name = HitHighlighter)]
pub struct JsHitHighlighter {
    highlighter: HitHighlighter,
}

#[wasm_bindgen(js_class = HitHighlighter)]
impl JsHitHighlighter {
    /// Create a new HitHighlighter with default tags (<em> and </em>)
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            highlighter: HitHighlighter::new(),
        }
    }

    /// Create a new HitHighlighter with a custom highlight tag (e.g., "em", "strong", "mark")
    #[wasm_bindgen(js_name = withTag)]
    pub fn with_tag(tag: &str) -> Self {
        Self {
            highlighter: HitHighlighter::new_with_tag(tag),
        }
    }

    /// Highlight hits in the given text
    ///
    /// hits should be an array of [start, end] pairs representing character ranges to highlight
    #[wasm_bindgen]
    pub fn highlight(&self, text: &str, hits: &Array) -> String {
        let hit_ranges: Vec<(usize, usize)> = hits
            .iter()
            .filter_map(|val| {
                let arr = Array::from(&val);
                if arr.length() >= 2 {
                    let start = arr.get(0).as_f64()? as usize;
                    let end = arr.get(1).as_f64()? as usize;
                    Some((start, end))
                } else {
                    None
                }
            })
            .collect();

        self.highlighter.highlight(text, hit_ranges)
    }
}

impl Default for JsHitHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
