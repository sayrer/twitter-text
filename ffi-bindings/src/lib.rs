use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::autolinker::{AddAttributeModifier, Autolinker, ReplaceClassModifier};
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
}

impl From<TwitterTextEntityType> for entity::Type {
    fn from(t: TwitterTextEntityType) -> Self {
        match t {
            TwitterTextEntityType::URL => entity::Type::URL,
            TwitterTextEntityType::HASHTAG => entity::Type::HASHTAG,
            TwitterTextEntityType::MENTION => entity::Type::MENTION,
            TwitterTextEntityType::CASHTAG => entity::Type::CASHTAG,
        }
    }
}

/* ============================================================================
 * Basic Autolinker C API
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

    unsafe {
        let text_str = match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        let result = (*autolinker).autolink(text_str);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_autolink_hashtags(
    autolinker: *mut Autolinker,
    text: *const c_char,
) -> *mut c_char {
    if autolinker.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let text_str = match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        let result = (*autolinker).autolink_hashtags(text_str);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/* ============================================================================
 * AddAttributeModifier C API
 * ========================================================================= */

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

/* ============================================================================
 * ReplaceClassModifier C API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_replace_class_modifier_new(
    new_class: *const c_char,
) -> *mut ReplaceClassModifier {
    if new_class.is_null() {
        return std::ptr::null_mut();
    }

    let class_str = unsafe { CStr::from_ptr(new_class) };
    if let Ok(s) = class_str.to_str() {
        Box::into_raw(Box::new(ReplaceClassModifier {
            new_class: s.to_string(),
        }))
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_replace_class_modifier_free(modifier: *mut ReplaceClassModifier) {
    if !modifier.is_null() {
        unsafe {
            let _ = Box::from_raw(modifier);
        }
    }
}

/* ============================================================================
 * Autolinker Modifier Setters
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_set_add_attribute_modifier(
    autolinker: *mut Autolinker,
    modifier: *mut AddAttributeModifier,
) {
    if autolinker.is_null() || modifier.is_null() {
        return;
    }

    unsafe {
        // Clone the modifier data so the caller can still free its copy
        let modifier_ref = &*modifier;
        let cloned_modifier = Box::new(AddAttributeModifier {
            entity_types: modifier_ref.entity_types.clone(),
            key: modifier_ref.key.clone(),
            value: modifier_ref.value.clone(),
        });
        (*autolinker).link_attribute_modifier = Some(cloned_modifier);
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
        // Clone the modifier data so the caller can still free its copy
        let modifier_ref = &*modifier;
        let cloned_modifier = Box::new(ReplaceClassModifier {
            new_class: modifier_ref.new_class.clone(),
        });
        (*autolinker).link_attribute_modifier = Some(cloned_modifier);
    }
}
