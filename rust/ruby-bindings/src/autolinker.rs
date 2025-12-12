use std::cell::{Cell, RefCell};
use twitter_text::autolinker::{
    AddAttributeModifier as RustAddAttributeModifier, Autolinker as RustAutolinker,
    ReplaceClassModifier as RustReplaceClassModifier, DEFAULT_CASHTAG_CLASS,
    DEFAULT_CASHTAG_URL_BASE, DEFAULT_HASHTAG_CLASS, DEFAULT_HASHTAG_URL_BASE,
    DEFAULT_INVISIBLE_TAG_ATTRS, DEFAULT_LIST_CLASS, DEFAULT_LIST_URL_BASE, DEFAULT_USERNAME_CLASS,
    DEFAULT_USERNAME_URL_BASE,
};
use twitter_text::entity;

#[magnus::wrap(class = "Twittertext::Autolinker", free_immediately, size)]
pub struct Autolinker {
    no_follow: Cell<bool>,
    url_class: RefCell<String>,
    url_target: RefCell<String>,
    symbol_tag: RefCell<String>,
    text_with_symbol_tag: RefCell<String>,
    list_class: RefCell<String>,
    username_class: RefCell<String>,
    hashtag_class: RefCell<String>,
    cashtag_class: RefCell<String>,
    username_url_base: RefCell<String>,
    list_url_base: RefCell<String>,
    hashtag_url_base: RefCell<String>,
    cashtag_url_base: RefCell<String>,
    invisible_tag_attrs: RefCell<String>,
    username_include_symbol: Cell<bool>,
    add_attribute_modifier: RefCell<Option<AddAttributeModifier>>,
    replace_class_modifier: RefCell<Option<ReplaceClassModifier>>,
    link_text_modifier: RefCell<Option<LinkTextModifier>>,
}

// SAFETY: Ruby's GIL ensures single-threaded access to Ruby objects.
// The contained Proc in LinkTextModifier will only be called from within Ruby code execution.
unsafe impl Send for Autolinker {}
unsafe impl Sync for Autolinker {}

macro_rules! with_autolinker {
    ($self:expr, $text:expr, $method:ident) => {{
        let url_class = $self.url_class.borrow().clone();
        let url_target = $self.url_target.borrow().clone();
        let symbol_tag = $self.symbol_tag.borrow().clone();
        let text_with_symbol_tag = $self.text_with_symbol_tag.borrow().clone();
        let list_class = $self.list_class.borrow().clone();
        let username_class = $self.username_class.borrow().clone();
        let hashtag_class = $self.hashtag_class.borrow().clone();
        let cashtag_class = $self.cashtag_class.borrow().clone();
        let username_url_base = $self.username_url_base.borrow().clone();
        let list_url_base = $self.list_url_base.borrow().clone();
        let hashtag_url_base = $self.hashtag_url_base.borrow().clone();
        let cashtag_url_base = $self.cashtag_url_base.borrow().clone();
        let invisible_tag_attrs = $self.invisible_tag_attrs.borrow().clone();

        let link_attribute_modifier: Option<
            Box<dyn twitter_text::autolinker::LinkAttributeModifier>,
        > = if let Some(ref modifier) = *$self.add_attribute_modifier.borrow() {
            Some(Box::new(RustAddAttributeModifier {
                entity_types: modifier.entity_types.clone(),
                key: modifier.key.clone(),
                value: modifier.value.clone(),
            }))
        } else if let Some(ref modifier) = *$self.replace_class_modifier.borrow() {
            Some(Box::new(RustReplaceClassModifier {
                new_class: modifier.new_class.clone(),
            }))
        } else {
            None
        };

        let link_text_modifier: Option<Box<dyn twitter_text::autolinker::LinkTextModifier>> =
            if let Some(ref modifier) = *$self.link_text_modifier.borrow() {
                Some(Box::new(RustLinkTextModifier {
                    modifier_proc: modifier.modifier_proc.clone(),
                }))
            } else {
                None
            };

        let mut autolinker = RustAutolinker::new($self.no_follow.get());
        autolinker.url_class = &url_class;
        autolinker.url_target = &url_target;
        autolinker.symbol_tag = &symbol_tag;
        autolinker.text_with_symbol_tag = &text_with_symbol_tag;
        autolinker.list_class = &list_class;
        autolinker.username_class = &username_class;
        autolinker.hashtag_class = &hashtag_class;
        autolinker.cashtag_class = &cashtag_class;
        autolinker.username_url_base = &username_url_base;
        autolinker.list_url_base = &list_url_base;
        autolinker.hashtag_url_base = &hashtag_url_base;
        autolinker.cashtag_url_base = &cashtag_url_base;
        autolinker.invisible_tag_attrs = &invisible_tag_attrs;
        autolinker.username_include_symbol = $self.username_include_symbol.get();
        autolinker.link_attribute_modifier = link_attribute_modifier;
        autolinker.link_text_modifier = link_text_modifier;
        autolinker.$method($text)
    }};
}

impl Autolinker {
    pub fn ruby_new() -> Self {
        Autolinker {
            no_follow: Cell::new(false),
            url_class: RefCell::new(String::new()),
            url_target: RefCell::new(String::new()),
            symbol_tag: RefCell::new(String::new()),
            text_with_symbol_tag: RefCell::new(String::new()),
            list_class: RefCell::new(DEFAULT_LIST_CLASS.to_string()),
            username_class: RefCell::new(DEFAULT_USERNAME_CLASS.to_string()),
            hashtag_class: RefCell::new(DEFAULT_HASHTAG_CLASS.to_string()),
            cashtag_class: RefCell::new(DEFAULT_CASHTAG_CLASS.to_string()),
            username_url_base: RefCell::new(DEFAULT_USERNAME_URL_BASE.to_string()),
            list_url_base: RefCell::new(DEFAULT_LIST_URL_BASE.to_string()),
            hashtag_url_base: RefCell::new(DEFAULT_HASHTAG_URL_BASE.to_string()),
            cashtag_url_base: RefCell::new(DEFAULT_CASHTAG_URL_BASE.to_string()),
            invisible_tag_attrs: RefCell::new(DEFAULT_INVISIBLE_TAG_ATTRS.to_string()),
            username_include_symbol: Cell::new(false),
            add_attribute_modifier: RefCell::new(None),
            replace_class_modifier: RefCell::new(None),
            link_text_modifier: RefCell::new(None),
        }
    }

    pub fn get_no_follow(&self) -> bool {
        self.no_follow.get()
    }

    pub fn set_no_follow(&self, no_follow: bool) {
        self.no_follow.set(no_follow);
    }

    pub fn get_url_class(&self) -> String {
        self.url_class.borrow().clone()
    }

    pub fn set_url_class(&self, url_class: String) {
        *self.url_class.borrow_mut() = url_class;
    }

    pub fn get_url_target(&self) -> String {
        self.url_target.borrow().clone()
    }

    pub fn set_url_target(&self, url_target: String) {
        *self.url_target.borrow_mut() = url_target;
    }

    pub fn get_symbol_tag(&self) -> String {
        self.symbol_tag.borrow().clone()
    }

    pub fn set_symbol_tag(&self, symbol_tag: String) {
        *self.symbol_tag.borrow_mut() = symbol_tag;
    }

    pub fn get_text_with_symbol_tag(&self) -> String {
        self.text_with_symbol_tag.borrow().clone()
    }

    pub fn set_text_with_symbol_tag(&self, tag: String) {
        *self.text_with_symbol_tag.borrow_mut() = tag;
    }

    pub fn get_list_class(&self) -> String {
        self.list_class.borrow().clone()
    }

    pub fn set_list_class(&self, list_class: String) {
        *self.list_class.borrow_mut() = list_class;
    }

    pub fn get_username_class(&self) -> String {
        self.username_class.borrow().clone()
    }

    pub fn set_username_class(&self, username_class: String) {
        *self.username_class.borrow_mut() = username_class;
    }

    pub fn get_hashtag_class(&self) -> String {
        self.hashtag_class.borrow().clone()
    }

    pub fn set_hashtag_class(&self, hashtag_class: String) {
        *self.hashtag_class.borrow_mut() = hashtag_class;
    }

    pub fn get_cashtag_class(&self) -> String {
        self.cashtag_class.borrow().clone()
    }

    pub fn set_cashtag_class(&self, cashtag_class: String) {
        *self.cashtag_class.borrow_mut() = cashtag_class;
    }

    pub fn get_username_url_base(&self) -> String {
        self.username_url_base.borrow().clone()
    }

    pub fn set_username_url_base(&self, base: String) {
        *self.username_url_base.borrow_mut() = base;
    }

    pub fn get_list_url_base(&self) -> String {
        self.list_url_base.borrow().clone()
    }

    pub fn set_list_url_base(&self, base: String) {
        *self.list_url_base.borrow_mut() = base;
    }

    pub fn get_hashtag_url_base(&self) -> String {
        self.hashtag_url_base.borrow().clone()
    }

    pub fn set_hashtag_url_base(&self, base: String) {
        *self.hashtag_url_base.borrow_mut() = base;
    }

    pub fn get_cashtag_url_base(&self) -> String {
        self.cashtag_url_base.borrow().clone()
    }

    pub fn set_cashtag_url_base(&self, base: String) {
        *self.cashtag_url_base.borrow_mut() = base;
    }

    pub fn get_invisible_tag_attrs(&self) -> String {
        self.invisible_tag_attrs.borrow().clone()
    }

    pub fn set_invisible_tag_attrs(&self, attrs: String) {
        *self.invisible_tag_attrs.borrow_mut() = attrs;
    }

    pub fn get_username_include_symbol(&self) -> bool {
        self.username_include_symbol.get()
    }

    pub fn set_username_include_symbol(&self, include: bool) {
        self.username_include_symbol.set(include);
    }

    pub fn autolink(&self, text: String) -> String {
        with_autolinker!(self, &text, autolink)
    }

    pub fn autolink_usernames_and_lists(&self, text: String) -> String {
        with_autolinker!(self, &text, autolink_usernames_and_lists)
    }

    pub fn autolink_hashtags(&self, text: String) -> String {
        with_autolinker!(self, &text, autolink_hashtags)
    }

    pub fn autolink_urls(&self, text: String) -> String {
        with_autolinker!(self, &text, autolink_urls)
    }

    pub fn autolink_cashtags(&self, text: String) -> String {
        with_autolinker!(self, &text, autolink_cashtags)
    }

    pub fn set_add_attribute_modifier(&self, modifier: &AddAttributeModifier) {
        *self.add_attribute_modifier.borrow_mut() = Some(modifier.clone());
    }

    pub fn set_replace_class_modifier(&self, modifier: &ReplaceClassModifier) {
        *self.replace_class_modifier.borrow_mut() = Some(modifier.clone());
    }

    pub fn set_link_text_modifier(&self, modifier: &LinkTextModifier) {
        *self.link_text_modifier.borrow_mut() = Some(modifier.clone());
    }
}

/* ============================================================================
 * Link Attribute Modifiers
 * ========================================================================= */

#[derive(Clone)]
#[magnus::wrap(class = "Twittertext::AddAttributeModifier", free_immediately, size)]
pub struct AddAttributeModifier {
    entity_types: Vec<entity::Type>,
    key: String,
    value: String,
}

impl AddAttributeModifier {
    pub fn ruby_new(entity_types: Vec<String>, key: String, value: String) -> Self {
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

        AddAttributeModifier {
            entity_types: types,
            key,
            value,
        }
    }
}

#[derive(Clone)]
#[magnus::wrap(class = "Twittertext::ReplaceClassModifier", free_immediately, size)]
pub struct ReplaceClassModifier {
    new_class: String,
}

impl ReplaceClassModifier {
    pub fn ruby_new(new_class: String) -> Self {
        ReplaceClassModifier { new_class }
    }
}

/* ============================================================================
 * Link Text Modifiers
 * ========================================================================= */

use magnus::{block::Proc, TryConvert, Value};

#[derive(Clone)]
#[magnus::wrap(class = "Twittertext::LinkTextModifier", free_immediately, size)]
pub struct LinkTextModifier {
    modifier_proc: Proc,
}

// SAFETY: Ruby's GIL ensures single-threaded access to Ruby objects.
// The Proc will only be called from within Ruby code execution.
unsafe impl Send for LinkTextModifier {}
unsafe impl Sync for LinkTextModifier {}

impl LinkTextModifier {
    pub fn ruby_new(modifier_proc: Proc) -> Self {
        LinkTextModifier { modifier_proc }
    }
}

struct RustLinkTextModifier {
    modifier_proc: Proc,
}

// SAFETY: Ruby's GIL ensures single-threaded access to Ruby objects.
unsafe impl Send for RustLinkTextModifier {}
unsafe impl Sync for RustLinkTextModifier {}

impl twitter_text::autolinker::LinkTextModifier for RustLinkTextModifier {
    fn modify(&self, entity: &entity::Entity, text: &str) -> String {
        magnus::Ruby::get()
            .ok()
            .and_then(|ruby| {
                // Create a hash for the entity
                let entity_hash = ruby.hash_new();
                let type_str = format!("{:?}", entity.t);
                let _ = entity_hash.aset(ruby.to_symbol("type"), ruby.str_new(&type_str));
                let _ = entity_hash.aset(ruby.to_symbol("value"), ruby.str_new(entity.value));
                let _ = entity_hash.aset(ruby.to_symbol("start"), entity.start);
                let _ = entity_hash.aset(ruby.to_symbol("end"), entity.end);

                // Call the Ruby proc
                let result: Value = self
                    .modifier_proc
                    .call((entity_hash, ruby.str_new(text)))
                    .ok()?;

                // Extract the string result
                TryConvert::try_convert(result).ok()
            })
            .unwrap_or_else(|| text.to_string())
    }
}
