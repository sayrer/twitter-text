use crate::configuration::TwitterTextConfiguration;
use magnus::{Error, RArray};
use std::cell::RefCell;
use twitter_text::entity::Entity as RustEntity;
use twitter_text::extractor::{
    Extract, Extractor as RustExtractor, ValidatingExtractor as RustValidatingExtractor,
};
use twitter_text_config::Configuration as RustConfiguration;

#[magnus::wrap(class = "Twittertext::Entity", free_immediately, size)]
#[derive(Clone)]
pub struct Entity {
    pub entity_type: i32,
    pub start: i32,
    pub end: i32,
    pub value: String,
    pub list_slug: String,
    pub display_url: String,
    pub expanded_url: String,
}

impl Entity {
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }

    pub fn start(&self) -> i32 {
        self.start
    }

    pub fn end(&self) -> i32 {
        self.end
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn list_slug(&self) -> String {
        self.list_slug.clone()
    }

    pub fn display_url(&self) -> String {
        self.display_url.clone()
    }

    pub fn expanded_url(&self) -> String {
        self.expanded_url.clone()
    }
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

#[magnus::wrap(class = "Twittertext::ExtractResult", free_immediately, size)]
pub struct ExtractResult {
    entities: RefCell<Vec<Entity>>,
}

impl ExtractResult {
    pub fn new(entities: Vec<Entity>) -> Self {
        ExtractResult {
            entities: RefCell::new(entities),
        }
    }

    pub fn get_entities(&self) -> Result<RArray, Error> {
        let array = RArray::new();
        for entity in self.entities.borrow().iter() {
            array.push(entity.clone())?;
        }
        Ok(array)
    }
}

#[magnus::wrap(class = "Twittertext::MentionResult", free_immediately, size)]
pub struct MentionResult {
    entity: RefCell<Option<Entity>>,
}

impl MentionResult {
    pub fn new(entity: Option<Entity>) -> Self {
        MentionResult {
            entity: RefCell::new(entity),
        }
    }

    pub fn get_entity(&self) -> Option<Entity> {
        self.entity.borrow().clone()
    }
}

#[magnus::wrap(class = "Twittertext::Extractor", free_immediately, size)]
pub struct Extractor {
    inner: RefCell<RustExtractor>,
}

impl Extractor {
    fn new() -> Self {
        Extractor {
            inner: RefCell::new(RustExtractor::new()),
        }
    }

    pub fn ruby_new() -> Self {
        Self::new()
    }

    pub fn get_extract_url_without_protocol(&self) -> bool {
        self.inner.borrow().get_extract_url_without_protocol()
    }

    pub fn set_extract_url_without_protocol(&self, extract: bool) {
        self.inner
            .borrow_mut()
            .set_extract_url_without_protocol(extract);
    }

    pub fn extract_mentioned_screennames(&self, text: String) -> Result<RArray, Error> {
        let mentions = self.inner.borrow().extract_mentioned_screennames(&text);
        let array = RArray::new();
        for mention in mentions {
            array.push(mention)?;
        }
        Ok(array)
    }

    pub fn extract_mentioned_screennames_with_indices(
        &self,
        text: String,
    ) -> Result<RArray, Error> {
        let entities = self
            .inner
            .borrow()
            .extract_mentioned_screennames_with_indices(&text);
        let array = RArray::new();
        for entity in entities.iter() {
            array.push(Entity::from(entity))?;
        }
        Ok(array)
    }

    pub fn extract_mentions_or_lists_with_indices(&self, text: String) -> Result<RArray, Error> {
        let entities = self
            .inner
            .borrow()
            .extract_mentions_or_lists_with_indices(&text);
        let array = RArray::new();
        for entity in entities.iter() {
            array.push(Entity::from(entity))?;
        }
        Ok(array)
    }

    pub fn extract_reply_screenname(&self, text: String) -> Result<Option<Entity>, Error> {
        Ok(self
            .inner
            .borrow()
            .extract_reply_username(&text)
            .map(|e| Entity::from(&e)))
    }

    pub fn extract_urls(&self, text: String) -> Result<RArray, Error> {
        let urls = self.inner.borrow().extract_urls(&text);
        let array = RArray::new();
        for url in urls {
            array.push(url)?;
        }
        Ok(array)
    }

    pub fn extract_urls_with_indices(&self, text: String) -> Result<RArray, Error> {
        let entities = self.inner.borrow().extract_urls_with_indices(&text);
        let array = RArray::new();
        for entity in entities.iter() {
            array.push(Entity::from(entity))?;
        }
        Ok(array)
    }

    pub fn extract_hashtags(&self, text: String) -> Result<RArray, Error> {
        let hashtags = self.inner.borrow().extract_hashtags(&text);
        let array = RArray::new();
        for hashtag in hashtags {
            array.push(hashtag)?;
        }
        Ok(array)
    }

    pub fn extract_hashtags_with_indices(&self, text: String) -> Result<RArray, Error> {
        let entities = self.inner.borrow().extract_hashtags_with_indices(&text);
        let array = RArray::new();
        for entity in entities.iter() {
            array.push(Entity::from(entity))?;
        }
        Ok(array)
    }

    pub fn extract_cashtags(&self, text: String) -> Result<RArray, Error> {
        let cashtags = self.inner.borrow().extract_cashtags(&text);
        let array = RArray::new();
        for cashtag in cashtags {
            array.push(cashtag)?;
        }
        Ok(array)
    }

    pub fn extract_cashtags_with_indices(&self, text: String) -> Result<RArray, Error> {
        let entities = self.inner.borrow().extract_cashtags_with_indices(&text);
        let array = RArray::new();
        for entity in entities.iter() {
            array.push(Entity::from(entity))?;
        }
        Ok(array)
    }

    pub fn extract_entities_with_indices(&self, text: String) -> Result<RArray, Error> {
        let entities = self.inner.borrow().extract_entities_with_indices(&text);
        let array = RArray::new();
        for entity in entities.iter() {
            array.push(Entity::from(entity))?;
        }
        Ok(array)
    }
}

#[magnus::wrap(class = "Twittertext::ValidatingExtractor", free_immediately, size)]
pub struct ValidatingExtractor {
    config: RustConfiguration,
    extract_url_without_protocol: std::cell::Cell<bool>,
    normalize: std::cell::Cell<bool>,
}

impl ValidatingExtractor {
    pub fn ruby_new(config: &TwitterTextConfiguration) -> Self {
        ValidatingExtractor {
            config: config.inner().borrow().clone(),
            extract_url_without_protocol: std::cell::Cell::new(true),
            normalize: std::cell::Cell::new(true),
        }
    }

    pub fn get_extract_url_without_protocol(&self) -> bool {
        self.extract_url_without_protocol.get()
    }

    pub fn set_extract_url_without_protocol(&self, extract: bool) {
        self.extract_url_without_protocol.set(extract);
    }

    pub fn get_normalize(&self) -> bool {
        self.normalize.get()
    }

    pub fn set_normalize(&self, normalize: bool) {
        self.normalize.set(normalize);
    }

    pub fn extract_mentioned_screennames_with_indices(
        &self,
        text: String,
    ) -> Result<ExtractResult, Error> {
        let mut extractor = RustValidatingExtractor::new(&self.config);
        let input = extractor.prep_input(&text);
        let result = extractor.extract_mentioned_screennames_with_indices(&input);
        let entities: Vec<Entity> = result.entities.iter().map(Entity::from).collect();
        Ok(ExtractResult::new(entities))
    }

    pub fn extract_mentions_or_lists_with_indices(
        &self,
        text: String,
    ) -> Result<ExtractResult, Error> {
        let mut extractor = RustValidatingExtractor::new(&self.config);
        let input = extractor.prep_input(&text);
        let result = extractor.extract_mentions_or_lists_with_indices(&input);
        let entities: Vec<Entity> = result.entities.iter().map(Entity::from).collect();
        Ok(ExtractResult::new(entities))
    }

    pub fn extract_reply_screenname(&self, text: String) -> Result<MentionResult, Error> {
        let mut extractor = RustValidatingExtractor::new(&self.config);
        let input = extractor.prep_input(&text);
        let result = extractor.extract_reply_username(&input);
        Ok(MentionResult::new(result.mention.map(|e| Entity::from(&e))))
    }

    pub fn extract_urls_with_indices(&self, text: String) -> Result<ExtractResult, Error> {
        let mut extractor = RustValidatingExtractor::new(&self.config);
        let input = extractor.prep_input(&text);
        let result = extractor.extract_urls_with_indices(&input);
        let entities: Vec<Entity> = result.entities.iter().map(Entity::from).collect();
        Ok(ExtractResult::new(entities))
    }

    pub fn extract_hashtags_with_indices(&self, text: String) -> Result<ExtractResult, Error> {
        let mut extractor = RustValidatingExtractor::new(&self.config);
        let input = extractor.prep_input(&text);
        let result = extractor.extract_hashtags_with_indices(&input);
        let entities: Vec<Entity> = result.entities.iter().map(Entity::from).collect();
        Ok(ExtractResult::new(entities))
    }

    pub fn extract_cashtags_with_indices(&self, text: String) -> Result<ExtractResult, Error> {
        let mut extractor = RustValidatingExtractor::new(&self.config);
        let input = extractor.prep_input(&text);
        let result = extractor.extract_cashtags_with_indices(&input);
        let entities: Vec<Entity> = result.entities.iter().map(Entity::from).collect();
        Ok(ExtractResult::new(entities))
    }
}
