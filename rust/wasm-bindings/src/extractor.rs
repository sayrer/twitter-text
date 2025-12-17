use js_sys::Array;
use twitter_text::entity::{Entity as RustEntity, Type as EntityType};
use twitter_text::extractor::{Extract, Extractor as RustExtractor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Entity {
    entity_type: i32,
    start: i32,
    end: i32,
    value: String,
    list_slug: String,
}

#[wasm_bindgen]
impl Entity {
    #[wasm_bindgen(getter, js_name = "entityType")]
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }

    #[wasm_bindgen(getter)]
    pub fn start(&self) -> i32 {
        self.start
    }

    #[wasm_bindgen(getter)]
    pub fn end(&self) -> i32 {
        self.end
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.value.clone()
    }

    #[wasm_bindgen(getter, js_name = "listSlug")]
    pub fn list_slug(&self) -> String {
        self.list_slug.clone()
    }

    #[wasm_bindgen(js_name = "isUrl")]
    pub fn is_url(&self) -> bool {
        self.entity_type == EntityType::URL as i32
    }

    #[wasm_bindgen(js_name = "isMention")]
    pub fn is_mention(&self) -> bool {
        self.entity_type == EntityType::MENTION as i32
    }

    #[wasm_bindgen(js_name = "isHashtag")]
    pub fn is_hashtag(&self) -> bool {
        self.entity_type == EntityType::HASHTAG as i32
    }

    #[wasm_bindgen(js_name = "isCashtag")]
    pub fn is_cashtag(&self) -> bool {
        self.entity_type == EntityType::CASHTAG as i32
    }

    #[wasm_bindgen(js_name = "isFederatedMention")]
    pub fn is_federated_mention(&self) -> bool {
        self.entity_type == EntityType::FEDERATEDMENTION as i32
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
        }
    }
}

#[wasm_bindgen]
pub struct Extractor {
    inner: RustExtractor,
}

#[wasm_bindgen]
impl Extractor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Extractor {
        Extractor {
            inner: RustExtractor::new(),
        }
    }

    #[wasm_bindgen(js_name = "getExtractUrlWithoutProtocol")]
    pub fn get_extract_url_without_protocol(&self) -> bool {
        self.inner.get_extract_url_without_protocol()
    }

    #[wasm_bindgen(js_name = "setExtractUrlWithoutProtocol")]
    pub fn set_extract_url_without_protocol(&mut self, extract: bool) {
        self.inner.set_extract_url_without_protocol(extract);
    }

    #[wasm_bindgen(js_name = "extractEntitiesWithIndices")]
    pub fn extract_entities_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_entities_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractMentionedScreennames")]
    pub fn extract_mentioned_screennames(&self, text: &str) -> Array {
        let mentions = self.inner.extract_mentioned_screennames(text);
        mentions.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "extractMentionedScreennamesWithIndices")]
    pub fn extract_mentioned_screennames_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_mentioned_screennames_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractMentionsOrListsWithIndices")]
    pub fn extract_mentions_or_lists_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_mentions_or_lists_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractReplyScreenname")]
    pub fn extract_reply_screenname(&self, text: &str) -> Option<Entity> {
        self.inner
            .extract_reply_username(text)
            .map(|e| Entity::from(&e))
    }

    #[wasm_bindgen(js_name = "extractUrls")]
    pub fn extract_urls(&self, text: &str) -> Array {
        let urls = self.inner.extract_urls(text);
        urls.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "extractUrlsWithIndices")]
    pub fn extract_urls_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_urls_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractHashtags")]
    pub fn extract_hashtags(&self, text: &str) -> Array {
        let hashtags = self.inner.extract_hashtags(text);
        hashtags.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "extractHashtagsWithIndices")]
    pub fn extract_hashtags_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_hashtags_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractCashtags")]
    pub fn extract_cashtags(&self, text: &str) -> Array {
        let cashtags = self.inner.extract_cashtags(text);
        cashtags.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "extractCashtagsWithIndices")]
    pub fn extract_cashtags_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_cashtags_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractFederatedMentions")]
    pub fn extract_federated_mentions(&self, text: &str) -> Array {
        let mentions = self.inner.extract_federated_mentions(text);
        mentions.into_iter().map(JsValue::from).collect()
    }

    #[wasm_bindgen(js_name = "extractFederatedMentionsWithIndices")]
    pub fn extract_federated_mentions_with_indices(&self, text: &str) -> Array {
        let entities = self.inner.extract_federated_mentions_with_indices(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    #[wasm_bindgen(js_name = "extractEntitiesWithIndicesFederated")]
    pub fn extract_entities_with_indices_federated(&self, text: &str) -> Array {
        let entities = self.inner.extract_entities_with_indices_federated(text);
        entities
            .iter()
            .map(|e| JsValue::from(Entity::from(e)))
            .collect()
    }

    /// Batch extraction of URLs from multiple texts.
    /// This reduces WASM-JS boundary crossings by processing multiple texts in one call.
    #[wasm_bindgen(js_name = "extractUrlsBatch")]
    pub fn extract_urls_batch(&self, texts: Vec<String>) -> Array {
        texts
            .iter()
            .map(|text| {
                let urls = self.inner.extract_urls(text);
                let arr: Array = urls.into_iter().map(JsValue::from).collect();
                JsValue::from(arr)
            })
            .collect()
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}
