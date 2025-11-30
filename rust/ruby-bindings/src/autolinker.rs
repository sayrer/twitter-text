use std::cell::{Cell, RefCell};
use twitter_text::autolinker::{
    Autolinker as RustAutolinker, DEFAULT_CASHTAG_CLASS, DEFAULT_CASHTAG_URL_BASE,
    DEFAULT_HASHTAG_CLASS, DEFAULT_HASHTAG_URL_BASE, DEFAULT_INVISIBLE_TAG_ATTRS,
    DEFAULT_LIST_CLASS, DEFAULT_LIST_URL_BASE, DEFAULT_USERNAME_CLASS, DEFAULT_USERNAME_URL_BASE,
};

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
}

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
}
