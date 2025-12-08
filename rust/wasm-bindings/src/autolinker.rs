use twitter_text::autolinker::{
    Autolinker as RustAutolinker, DEFAULT_CASHTAG_CLASS, DEFAULT_CASHTAG_URL_BASE,
    DEFAULT_HASHTAG_CLASS, DEFAULT_HASHTAG_URL_BASE, DEFAULT_INVISIBLE_TAG_ATTRS,
    DEFAULT_LIST_CLASS, DEFAULT_LIST_URL_BASE, DEFAULT_USERNAME_CLASS, DEFAULT_USERNAME_URL_BASE,
};
use twitter_text::extractor::{Extract, Extractor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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
}

#[wasm_bindgen]
impl Autolinker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Autolinker {
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
        }
    }

    #[wasm_bindgen(getter, js_name = "noFollow")]
    pub fn no_follow(&self) -> bool {
        self.no_follow
    }

    #[wasm_bindgen(setter, js_name = "noFollow")]
    pub fn set_no_follow(&mut self, no_follow: bool) {
        self.no_follow = no_follow;
    }

    #[wasm_bindgen(getter, js_name = "urlClass")]
    pub fn url_class(&self) -> String {
        self.url_class.clone()
    }

    #[wasm_bindgen(setter, js_name = "urlClass")]
    pub fn set_url_class(&mut self, url_class: String) {
        self.url_class = url_class;
    }

    #[wasm_bindgen(getter, js_name = "urlTarget")]
    pub fn url_target(&self) -> String {
        self.url_target.clone()
    }

    #[wasm_bindgen(setter, js_name = "urlTarget")]
    pub fn set_url_target(&mut self, url_target: String) {
        self.url_target = url_target;
    }

    #[wasm_bindgen(getter, js_name = "listClass")]
    pub fn list_class(&self) -> String {
        self.list_class.clone()
    }

    #[wasm_bindgen(setter, js_name = "listClass")]
    pub fn set_list_class(&mut self, list_class: String) {
        self.list_class = list_class;
    }

    #[wasm_bindgen(getter, js_name = "usernameClass")]
    pub fn username_class(&self) -> String {
        self.username_class.clone()
    }

    #[wasm_bindgen(setter, js_name = "usernameClass")]
    pub fn set_username_class(&mut self, username_class: String) {
        self.username_class = username_class;
    }

    #[wasm_bindgen(getter, js_name = "hashtagClass")]
    pub fn hashtag_class(&self) -> String {
        self.hashtag_class.clone()
    }

    #[wasm_bindgen(setter, js_name = "hashtagClass")]
    pub fn set_hashtag_class(&mut self, hashtag_class: String) {
        self.hashtag_class = hashtag_class;
    }

    #[wasm_bindgen(getter, js_name = "cashtagClass")]
    pub fn cashtag_class(&self) -> String {
        self.cashtag_class.clone()
    }

    #[wasm_bindgen(setter, js_name = "cashtagClass")]
    pub fn set_cashtag_class(&mut self, cashtag_class: String) {
        self.cashtag_class = cashtag_class;
    }

    #[wasm_bindgen(getter, js_name = "usernameUrlBase")]
    pub fn username_url_base(&self) -> String {
        self.username_url_base.clone()
    }

    #[wasm_bindgen(setter, js_name = "usernameUrlBase")]
    pub fn set_username_url_base(&mut self, base: String) {
        self.username_url_base = base;
    }

    #[wasm_bindgen(getter, js_name = "listUrlBase")]
    pub fn list_url_base(&self) -> String {
        self.list_url_base.clone()
    }

    #[wasm_bindgen(setter, js_name = "listUrlBase")]
    pub fn set_list_url_base(&mut self, base: String) {
        self.list_url_base = base;
    }

    #[wasm_bindgen(getter, js_name = "hashtagUrlBase")]
    pub fn hashtag_url_base(&self) -> String {
        self.hashtag_url_base.clone()
    }

    #[wasm_bindgen(setter, js_name = "hashtagUrlBase")]
    pub fn set_hashtag_url_base(&mut self, base: String) {
        self.hashtag_url_base = base;
    }

    #[wasm_bindgen(getter, js_name = "cashtagUrlBase")]
    pub fn cashtag_url_base(&self) -> String {
        self.cashtag_url_base.clone()
    }

    #[wasm_bindgen(setter, js_name = "cashtagUrlBase")]
    pub fn set_cashtag_url_base(&mut self, base: String) {
        self.cashtag_url_base = base;
    }

    #[wasm_bindgen(getter, js_name = "usernameIncludeSymbol")]
    pub fn username_include_symbol(&self) -> bool {
        self.username_include_symbol
    }

    #[wasm_bindgen(setter, js_name = "usernameIncludeSymbol")]
    pub fn set_username_include_symbol(&mut self, include: bool) {
        self.username_include_symbol = include;
    }

    pub fn autolink(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink(text)
    }

    #[wasm_bindgen(js_name = "autolinkUsernamesAndLists")]
    pub fn autolink_usernames_and_lists(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_usernames_and_lists(text)
    }

    #[wasm_bindgen(js_name = "autolinkHashtags")]
    pub fn autolink_hashtags(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_hashtags(text)
    }

    #[wasm_bindgen(js_name = "autolinkUrls")]
    pub fn autolink_urls(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_urls(text)
    }

    #[wasm_bindgen(js_name = "autolinkCashtags")]
    pub fn autolink_cashtags(&self, text: &str) -> String {
        self.to_rust_autolinker().autolink_cashtags(text)
    }
}

impl Autolinker {
    fn to_rust_autolinker(&self) -> RustAutolinker<'_> {
        let mut extractor = Extractor::new();
        extractor.set_extract_url_without_protocol(false);

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
            link_attribute_modifier: None,
            link_text_modifier: None,
        }
    }
}

impl Default for Autolinker {
    fn default() -> Self {
        Self::new()
    }
}
