use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::autolinker::Autolinker;

// Re-export the basic autolinker API and modifier types/functions from ffi-bindings
pub use twitter_text_ffi::{
    twitter_text_add_attribute_modifier_free, twitter_text_add_attribute_modifier_new,
    twitter_text_autolinker_autolink, twitter_text_autolinker_autolink_hashtags,
    twitter_text_autolinker_free, twitter_text_autolinker_new,
    twitter_text_autolinker_set_add_attribute_modifier,
    twitter_text_autolinker_set_link_text_modifier,
    twitter_text_autolinker_set_replace_class_modifier, twitter_text_replace_class_modifier_free,
    twitter_text_replace_class_modifier_new, twitter_text_string_free, CEntity,
    TwitterTextEntityType,
};

/* ============================================================================
 * Configuration setters
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_url_class(
    autolinker: *mut Autolinker<'static>,
    url_class: *const c_char,
) {
    if autolinker.is_null() || url_class.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(url_class) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).url_class = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_url_target(
    autolinker: *mut Autolinker<'static>,
    url_target: *const c_char,
) {
    if autolinker.is_null() || url_target.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(url_target) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).url_target = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_symbol_tag(
    autolinker: *mut Autolinker<'static>,
    symbol_tag: *const c_char,
) {
    if autolinker.is_null() || symbol_tag.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(symbol_tag) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).symbol_tag = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_text_with_symbol_tag(
    autolinker: *mut Autolinker<'static>,
    text_with_symbol_tag: *const c_char,
) {
    if autolinker.is_null() || text_with_symbol_tag.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(text_with_symbol_tag) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).text_with_symbol_tag = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_list_class(
    autolinker: *mut Autolinker<'static>,
    list_class: *const c_char,
) {
    if autolinker.is_null() || list_class.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(list_class) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).list_class = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_username_class(
    autolinker: *mut Autolinker<'static>,
    username_class: *const c_char,
) {
    if autolinker.is_null() || username_class.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(username_class) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).username_class = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_hashtag_class(
    autolinker: *mut Autolinker<'static>,
    hashtag_class: *const c_char,
) {
    if autolinker.is_null() || hashtag_class.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(hashtag_class) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).hashtag_class = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_cashtag_class(
    autolinker: *mut Autolinker<'static>,
    cashtag_class: *const c_char,
) {
    if autolinker.is_null() || cashtag_class.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(cashtag_class) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).cashtag_class = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_username_url_base(
    autolinker: *mut Autolinker<'static>,
    username_url_base: *const c_char,
) {
    if autolinker.is_null() || username_url_base.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(username_url_base) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).username_url_base = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_list_url_base(
    autolinker: *mut Autolinker<'static>,
    list_url_base: *const c_char,
) {
    if autolinker.is_null() || list_url_base.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(list_url_base) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).list_url_base = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_hashtag_url_base(
    autolinker: *mut Autolinker<'static>,
    hashtag_url_base: *const c_char,
) {
    if autolinker.is_null() || hashtag_url_base.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(hashtag_url_base) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).hashtag_url_base = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_cashtag_url_base(
    autolinker: *mut Autolinker<'static>,
    cashtag_url_base: *const c_char,
) {
    if autolinker.is_null() || cashtag_url_base.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(cashtag_url_base) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).cashtag_url_base = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_invisible_tag_attrs(
    autolinker: *mut Autolinker<'static>,
    invisible_tag_attrs: *const c_char,
) {
    if autolinker.is_null() || invisible_tag_attrs.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(invisible_tag_attrs) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*autolinker).invisible_tag_attrs = Box::leak(s.to_string().into_boxed_str());
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_username_include_symbol(
    autolinker: *mut Autolinker,
    username_include_symbol: bool,
) {
    if !autolinker.is_null() {
        unsafe {
            (*autolinker).username_include_symbol = username_include_symbol;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_no_follow(
    autolinker: *mut Autolinker,
    no_follow: bool,
) {
    if !autolinker.is_null() {
        unsafe {
            (*autolinker).no_follow = no_follow;
        }
    }
}

/* ============================================================================
 * Autolinking functions
 * ========================================================================= */

// Basic autolinking functions are re-exported from ffi-bindings at the top of this file
// Only the specialized functions that are not in ffi-bindings are defined here:

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_autolink_usernames_and_lists(
    autolinker: *mut Autolinker,
    text: *const c_char,
) -> *mut c_char {
    if autolinker.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    let autolinker_ref = unsafe { &*autolinker };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = autolinker_ref.autolink_usernames_and_lists(text_str);
    CString::new(result).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_autolink_urls(
    autolinker: *mut Autolinker,
    text: *const c_char,
) -> *mut c_char {
    if autolinker.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    let autolinker_ref = unsafe { &*autolinker };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = autolinker_ref.autolink_urls(text_str);
    CString::new(result).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_autolink_cashtags(
    autolinker: *mut Autolinker,
    text: *const c_char,
) -> *mut c_char {
    if autolinker.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    let autolinker_ref = unsafe { &*autolinker };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let result = autolinker_ref.autolink_cashtags(text_str);
    CString::new(result).unwrap_or_default().into_raw()
}

/* ============================================================================
 * Default constants
 * ========================================================================= */

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_LIST_CLASS: &str = twitter_text::autolinker::DEFAULT_LIST_CLASS;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_USERNAME_CLASS: &str =
    twitter_text::autolinker::DEFAULT_USERNAME_CLASS;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_HASHTAG_CLASS: &str =
    twitter_text::autolinker::DEFAULT_HASHTAG_CLASS;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_CASHTAG_CLASS: &str =
    twitter_text::autolinker::DEFAULT_CASHTAG_CLASS;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_USERNAME_URL_BASE: &str =
    twitter_text::autolinker::DEFAULT_USERNAME_URL_BASE;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_LIST_URL_BASE: &str =
    twitter_text::autolinker::DEFAULT_LIST_URL_BASE;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_HASHTAG_URL_BASE: &str =
    twitter_text::autolinker::DEFAULT_HASHTAG_URL_BASE;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_CASHTAG_URL_BASE: &str =
    twitter_text::autolinker::DEFAULT_CASHTAG_URL_BASE;

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_INVISIBLE_TAG_ATTRS: &str =
    twitter_text::autolinker::DEFAULT_INVISIBLE_TAG_ATTRS;

/* ============================================================================
 * Link Attribute Modifiers
 * ========================================================================= */

// Modifier functions are re-exported from ffi-bindings at the top of this file
