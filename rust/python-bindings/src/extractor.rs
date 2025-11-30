use pyo3::prelude::*;
use twitter_text::entity::{Entity as RustEntity, Type as EntityType};
use twitter_text::extractor::{Extract, Extractor as RustExtractor};

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
}
