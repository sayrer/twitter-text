extern crate twitter_text_config;
extern crate twitter_text;

use twitter_text_config::Configuration;
use twitter_text_config::Range;
use twitter_text_config::WeightedRange;
use twitter_text::autolinker::Autolinker as RustAutolinker;
use twitter_text::TwitterTextParseResults;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Configuration;
        type TwitterTextParseResults;
        fn parse(text: &str, config: &Configuration, extract_urls: bool) -> TwitterTextParseResults;
    }
}

fn parse(text: &str, config: &Configuration, extract_urls: bool) -> TwitterTextParseResults {
    twitter_text::parse(text, &config, extract_urls)
}


/*
use libc::size_t;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::mem;
use std::slice;

//
// Configuration
//

#[no_mangle]
pub extern fn config_v1_new() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::from(twitter_text_config::config_v1())))
}

#[no_mangle]
pub extern fn config_v2_new() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::from(twitter_text_config::config_v2())))
}

#[no_mangle]
pub extern fn config_v3_new() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::from(twitter_text_config::config_v3())))
}

#[no_mangle]
pub extern fn default_config_new() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::from(twitter_text_config::default())))
}

#[no_mangle]
pub extern fn configuration_free(ptr: *mut Configuration) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[repr(C)]
pub struct WeightedRangeVec {
    pub data: *const WeightedRange,
    pub len: size_t,
}

fn unwrap(v: &WeightedRangeVec) -> Vec<WeightedRange> {
    unsafe { slice::from_raw_parts(v.data, v.len).to_vec() }
}

fn wrap(v: Vec<WeightedRange>) -> WeightedRangeVec {
    let wrv = WeightedRangeVec {
        data: v.as_ptr() as *const WeightedRange,
        len: v.len() as size_t,
    };
    mem::forget(v);
    wrv
}

#[repr(C)]
pub struct Configuration {
    pub version: i32,
    pub max_weighted_tweet_length: i32,
    pub scale: i32,
    pub default_weight: i32,
    pub transformed_url_length: i32,
    pub ranges: WeightedRangeVec,
    pub emoji_parsing_enabled: bool,
}

impl Configuration {
    pub fn from(config: &RustConfiguration) -> Configuration {
        Configuration {
            version: config.version,
            max_weighted_tweet_length: config.max_weighted_tweet_length,
            scale: config.scale,
            default_weight: config.default_weight,
            transformed_url_length: config.transformed_url_length,
            ranges: wrap(config.ranges.clone()),
            emoji_parsing_enabled: config.emoji_parsing_enabled
        }
    }
}

//
// Autolinker
//
pub struct Autolinker<'a> {
    autolinker: RustAutolinker<'a>,
    config: AutolinkerConfig,
}

pub struct AutolinkerConfig {
    pub no_follow: bool,
    pub url_class: *const c_char,
    pub url_target: *const c_char,
    pub symbol_tag: *const c_char,
    pub text_with_symbol_tag: *const c_char,
    pub list_class: *const c_char,
    pub username_class: *const c_char,
    pub hashtag_class: *const c_char,
    pub cashtag_class: *const c_char,
    pub username_url_base: *const c_char,
    pub list_url_base: *const c_char,
    pub hashtag_url_base: *const c_char,
    pub cashtag_url_base: *const c_char,
    pub invisible_tag_attrs: *const c_char,
    pub username_include_symbol: bool,
}

impl AutolinkerConfig {
    pub fn from(autolinker: &RustAutolinker) -> AutolinkerConfig {
        AutolinkerConfig {
            no_follow: autolinker.no_follow,
            url_class: CString::new(autolinker.url_class).unwrap().as_ptr(),
            url_target: CString::new(autolinker.url_target).unwrap().as_ptr(),
            symbol_tag: CString::new(autolinker.symbol_tag).unwrap().as_ptr(),
            text_with_symbol_tag: CString::new(autolinker.text_with_symbol_tag).unwrap().as_ptr(),
            list_class: CString::new(autolinker.list_class).unwrap().as_ptr(),
            username_class: CString::new(autolinker.username_class).unwrap().as_ptr(),
            hashtag_class: CString::new(autolinker.hashtag_class).unwrap().as_ptr(),
            cashtag_class: CString::new(autolinker.cashtag_class).unwrap().as_ptr(),
            username_url_base: CString::new(autolinker.username_url_base).unwrap().as_ptr(),
            list_url_base: CString::new(autolinker.list_url_base).unwrap().as_ptr(),
            hashtag_url_base: CString::new(autolinker.hashtag_url_base).unwrap().as_ptr(),
            cashtag_url_base: CString::new(autolinker.cashtag_url_base).unwrap().as_ptr(),
            invisible_tag_attrs: CString::new(autolinker.invisible_tag_attrs).unwrap().as_ptr(),
            username_include_symbol: autolinker.username_include_symbol,
        }
    }
}

#[no_mangle]
pub extern fn autolinker_new(no_follow: bool) -> *mut Autolinker<'static> {
    let linker = RustAutolinker::new(no_follow);
    let config = AutolinkerConfig::from(&linker);
    Box::into_raw(Box::new(Autolinker {
        autolinker: linker,
        config
    }))
}

#[no_mangle]
pub extern fn autolinker_free(ptr: *mut Autolinker) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn parse(text: *const c_char, config: *mut Configuration, extract_urls: bool) -> *mut TwitterTextParseResults {
    let cstr = unsafe { CStr::from_ptr(text) };

    match cstr.to_str() {
        Ok(s) => {
            twitter_text::parse(s, &config, extract_urls)
        }
        Err(_) => {
            TwitterTextParseResults::empty()
        }
    }
}
*/