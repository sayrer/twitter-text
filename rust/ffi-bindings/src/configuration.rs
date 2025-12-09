use std::ffi::CStr;
use std::os::raw::c_char;
use twitter_text_config::{Configuration, Range};

/* ============================================================================
 * C-compatible types matching the C header
 * ========================================================================= */

/// TwitterTextRange - matches the C header typedef
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TwitterTextRange {
    pub start: i32,
    pub end: i32,
}

impl From<Range> for TwitterTextRange {
    fn from(range: Range) -> Self {
        TwitterTextRange {
            start: range.start(),
            end: range.end(),
        }
    }
}

impl From<TwitterTextRange> for Range {
    fn from(tr: TwitterTextRange) -> Self {
        Range::new(tr.start, tr.end)
    }
}

/// TwitterTextParseResults - matches the C header typedef
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TwitterTextParseResults {
    pub weighted_length: i32,
    pub permillage: i32,
    pub is_valid: bool,
    pub display_text_range: TwitterTextRange,
    pub valid_text_range: TwitterTextRange,
}

impl From<twitter_text::TwitterTextParseResults> for TwitterTextParseResults {
    fn from(results: twitter_text::TwitterTextParseResults) -> Self {
        TwitterTextParseResults {
            weighted_length: results.weighted_length,
            permillage: results.permillage,
            is_valid: results.is_valid,
            display_text_range: results.display_text_range.into(),
            valid_text_range: results.valid_text_range.into(),
        }
    }
}

/// TwitterTextWeightedRange - for Configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TwitterTextWeightedRange {
    pub range: TwitterTextRange,
    pub weight: i32,
}

impl From<twitter_text_config::WeightedRange> for TwitterTextWeightedRange {
    fn from(wr: twitter_text_config::WeightedRange) -> Self {
        TwitterTextWeightedRange {
            range: wr.range.into(),
            weight: wr.weight,
        }
    }
}

impl From<TwitterTextWeightedRange> for twitter_text_config::WeightedRange {
    fn from(twr: TwitterTextWeightedRange) -> Self {
        twitter_text_config::WeightedRange::new(twr.range.start, twr.range.end, twr.weight)
    }
}

/// TwitterTextWeightedRangeArray - for Configuration
#[repr(C)]
pub struct TwitterTextWeightedRangeArray {
    pub ranges: *mut TwitterTextWeightedRange,
    pub length: usize,
}

/* ============================================================================
 * Configuration API - Constructors
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_config_new() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::default()))
}

#[no_mangle]
pub extern "C" fn twitter_text_config_default() -> *mut Configuration {
    Box::into_raw(Box::new(Configuration::default()))
}

#[no_mangle]
pub extern "C" fn twitter_text_config_v1() -> *mut Configuration {
    Box::into_raw(Box::new(twitter_text_config::config_v1().clone()))
}

#[no_mangle]
pub extern "C" fn twitter_text_config_v2() -> *mut Configuration {
    Box::into_raw(Box::new(twitter_text_config::config_v2().clone()))
}

#[no_mangle]
pub extern "C" fn twitter_text_config_v3() -> *mut Configuration {
    Box::into_raw(Box::new(twitter_text_config::config_v3().clone()))
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
 * Configuration API - Getters
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_config_get_version(config: *mut Configuration) -> i32 {
    if config.is_null() {
        return 0;
    }
    unsafe { (*config).version }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_get_max_weighted_tweet_length(
    config: *mut Configuration,
) -> i32 {
    if config.is_null() {
        return 0;
    }
    unsafe { (*config).max_weighted_tweet_length }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_get_scale(config: *mut Configuration) -> i32 {
    if config.is_null() {
        return 0;
    }
    unsafe { (*config).scale }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_get_default_weight(config: *mut Configuration) -> i32 {
    if config.is_null() {
        return 0;
    }
    unsafe { (*config).default_weight }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_get_transformed_url_length(
    config: *mut Configuration,
) -> i32 {
    if config.is_null() {
        return 0;
    }
    unsafe { (*config).transformed_url_length }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_get_emoji_parsing_enabled(
    config: *mut Configuration,
) -> bool {
    if config.is_null() {
        return false;
    }
    unsafe { (*config).emoji_parsing_enabled }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_get_ranges(
    config: *mut Configuration,
) -> TwitterTextWeightedRangeArray {
    if config.is_null() {
        return TwitterTextWeightedRangeArray {
            ranges: std::ptr::null_mut(),
            length: 0,
        };
    }

    let config_ref = unsafe { &*config };
    let ranges = &config_ref.ranges;
    let length = ranges.len();

    if length == 0 {
        return TwitterTextWeightedRangeArray {
            ranges: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_ranges: Vec<TwitterTextWeightedRange> =
        ranges.iter().map(|r| r.clone().into()).collect();

    let ranges_ptr = c_ranges.as_mut_ptr();
    std::mem::forget(c_ranges);

    TwitterTextWeightedRangeArray {
        ranges: ranges_ptr,
        length,
    }
}

/* ============================================================================
 * Configuration API - Setters
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_config_set_version(config: *mut Configuration, version: i32) {
    if !config.is_null() {
        unsafe {
            (*config).version = version;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_set_max_weighted_tweet_length(
    config: *mut Configuration,
    length: i32,
) {
    if !config.is_null() {
        unsafe {
            (*config).max_weighted_tweet_length = length;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_set_scale(config: *mut Configuration, scale: i32) {
    if !config.is_null() {
        unsafe {
            (*config).scale = scale;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_set_default_weight(config: *mut Configuration, weight: i32) {
    if !config.is_null() {
        unsafe {
            (*config).default_weight = weight;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_set_transformed_url_length(
    config: *mut Configuration,
    length: i32,
) {
    if !config.is_null() {
        unsafe {
            (*config).transformed_url_length = length;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_set_emoji_parsing_enabled(
    config: *mut Configuration,
    enabled: bool,
) {
    if !config.is_null() {
        unsafe {
            (*config).emoji_parsing_enabled = enabled;
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_config_set_ranges(
    config: *mut Configuration,
    ranges: *mut TwitterTextWeightedRange,
    length: usize,
) {
    if config.is_null() || ranges.is_null() || length == 0 {
        return;
    }

    unsafe {
        let c_ranges = std::slice::from_raw_parts(ranges, length);
        let rust_ranges: Vec<twitter_text_config::WeightedRange> =
            c_ranges.iter().map(|r| (*r).into()).collect();

        (*config).ranges = rust_ranges;
    }
}

/* ============================================================================
 * Free function for weighted range array
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_weighted_range_array_free(array: TwitterTextWeightedRangeArray) {
    if !array.ranges.is_null() && array.length > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(array.ranges, array.length, array.length);
        }
    }
}
