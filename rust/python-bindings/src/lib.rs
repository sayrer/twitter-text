use pyo3::prelude::*;

mod autolinker;
mod configuration;
mod extractor;
mod hithighlighter;
mod parser;
mod validator;

use autolinker::{
    AddAttributeModifier, Autolinker, Entity as AutolinkEntity, LinkTextModifier,
    ReplaceClassModifier,
};
use configuration::TwitterTextConfiguration;
use extractor::{ExtractResult, Extractor, MentionResult, ValidatingExtractor};
use hithighlighter::{Hit, HitHighlighter, Hits};
use parser::{TwitterTextParseResult, TwitterTextParser};
use validator::Validator;

/// Twitter text processing library for Python
#[pymodule]
fn twitter_text(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TwitterTextConfiguration>()?;
    m.add_class::<extractor::Entity>()?;
    m.add_class::<extractor::Range>()?;
    m.add_class::<configuration::WeightedRange>()?;
    m.add_class::<Extractor>()?;
    m.add_class::<ValidatingExtractor>()?;
    m.add_class::<ExtractResult>()?;
    m.add_class::<MentionResult>()?;
    m.add_class::<Validator>()?;
    m.add_class::<Autolinker>()?;
    m.add_class::<AutolinkEntity>()?;
    m.add_class::<AddAttributeModifier>()?;
    m.add_class::<ReplaceClassModifier>()?;
    m.add_class::<LinkTextModifier>()?;
    m.add_class::<Hit>()?;
    m.add_class::<Hits>()?;
    m.add_class::<HitHighlighter>()?;
    m.add_class::<TwitterTextParser>()?;
    m.add_class::<TwitterTextParseResult>()?;
    m.add_class::<parser::Range>()?;
    Ok(())
}
