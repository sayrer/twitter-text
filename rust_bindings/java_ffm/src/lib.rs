/* Rust C-ABI exports for Java FFM bindings */

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::autolinker::Autolinker;
use twitter_text::extractor::Extractor;
use twitter_text::validator::Validator;
use twitter_text::TwitterTextParseResults;
use twitter_text_config::{Configuration, Range};

/* ============================================================================
 * C-compatible types
 * ========================================================================= */

#[repr(C)]
pub struct CRange {
    start: i32,
    end: i32,
}

impl From<Range> for CRange {
    fn from(r: Range) -> Self {
        CRange {
            start: r.start,
            end: r.end,
        }
    }
}

#[repr(C)]
pub struct CTwitterTextParseResults {
    weighted_length: i32,
    permillage: i32,
    is_valid: bool,
    display_text_range: CRange,
    valid_text_range: CRange,
}

impl From<TwitterTextParseResults> for CTwitterTextParseResults {
    fn from(r: TwitterTextParseResults) -> Self {
        CTwitterTextParseResults {
            weighted_length: r.weighted_length,
            permillage: r.permillage,
            is_valid: r.is_valid,
            display_text_range: r.display_text_range.into(),
            valid_text_range: r.valid_text_range.into(),
        }
    }
}

/* ============================================================================
 * Configuration API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_config_default() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::default()))
}

#[no_mangle]
pub extern "C" fn twitter_text_config_from_json(json: *const c_char) -> *mut Configuration {
    if json.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(json) };
    let json_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match serde_json::from_str::<Configuration>(json_str) {
        Ok(config) => Box::into_raw(Box::new(config)),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_free(config: *mut Configuration) {
    if !config.is_null() {
        unsafe {
            let _ = Box::from_raw(config);
        }
    }
}

/* ============================================================================
 * Validator API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_validator_new(config: *mut Configuration) -> *mut Validator {
    if config.is_null() {
        return std::ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    Box::into_raw(Box::new(Validator::new(config_ref.clone())))
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_free(validator: *mut Validator) {
    if !validator.is_null() {
        unsafe {
            let _ = Box::from_raw(validator);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_parse_tweet(
    validator: *mut Validator,
    text: *const c_char,
) -> CTwitterTextParseResults {
    if validator.is_null() || text.is_null() {
        return CTwitterTextParseResults {
            weighted_length: 0,
            permillage: 0,
            is_valid: false,
            display_text_range: CRange { start: 0, end: 0 },
            valid_text_range: CRange { start: 0, end: 0 },
        };
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CTwitterTextParseResults {
                weighted_length: 0,
                permillage: 0,
                is_valid: false,
                display_text_range: CRange { start: 0, end: 0 },
                valid_text_range: CRange { start: 0, end: 0 },
            }
        }
    };

    let results = validator_ref.parse_tweet(text_str);
    results.into()
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_is_valid_tweet(
    validator: *mut Validator,
    text: *const c_char,
) -> bool {
    if validator.is_null() || text.is_null() {
        return false;
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return false,
    };

    validator_ref.is_valid_tweet(text_str)
}

/* ============================================================================
 * Autolinker API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_autolinker_new(config: *mut Configuration) -> *mut Autolinker {
    if config.is_null() {
        return std::ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    Box::into_raw(Box::new(Autolinker::new(config_ref.clone())))
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
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/* ============================================================================
 * Extractor API
 * TODO: Implement extractor functions
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_extractor_new(config: *mut Configuration) -> *mut Extractor {
    if config.is_null() {
        return std::ptr::null_mut();
    }

    let config_ref = unsafe { &*config };
    Box::into_raw(Box::new(Extractor::new(config_ref.clone())))
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_free(extractor: *mut Extractor) {
    if !extractor.is_null() {
        unsafe {
            let _ = Box::from_raw(extractor);
        }
    }
}

/* TODO: Add extract_urls, extract_hashtags, etc.
 * Consider returning JSON strings for complex types:
 *
 * #[no_mangle]
 * pub extern "C" fn twitter_text_extractor_extract_urls_json(
 *     extractor: *mut Extractor,
 *     text: *const c_char,
 * ) -> *mut c_char {
 *     // Extract URLs and serialize to JSON
 * }
 */

/* ============================================================================
 * Utility functions
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
