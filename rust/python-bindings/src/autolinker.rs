use pyo3::prelude::*;
use twitter_text::autolinker::{
    AddAttributeModifier as RustAddAttributeModifier, Autolinker as RustAutolinker,
    ReplaceClassModifier as RustReplaceClassModifier, DEFAULT_CASHTAG_CLASS,
    DEFAULT_CASHTAG_URL_BASE, DEFAULT_HASHTAG_CLASS, DEFAULT_HASHTAG_URL_BASE,
    DEFAULT_INVISIBLE_TAG_ATTRS, DEFAULT_LIST_CLASS, DEFAULT_LIST_URL_BASE, DEFAULT_USERNAME_CLASS,
    DEFAULT_USERNAME_URL_BASE,
};
use twitter_text::entity;
use twitter_text::extractor::{Extract, Extractor};

#[pyclass]
pub struct Autolinker {
    no_follow: bool,
    url_class: String,
    url_target: String,
    symbol_tag: String,
    text_with_symbol_tag: String,
    list_class: String,
    username_class: String,
    hashtag_class: String,
    cashtag_class: String,
    username_url_base: String,
    list_url_base: String,
    hashtag_url_base: String,
    cashtag_url_base: String,
    invisible_tag_attrs: String,
    username_include_symbol: bool,
    add_attribute_modifier: Option<AddAttributeModifier>,
    replace_class_modifier: Option<ReplaceClassModifier>,
}

#[pymethods]
impl Autolinker {
    #[new]
    fn new() -> Self {
        Autolinker {
            no_follow: false,
            url_class: String::new(),
            url_target: String::new(),
            symbol_tag: String::new(),
            text_with_symbol_tag: String::new(),
            list_class: DEFAULT_LIST_CLASS.to_string(),
            username_class: DEFAULT_USERNAME_CLASS.to_string(),
            hashtag_class: DEFAULT_HASHTAG_CLASS.to_string(),
            cashtag_class: DEFAULT_CASHTAG_CLASS.to_string(),
            username_url_base: DEFAULT_USERNAME_URL_BASE.to_string(),
            list_url_base: DEFAULT_LIST_URL_BASE.to_string(),
            hashtag_url_base: DEFAULT_HASHTAG_URL_BASE.to_string(),
            cashtag_url_base: DEFAULT_CASHTAG_URL_BASE.to_string(),
            invisible_tag_attrs: DEFAULT_INVISIBLE_TAG_ATTRS.to_string(),
            username_include_symbol: false,
            add_attribute_modifier: None,
            replace_class_modifier: None,
        }
    }

    fn get_no_follow(&self) -> bool {
        self.no_follow
    }

    fn set_no_follow(&mut self, no_follow: bool) {
        self.no_follow = no_follow;
    }

    fn get_url_class(&self) -> &str {
        &self.url_class
    }

    fn set_url_class(&mut self, url_class: String) {
        self.url_class = url_class;
    }

    fn get_url_target(&self) -> &str {
        &self.url_target
    }

    fn set_url_target(&mut self, url_target: String) {
        self.url_target = url_target;
    }

    fn get_symbol_tag(&self) -> &str {
        &self.symbol_tag
    }

    fn set_symbol_tag(&mut self, symbol_tag: String) {
        self.symbol_tag = symbol_tag;
    }

    fn get_text_with_symbol_tag(&self) -> &str {
        &self.text_with_symbol_tag
    }

    fn set_text_with_symbol_tag(&mut self, tag: String) {
        self.text_with_symbol_tag = tag;
    }

    fn get_list_class(&self) -> &str {
        &self.list_class
    }

    fn set_list_class(&mut self, list_class: String) {
        self.list_class = list_class;
    }

    fn get_username_class(&self) -> &str {
        &self.username_class
    }

    fn set_username_class(&mut self, username_class: String) {
        self.username_class = username_class;
    }

    fn get_hashtag_class(&self) -> &str {
        &self.hashtag_class
    }

    fn set_hashtag_class(&mut self, hashtag_class: String) {
        self.hashtag_class = hashtag_class;
    }

    fn get_cashtag_class(&self) -> &str {
        &self.cashtag_class
    }

    fn set_cashtag_class(&mut self, cashtag_class: String) {
        self.cashtag_class = cashtag_class;
    }

    fn get_username_url_base(&self) -> &str {
        &self.username_url_base
    }

    fn set_username_url_base(&mut self, base: String) {
        self.username_url_base = base;
    }

    fn get_list_url_base(&self) -> &str {
        &self.list_url_base
    }

    fn set_list_url_base(&mut self, base: String) {
        self.list_url_base = base;
    }

    fn get_hashtag_url_base(&self) -> &str {
        &self.hashtag_url_base
    }

    fn set_hashtag_url_base(&mut self, base: String) {
        self.hashtag_url_base = base;
    }

    fn get_cashtag_url_base(&self) -> &str {
        &self.cashtag_url_base
    }

    fn set_cashtag_url_base(&mut self, base: String) {
        self.cashtag_url_base = base;
    }

    fn get_invisible_tag_attrs(&self) -> &str {
        &self.invisible_tag_attrs
    }

    fn set_invisible_tag_attrs(&mut self, attrs: String) {
        self.invisible_tag_attrs = attrs;
    }

    fn get_username_include_symbol(&self) -> bool {
        self.username_include_symbol
    }

    fn set_username_include_symbol(&mut self, include: bool) {
        self.username_include_symbol = include;
    }

    fn set_add_attribute_modifier(&mut self, modifier: AddAttributeModifier) {
        self.add_attribute_modifier = Some(modifier);
    }

    fn set_replace_class_modifier(&mut self, modifier: ReplaceClassModifier) {
        self.replace_class_modifier = Some(modifier);
    }

    fn autolink(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink(text)
    }

    fn autolink_usernames_and_lists(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_usernames_and_lists(text)
    }

    fn autolink_hashtags(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_hashtags(text)
    }

    fn autolink_urls(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_urls(text)
    }

    fn autolink_cashtags(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_cashtags(text)
    }
}

impl Autolinker {
    fn to_rust_autolinker(&self) -> RustAutolinker {
        let mut extractor = Extractor::new();
        extractor.set_extract_url_without_protocol(false);

        let link_attribute_modifier: Option<
            Box<dyn twitter_text::autolinker::LinkAttributeModifier>,
        > = if let Some(ref modifier) = self.add_attribute_modifier {
            Some(Box::new(RustAddAttributeModifier {
                entity_types: modifier.entity_types.clone(),
                key: modifier.key.clone(),
                value: modifier.value.clone(),
            }))
        } else if let Some(ref modifier) = self.replace_class_modifier {
            Some(Box::new(RustReplaceClassModifier {
                new_class: modifier.new_class.clone(),
            }))
        } else {
            None
        };

        RustAutolinker {
            no_follow: self.no_follow,
            url_class: &self.url_class,
            url_target: &self.url_target,
            symbol_tag: &self.symbol_tag,
            text_with_symbol_tag: &self.text_with_symbol_tag,
            list_class: &self.list_class,
            username_class: &self.username_class,
            hashtag_class: &self.hashtag_class,
            cashtag_class: &self.cashtag_class,
            username_url_base: &self.username_url_base,
            list_url_base: &self.list_url_base,
            hashtag_url_base: &self.hashtag_url_base,
            cashtag_url_base: &self.cashtag_url_base,
            invisible_tag_attrs: &self.invisible_tag_attrs,
            username_include_symbol: self.username_include_symbol,
            extractor,
            link_attribute_modifier,
        }
    }
}

/* ============================================================================
 * Link Attribute Modifiers
 * ========================================================================= */

#[pyclass]
#[derive(Clone)]
pub struct AddAttributeModifier {
    entity_types: Vec<entity::Type>,
    key: String,
    value: String,
}

#[pymethods]
impl AddAttributeModifier {
    #[new]
    fn new(entity_types: Vec<String>, key: String, value: String) -> PyResult<Self> {
        let types: Vec<entity::Type> = entity_types
            .iter()
            .filter_map(|s| match s.to_uppercase().as_str() {
                "URL" => Some(entity::Type::URL),
                "HASHTAG" => Some(entity::Type::HASHTAG),
                "MENTION" => Some(entity::Type::MENTION),
                "CASHTAG" => Some(entity::Type::CASHTAG),
                _ => None,
            })
            .collect();

        Ok(AddAttributeModifier {
            entity_types: types,
            key,
            value,
        })
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ReplaceClassModifier {
    new_class: String,
}

#[pymethods]
impl ReplaceClassModifier {
    #[new]
    fn new(new_class: String) -> Self {
        ReplaceClassModifier { new_class }
    }
}
