use crate::configuration::TwitterTextConfiguration;
use crate::parser::TwitterTextParseResult;
use pyo3::prelude::*;
use twitter_text::entity::{Entity as RustEntity, Type as EntityType};
use twitter_text::extractor::{
    Extract, ExtractResult as RustExtractResult, Extractor as RustExtractor,
    MentionResult as RustMentionResult, ValidatingExtractor as RustValidatingExtractor,
};

#[pyclass]
#[derive(Clone)]
pub struct Range {
    #[pyo3(get, set)]
    pub start: i32,
    #[pyo3(get, set)]
    pub end: i32,
}

#[pyclass]
#[derive(Clone)]
pub struct Entity {
    #[pyo3(get, set)]
    pub entity_type: i32,
    #[pyo3(get, set)]
    pub start: i32,
    #[pyo3(get, set)]
    pub end: i32,
    #[pyo3(get, set)]
    pub value: String,
    #[pyo3(get, set)]
    pub list_slug: String,
    #[pyo3(get, set)]
    pub display_url: String,
    #[pyo3(get, set)]
    pub expanded_url: String,
}

impl<'a> From<&RustEntity<'a>> for Entity {
    fn from(entity: &RustEntity<'a>) -> Self {
        Entity {
            entity_type: entity.t as i32,
            start: entity.start,
            end: entity.end,
            value: entity.value.to_string(),
            list_slug: entity.list_slug.to_string(),
            display_url: entity.display_url.to_string(),
            expanded_url: entity.expanded_url.to_string(),
        }
    }
}

#[pyclass]
pub struct Extractor {
    inner: RustExtractor,
}

#[pymethods]
impl Extractor {
    #[new]
    fn new() -> Self {
        Extractor {
            inner: RustExtractor::new(),
        }
    }

    fn get_extract_url_without_protocol(&self) -> bool {
        self.inner.get_extract_url_without_protocol()
    }

    fn set_extract_url_without_protocol(&mut self, extract: bool) {
        self.inner.set_extract_url_without_protocol(extract);
    }

    fn extract_entities_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_entities_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_mentioned_screennames(&self, text: &str) -> Vec<String> {
        self.inner.extract_mentioned_screennames(text)
    }

    fn extract_mentioned_screennames_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_mentioned_screennames_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_mentions_or_lists_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_mentions_or_lists_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_reply_screenname(&self, text: &str) -> Option<Entity> {
        self.inner
            .extract_reply_username(text)
            .map(|e| Entity::from(&e))
    }

    fn extract_urls(&self, text: &str) -> Vec<String> {
        self.inner.extract_urls(text)
    }

    fn extract_urls_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_urls_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_hashtags(&self, text: &str) -> Vec<String> {
        self.inner.extract_hashtags(text)
    }

    fn extract_hashtags_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_hashtags_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_cashtags(&self, text: &str) -> Vec<String> {
        self.inner.extract_cashtags(text)
    }

    fn extract_cashtags_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_cashtags_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_federated_mentions(&self, text: &str) -> Vec<String> {
        self.inner.extract_federated_mentions(text)
    }

    fn extract_federated_mentions_with_indices(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_federated_mentions_with_indices(text)
            .iter()
            .map(Entity::from)
            .collect()
    }

    fn extract_entities_with_indices_federated(&self, text: &str) -> Vec<Entity> {
        self.inner
            .extract_entities_with_indices_federated(text)
            .iter()
            .map(Entity::from)
            .collect()
    }
}

#[pyclass]
pub struct ExtractResult {
    #[pyo3(get)]
    pub parse_results: TwitterTextParseResult,
    #[pyo3(get)]
    pub entities: Vec<Entity>,
}

impl<'a> From<RustExtractResult<'a>> for ExtractResult {
    fn from(result: RustExtractResult<'a>) -> Self {
        ExtractResult {
            parse_results: result.parse_results.into(),
            entities: result.entities.iter().map(Entity::from).collect(),
        }
    }
}

#[pyclass]
pub struct MentionResult {
    #[pyo3(get)]
    pub parse_results: TwitterTextParseResult,
    #[pyo3(get)]
    pub entity: Option<Entity>,
}

impl<'a> From<RustMentionResult<'a>> for MentionResult {
    fn from(result: RustMentionResult<'a>) -> Self {
        MentionResult {
            parse_results: result.parse_results.into(),
            entity: result.mention.map(|e| Entity::from(&e)),
        }
    }
}

#[pyclass]
pub struct ValidatingExtractor {
    config: Py<TwitterTextConfiguration>,
    extract_url_without_protocol: std::cell::Cell<bool>,
    normalize: std::cell::Cell<bool>,
}

#[pymethods]
impl ValidatingExtractor {
    #[new]
    fn new(_py: Python, config: Py<TwitterTextConfiguration>) -> Self {
        ValidatingExtractor {
            config,
            extract_url_without_protocol: std::cell::Cell::new(true),
            normalize: std::cell::Cell::new(true),
        }
    }

    fn get_extract_url_without_protocol(&self) -> bool {
        self.extract_url_without_protocol.get()
    }

    fn set_extract_url_without_protocol(&self, extract: bool) {
        self.extract_url_without_protocol.set(extract);
    }

    fn get_normalize(&self) -> bool {
        self.normalize.get()
    }

    fn set_normalize(&self, normalize: bool) {
        self.normalize.set(normalize);
    }

    fn extract_mentioned_screennames_with_indices(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_mentioned_screennames_with_indices(&input);
        result.into()
    }

    fn extract_mentions_or_lists_with_indices(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_mentions_or_lists_with_indices(&input);
        result.into()
    }

    fn extract_reply_screenname(&self, py: Python, text: &str) -> MentionResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_reply_username(&input);
        result.into()
    }

    fn extract_urls_with_indices(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_urls_with_indices(&input);
        result.into()
    }

    fn extract_hashtags_with_indices(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_hashtags_with_indices(&input);
        result.into()
    }

    fn extract_cashtags_with_indices(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_cashtags_with_indices(&input);
        result.into()
    }

    fn extract_federated_mentions(&self, py: Python, text: &str) -> Vec<String> {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        extractor.extract_federated_mentions(&input)
    }

    fn extract_federated_mentions_with_indices(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_federated_mentions_with_indices(&input);
        result.into()
    }

    fn extract_entities_with_indices_federated(&self, py: Python, text: &str) -> ExtractResult {
        let config = self.config.borrow(py);
        let mut extractor = RustValidatingExtractor::new(config.inner());
        extractor.set_extract_url_without_protocol(self.extract_url_without_protocol.get());
        let input = extractor.prep_input(text);
        let result = extractor.extract_entities_with_indices_federated(&input);
        result.into()
    }
}
