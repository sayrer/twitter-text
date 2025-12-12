use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::autolinker::Autolinker;
use twitter_text::entity;

/* ============================================================================
 * Entity Type Enum for C FFI
 * ========================================================================= */

#[repr(C)]
#[derive(Copy, Clone)]
pub enum TwitterTextEntityType {
    URL = 0,
    HASHTAG = 1,
    MENTION = 2,
    CASHTAG = 3,
    FEDERATEDMENTION = 4,
}

impl From<TwitterTextEntityType> for entity::Type {
    fn from(t: TwitterTextEntityType) -> Self {
        match t {
            TwitterTextEntityType::URL => entity::Type::URL,
            TwitterTextEntityType::HASHTAG => entity::Type::HASHTAG,
            TwitterTextEntityType::MENTION => entity::Type::MENTION,
            TwitterTextEntityType::CASHTAG => entity::Type::CASHTAG,
            TwitterTextEntityType::FEDERATEDMENTION => entity::Type::FEDERATEDMENTION,
        }
    }
}

/* ============================================================================
 * Basic Autolinker API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_new(no_follow: bool) -> *mut Autolinker<'static> {
    Box::into_raw(Box::new(Autolinker::new(no_follow)))
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_free(autolinker: *mut Autolinker) {
    if !autolinker.is_null() {
        unsafe {
            let _ = Box::from_raw(autolinker);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_autolink(
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

    let result = autolinker_ref.autolink(text_str);
    CString::new(result).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_autolink_hashtags(
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

    let result = autolinker_ref.autolink_hashtags(text_str);
    CString::new(result).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn twitter_text_string_free(str: *mut c_char) {
    if !str.is_null() {
        unsafe {
            let _ = CString::from_raw(str);
        }
    }
}

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

use twitter_text::autolinker::{AddAttributeModifier, ReplaceClassModifier};

// Constants for entity type filtering
pub const TWITTER_TEXT_ENTITY_URL: i32 = TwitterTextEntityType::URL as i32;
pub const TWITTER_TEXT_ENTITY_HASHTAG: i32 = TwitterTextEntityType::HASHTAG as i32;
pub const TWITTER_TEXT_ENTITY_MENTION: i32 = TwitterTextEntityType::MENTION as i32;
pub const TWITTER_TEXT_ENTITY_CASHTAG: i32 = TwitterTextEntityType::CASHTAG as i32;

#[no_mangle]
pub extern "C" fn twitter_text_add_attribute_modifier_new(
    entity_types: *const TwitterTextEntityType,
    entity_types_count: usize,
    key: *const c_char,
    value: *const c_char,
) -> *mut AddAttributeModifier {
    if entity_types.is_null() || key.is_null() || value.is_null() {
        return std::ptr::null_mut();
    }

    let types: Vec<entity::Type> = unsafe {
        std::slice::from_raw_parts(entity_types, entity_types_count)
            .iter()
            .map(|&t| entity::Type::from(t))
            .collect()
    };

    let key_str = unsafe { CStr::from_ptr(key) };
    let value_str = unsafe { CStr::from_ptr(value) };

    if let (Ok(k), Ok(v)) = (key_str.to_str(), value_str.to_str()) {
        Box::into_raw(Box::new(AddAttributeModifier {
            entity_types: types,
            key: k.to_string(),
            value: v.to_string(),
        }))
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_add_attribute_modifier_free(modifier: *mut AddAttributeModifier) {
    if !modifier.is_null() {
        unsafe {
            let _ = Box::from_raw(modifier);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_replace_class_modifier_new(
    new_class: *const c_char,
) -> *mut ReplaceClassModifier {
    if new_class.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(new_class) };
    let class_str = match c_str.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return std::ptr::null_mut(),
    };

    let modifier = ReplaceClassModifier::new(class_str);
    Box::into_raw(Box::new(modifier))
}

#[no_mangle]
pub extern "C" fn twitter_text_replace_class_modifier_free(modifier: *mut ReplaceClassModifier) {
    if !modifier.is_null() {
        unsafe {
            let _ = Box::from_raw(modifier);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_add_attribute_modifier(
    autolinker: *mut Autolinker,
    modifier: *mut AddAttributeModifier,
) {
    if autolinker.is_null() || modifier.is_null() {
        return;
    }

    unsafe {
        // Clone the modifier - caller still owns the original and should free it
        let modifier_clone = (*modifier).clone();
        // Clear any existing modifier first
        (*autolinker).link_attribute_modifier = None;
        (*autolinker).link_attribute_modifier = Some(Box::new(modifier_clone));
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_replace_class_modifier(
    autolinker: *mut Autolinker,
    modifier: *mut ReplaceClassModifier,
) {
    if autolinker.is_null() || modifier.is_null() {
        return;
    }

    unsafe {
        // Clone the modifier - caller still owns the original and should free it
        let modifier_clone = (*modifier).clone();
        // Clear any existing modifier first
        (*autolinker).link_attribute_modifier = None;
        (*autolinker).link_attribute_modifier = Some(Box::new(modifier_clone));
    }
}

/* ============================================================================
 * Link Text Modifier C API (callback-based)
 * ========================================================================= */

use twitter_text::autolinker::LinkTextModifier;

// C-compatible entity representation for callbacks
#[repr(C)]
pub struct CEntity {
    pub entity_type: TwitterTextEntityType,
    pub start: i32,
    pub end: i32,
}

impl From<&entity::Entity<'_>> for CEntity {
    fn from(entity: &entity::Entity) -> Self {
        let entity_type = match entity.t {
            entity::Type::URL => TwitterTextEntityType::URL,
            entity::Type::HASHTAG => TwitterTextEntityType::HASHTAG,
            entity::Type::MENTION => TwitterTextEntityType::MENTION,
            entity::Type::CASHTAG => TwitterTextEntityType::CASHTAG,
            entity::Type::FEDERATEDMENTION => TwitterTextEntityType::FEDERATEDMENTION,
        };
        CEntity {
            entity_type,
            start: entity.start,
            end: entity.end,
        }
    }
}

// Function pointer type for link text modification callback
// Returns a new C string that must be freed by the caller
pub type LinkTextModifierCallback = extern "C" fn(
    entity: *const CEntity,
    text: *const c_char,
    user_data: *mut std::os::raw::c_void,
) -> *mut c_char;

// Wrapper struct that implements LinkTextModifier trait using a callback
struct CallbackLinkTextModifier {
    callback: LinkTextModifierCallback,
    user_data: *mut std::os::raw::c_void,
}

impl LinkTextModifier for CallbackLinkTextModifier {
    fn modify(&self, entity: &entity::Entity, text: &str) -> String {
        let c_entity = CEntity::from(entity);

        let c_text = match CString::new(text) {
            Ok(s) => s,
            Err(_) => return text.to_string(),
        };

        let result_ptr = (self.callback)(&c_entity, c_text.as_ptr(), self.user_data);

        if result_ptr.is_null() {
            return text.to_string();
        }

        unsafe {
            let result_cstr = CStr::from_ptr(result_ptr);
            // Use to_str() instead of to_string_lossy() to preserve all Unicode
            let result_string = match result_cstr.to_str() {
                Ok(s) => s.to_string(),
                Err(_) => {
                    // If invalid UTF-8, free and return original text
                    libc::free(result_ptr as *mut libc::c_void);
                    return text.to_string();
                }
            };

            // Free the C string using libc::free() since it was allocated with malloc()
            // Note: Java FFM uses Arena.ofAuto() which doesn't actually use malloc,
            // but Arena memory is managed by the Java GC and calling free() on it is a no-op
            // or handled safely by the Java runtime's memory system.
            libc::free(result_ptr as *mut libc::c_void);

            result_string
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_link_text_modifier(
    autolinker: *mut Autolinker,
    callback: LinkTextModifierCallback,
    user_data: *mut std::os::raw::c_void,
) {
    if autolinker.is_null() {
        return;
    }

    unsafe {
        let modifier = Box::new(CallbackLinkTextModifier {
            callback,
            user_data,
        });
        (*autolinker).link_text_modifier = Some(modifier);
    }
}
