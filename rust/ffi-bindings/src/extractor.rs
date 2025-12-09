use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::extractor::Extractor;
use twitter_text_config::Configuration;

/* ============================================================================
 * C-compatible Entity types
 * ========================================================================= */

#[repr(C)]
pub struct CEntity {
    entity_type: i32,
    start: i32,
    end: i32,
    value: *mut c_char,
    list_slug: *mut c_char,
    display_url: *mut c_char,
    expanded_url: *mut c_char,
}

impl<'a> From<twitter_text::entity::Entity<'a>> for CEntity {
    fn from(e: twitter_text::entity::Entity<'a>) -> Self {
        CEntity {
            entity_type: e.t as i32,
            start: e.start,
            end: e.end,
            value: CString::new(e.value).unwrap_or_default().into_raw(),
            list_slug: CString::new(e.list_slug).unwrap_or_default().into_raw(),
            display_url: CString::new(e.display_url).unwrap_or_default().into_raw(),
            expanded_url: CString::new(e.expanded_url).unwrap_or_default().into_raw(),
        }
    }
}

#[repr(C)]
pub struct CEntityArray {
    entities: *mut CEntity,
    length: usize,
}

#[repr(C)]
pub struct CStringArray {
    strings: *mut *mut c_char,
    length: usize,
}

/* ============================================================================
 * Parse Results types
 * ========================================================================= */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CRange {
    pub start: i32,
    pub end: i32,
}

impl From<twitter_text_config::Range> for CRange {
    fn from(r: twitter_text_config::Range) -> Self {
        CRange {
            start: r.start(),
            end: r.end(),
        }
    }
}

#[repr(C)]
pub struct CParseResults {
    weighted_length: i32,
    permillage: i32,
    is_valid: bool,
    display_text_range: CRange,
    valid_text_range: CRange,
}

impl From<twitter_text::TwitterTextParseResults> for CParseResults {
    fn from(r: twitter_text::TwitterTextParseResults) -> Self {
        CParseResults {
            weighted_length: r.weighted_length,
            permillage: r.permillage,
            is_valid: r.is_valid,
            display_text_range: r.display_text_range.into(),
            valid_text_range: r.valid_text_range.into(),
        }
    }
}

#[repr(C)]
pub struct CExtractResult {
    parse_results: CParseResults,
    entities: CEntityArray,
}

#[repr(C)]
pub struct CMentionResult {
    parse_results: CParseResults,
    mention: *mut CEntity,
}

/* ============================================================================
 * Basic Extractor API (no validation)
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_extractor_new() -> *mut Extractor {
    Box::into_raw(Box::new(Extractor::new()))
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_free(extractor: *mut Extractor) {
    if !extractor.is_null() {
        unsafe {
            let _ = Box::from_raw(extractor);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_set_extract_url_without_protocol(
    extractor: *mut Extractor,
    extract_url_without_protocol: bool,
) {
    if !extractor.is_null() {
        unsafe {
            (*extractor).set_extract_url_without_protocol(extract_url_without_protocol);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_get_extract_url_without_protocol(
    extractor: *mut Extractor,
) -> bool {
    if extractor.is_null() {
        return false;
    }
    unsafe { (*extractor).get_extract_url_without_protocol() }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_urls(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CStringArray {
    if extractor.is_null() || text.is_null() {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CStringArray {
                strings: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let urls = extractor_ref.extract_urls(text_str);
    let length = urls.len();

    if length == 0 {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_strings: Vec<*mut c_char> = urls
        .into_iter()
        .map(|s| CString::new(s).unwrap_or_default().into_raw())
        .collect();

    let strings_ptr = c_strings.as_mut_ptr();
    std::mem::forget(c_strings);

    CStringArray {
        strings: strings_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_urls_with_indices(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CEntityArray {
    if extractor.is_null() || text.is_null() {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let entities = extractor_ref.extract_urls_with_indices(text_str);
    let length = entities.len();

    if length == 0 {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_entities: Vec<CEntity> = entities.into_iter().map(|e| e.into()).collect();

    let entities_ptr = c_entities.as_mut_ptr();
    std::mem::forget(c_entities);

    CEntityArray {
        entities: entities_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_hashtags(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CStringArray {
    if extractor.is_null() || text.is_null() {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CStringArray {
                strings: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let hashtags = extractor_ref.extract_hashtags(text_str);
    let length = hashtags.len();

    if length == 0 {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_strings: Vec<*mut c_char> = hashtags
        .into_iter()
        .map(|s| CString::new(s).unwrap_or_default().into_raw())
        .collect();

    let strings_ptr = c_strings.as_mut_ptr();
    std::mem::forget(c_strings);

    CStringArray {
        strings: strings_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_hashtags_with_indices(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CEntityArray {
    if extractor.is_null() || text.is_null() {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let entities = extractor_ref.extract_hashtags_with_indices(text_str);
    let length = entities.len();

    if length == 0 {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_entities: Vec<CEntity> = entities.into_iter().map(|e| e.into()).collect();

    let entities_ptr = c_entities.as_mut_ptr();
    std::mem::forget(c_entities);

    CEntityArray {
        entities: entities_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_cashtags(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CStringArray {
    if extractor.is_null() || text.is_null() {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CStringArray {
                strings: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let cashtags = extractor_ref.extract_cashtags(text_str);
    let length = cashtags.len();

    if length == 0 {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_strings: Vec<*mut c_char> = cashtags
        .into_iter()
        .map(|s| CString::new(s).unwrap_or_default().into_raw())
        .collect();

    let strings_ptr = c_strings.as_mut_ptr();
    std::mem::forget(c_strings);

    CStringArray {
        strings: strings_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_cashtags_with_indices(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CEntityArray {
    if extractor.is_null() || text.is_null() {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let entities = extractor_ref.extract_cashtags_with_indices(text_str);
    let length = entities.len();

    if length == 0 {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_entities: Vec<CEntity> = entities.into_iter().map(|e| e.into()).collect();

    let entities_ptr = c_entities.as_mut_ptr();
    std::mem::forget(c_entities);

    CEntityArray {
        entities: entities_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_mentioned_screennames(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CStringArray {
    if extractor.is_null() || text.is_null() {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CStringArray {
                strings: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let screennames = extractor_ref.extract_mentioned_screennames(text_str);
    let length = screennames.len();

    if length == 0 {
        return CStringArray {
            strings: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_strings: Vec<*mut c_char> = screennames
        .into_iter()
        .map(|s| CString::new(s).unwrap_or_default().into_raw())
        .collect();

    let strings_ptr = c_strings.as_mut_ptr();
    std::mem::forget(c_strings);

    CStringArray {
        strings: strings_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_mentioned_screennames_with_indices(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CEntityArray {
    if extractor.is_null() || text.is_null() {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let entities = extractor_ref.extract_mentioned_screennames_with_indices(text_str);
    let length = entities.len();

    if length == 0 {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_entities: Vec<CEntity> = entities.into_iter().map(|e| e.into()).collect();

    let entities_ptr = c_entities.as_mut_ptr();
    std::mem::forget(c_entities);

    CEntityArray {
        entities: entities_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_mentions_or_lists_with_indices(
    extractor: *mut Extractor,
    text: *const c_char,
) -> CEntityArray {
    if extractor.is_null() || text.is_null() {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            }
        }
    };

    let entities = extractor_ref.extract_mentions_or_lists_with_indices(text_str);
    let length = entities.len();

    if length == 0 {
        return CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_entities: Vec<CEntity> = entities.into_iter().map(|e| e.into()).collect();

    let entities_ptr = c_entities.as_mut_ptr();
    std::mem::forget(c_entities);

    CEntityArray {
        entities: entities_ptr,
        length,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extractor_extract_reply_username(
    extractor: *mut Extractor,
    text: *const c_char,
) -> *mut CEntity {
    if extractor.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match extractor_ref.extract_reply_username(text_str) {
        Some(entity) => {
            let c_entity: CEntity = entity.into();
            Box::into_raw(Box::new(c_entity))
        }
        None => std::ptr::null_mut(),
    }
}

/* ============================================================================
 * Free functions for complex types
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_entity_free(entity: *mut CEntity) {
    if !entity.is_null() {
        unsafe {
            let e = Box::from_raw(entity);
            if !e.value.is_null() {
                let _ = CString::from_raw(e.value);
            }
            if !e.list_slug.is_null() {
                let _ = CString::from_raw(e.list_slug);
            }
            if !e.display_url.is_null() {
                let _ = CString::from_raw(e.display_url);
            }
            if !e.expanded_url.is_null() {
                let _ = CString::from_raw(e.expanded_url);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_entity_array_free(array: CEntityArray) {
    if !array.entities.is_null() && array.length > 0 {
        unsafe {
            let entities = Vec::from_raw_parts(array.entities, array.length, array.length);
            for entity in entities {
                if !entity.value.is_null() {
                    let _ = CString::from_raw(entity.value);
                }
                if !entity.list_slug.is_null() {
                    let _ = CString::from_raw(entity.list_slug);
                }
                if !entity.display_url.is_null() {
                    let _ = CString::from_raw(entity.display_url);
                }
                if !entity.expanded_url.is_null() {
                    let _ = CString::from_raw(entity.expanded_url);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_string_array_free(array: CStringArray) {
    if !array.strings.is_null() && array.length > 0 {
        unsafe {
            let strings = Vec::from_raw_parts(array.strings, array.length, array.length);
            for string in strings {
                if !string.is_null() {
                    let _ = CString::from_raw(string);
                }
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_extract_result_free(result: CExtractResult) {
    twitter_text_entity_array_free(result.entities);
}

#[no_mangle]
pub extern "C" fn twitter_text_mention_result_free(result: CMentionResult) {
    if !result.mention.is_null() {
        twitter_text_entity_free(result.mention);
    }
}

/* ============================================================================
 * Validating Extractor API (with parse results)
 * ========================================================================= */

use twitter_text::extractor::{Extract, ValidatingExtractor};

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_new(
    config: *mut Configuration,
) -> *mut ValidatingExtractor<'static> {
    if config.is_null() {
        return std::ptr::null_mut();
    }

    // Leak the config to give it 'static lifetime
    let config_ref: &'static Configuration = unsafe { &*(config as *const Configuration) };
    Box::into_raw(Box::new(ValidatingExtractor::new(config_ref)))
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_free(extractor: *mut ValidatingExtractor) {
    if !extractor.is_null() {
        unsafe {
            let _ = Box::from_raw(extractor);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_set_extract_url_without_protocol(
    extractor: *mut ValidatingExtractor,
    extract_url_without_protocol: bool,
) {
    if !extractor.is_null() {
        unsafe {
            (*extractor).set_extract_url_without_protocol(extract_url_without_protocol);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_get_extract_url_without_protocol(
    extractor: *mut ValidatingExtractor,
) -> bool {
    if extractor.is_null() {
        return false;
    }
    unsafe { (*extractor).get_extract_url_without_protocol() }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_prep_input(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> *mut c_char {
    if extractor.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    let extractor_ref = unsafe { &mut *extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let normalized = extractor_ref.prep_input(text_str);
    CString::new(normalized).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_extract_urls_with_indices(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> CExtractResult {
    if extractor.is_null() || text.is_null() {
        return CExtractResult {
            parse_results: twitter_text::TwitterTextParseResults::empty().into(),
            entities: CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            },
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CExtractResult {
                parse_results: twitter_text::TwitterTextParseResults::empty().into(),
                entities: CEntityArray {
                    entities: std::ptr::null_mut(),
                    length: 0,
                },
            }
        }
    };

    let result = extractor_ref.extract_urls_with_indices(text_str);
    let length = result.entities.len();

    let entities = if length == 0 {
        CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        }
    } else {
        let mut c_entities: Vec<CEntity> = result.entities.into_iter().map(|e| e.into()).collect();
        let entities_ptr = c_entities.as_mut_ptr();
        std::mem::forget(c_entities);
        CEntityArray {
            entities: entities_ptr,
            length,
        }
    };

    CExtractResult {
        parse_results: result.parse_results.into(),
        entities,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_extract_hashtags_with_indices(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> CExtractResult {
    if extractor.is_null() || text.is_null() {
        return CExtractResult {
            parse_results: twitter_text::TwitterTextParseResults::empty().into(),
            entities: CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            },
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CExtractResult {
                parse_results: twitter_text::TwitterTextParseResults::empty().into(),
                entities: CEntityArray {
                    entities: std::ptr::null_mut(),
                    length: 0,
                },
            }
        }
    };

    let result = extractor_ref.extract_hashtags_with_indices(text_str);
    let length = result.entities.len();

    let entities = if length == 0 {
        CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        }
    } else {
        let mut c_entities: Vec<CEntity> = result.entities.into_iter().map(|e| e.into()).collect();
        let entities_ptr = c_entities.as_mut_ptr();
        std::mem::forget(c_entities);
        CEntityArray {
            entities: entities_ptr,
            length,
        }
    };

    CExtractResult {
        parse_results: result.parse_results.into(),
        entities,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_extract_cashtags_with_indices(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> CExtractResult {
    if extractor.is_null() || text.is_null() {
        return CExtractResult {
            parse_results: twitter_text::TwitterTextParseResults::empty().into(),
            entities: CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            },
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CExtractResult {
                parse_results: twitter_text::TwitterTextParseResults::empty().into(),
                entities: CEntityArray {
                    entities: std::ptr::null_mut(),
                    length: 0,
                },
            }
        }
    };

    let result = extractor_ref.extract_cashtags_with_indices(text_str);
    let length = result.entities.len();

    let entities = if length == 0 {
        CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        }
    } else {
        let mut c_entities: Vec<CEntity> = result.entities.into_iter().map(|e| e.into()).collect();
        let entities_ptr = c_entities.as_mut_ptr();
        std::mem::forget(c_entities);
        CEntityArray {
            entities: entities_ptr,
            length,
        }
    };

    CExtractResult {
        parse_results: result.parse_results.into(),
        entities,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_extract_mentioned_screennames_with_indices(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> CExtractResult {
    if extractor.is_null() || text.is_null() {
        return CExtractResult {
            parse_results: twitter_text::TwitterTextParseResults::empty().into(),
            entities: CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            },
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CExtractResult {
                parse_results: twitter_text::TwitterTextParseResults::empty().into(),
                entities: CEntityArray {
                    entities: std::ptr::null_mut(),
                    length: 0,
                },
            }
        }
    };

    let result = extractor_ref.extract_mentioned_screennames_with_indices(text_str);
    let length = result.entities.len();

    let entities = if length == 0 {
        CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        }
    } else {
        let mut c_entities: Vec<CEntity> = result.entities.into_iter().map(|e| e.into()).collect();
        let entities_ptr = c_entities.as_mut_ptr();
        std::mem::forget(c_entities);
        CEntityArray {
            entities: entities_ptr,
            length,
        }
    };

    CExtractResult {
        parse_results: result.parse_results.into(),
        entities,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_extract_entities_with_indices(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> CExtractResult {
    if extractor.is_null() || text.is_null() {
        return CExtractResult {
            parse_results: twitter_text::TwitterTextParseResults::empty().into(),
            entities: CEntityArray {
                entities: std::ptr::null_mut(),
                length: 0,
            },
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CExtractResult {
                parse_results: twitter_text::TwitterTextParseResults::empty().into(),
                entities: CEntityArray {
                    entities: std::ptr::null_mut(),
                    length: 0,
                },
            }
        }
    };

    let result = extractor_ref.extract_entities_with_indices(text_str);
    let length = result.entities.len();

    let entities = if length == 0 {
        CEntityArray {
            entities: std::ptr::null_mut(),
            length: 0,
        }
    } else {
        let mut c_entities: Vec<CEntity> = result.entities.into_iter().map(|e| e.into()).collect();
        let entities_ptr = c_entities.as_mut_ptr();
        std::mem::forget(c_entities);
        CEntityArray {
            entities: entities_ptr,
            length,
        }
    };

    CExtractResult {
        parse_results: result.parse_results.into(),
        entities,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validating_extractor_extract_reply_username(
    extractor: *mut ValidatingExtractor,
    text: *const c_char,
) -> CMentionResult {
    if extractor.is_null() || text.is_null() {
        return CMentionResult {
            parse_results: twitter_text::TwitterTextParseResults::empty().into(),
            mention: std::ptr::null_mut(),
        };
    }

    let extractor_ref = unsafe { &*extractor };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return CMentionResult {
                parse_results: twitter_text::TwitterTextParseResults::empty().into(),
                mention: std::ptr::null_mut(),
            }
        }
    };

    let result = extractor_ref.extract_reply_username(text_str);

    let mention = match result.mention {
        Some(entity) => {
            let c_entity: CEntity = entity.into();
            Box::into_raw(Box::new(c_entity))
        }
        None => std::ptr::null_mut(),
    };

    CMentionResult {
        parse_results: result.parse_results.into(),
        mention,
    }
}
