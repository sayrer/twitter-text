use pyo3::prelude::*;

mod autolinker;
mod configuration;
mod extractor;
mod hithighlighter;
mod validator;

use autolinker::Autolinker;
use configuration::TwitterTextConfiguration;
use extractor::Extractor;
use hithighlighter::HitHighlighter;
use validator::Validator;

/// Twitter text processing library for Python
#[pymodule]
fn twitter_text(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TwitterTextConfiguration>()?;
    m.add_class::<extractor::Entity>()?;
    m.add_class::<extractor::Range>()?;
    m.add_class::<configuration::WeightedRange>()?;
    m.add_class::<Extractor>()?;
    m.add_class::<Validator>()?;
    m.add_class::<Autolinker>()?;
    m.add_class::<HitHighlighter>()?;
    Ok(())
}
