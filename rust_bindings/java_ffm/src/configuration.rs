/* ============================================================================
 * C-compatible Configuration types
 * ========================================================================= */

#[repr(C)]
pub struct CWeightedRange {
    range: CRange,
    weight: i32,
}

impl From<twitter_text_config::WeightedRange> for CWeightedRange {
    fn from(wr: twitter_text_config::WeightedRange) -> Self {
        CWeightedRange {
            range: wr.range.into(),
            weight: wr.weight,
        }
    }
}

impl From<CWeightedRange> for twitter_text_config::WeightedRange {
    fn from(cwr: CWeightedRange) -> Self {
        twitter_text_config::WeightedRange::new(cwr.range.start, cwr.range.end, cwr.weight)
    }
}

#[repr(C)]
pub struct CWeightedRangeArray {
    ranges: *mut CWeightedRange,
    length: usize,
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
) -> CWeightedRangeArray {
    if config.is_null() {
        return CWeightedRangeArray {
            ranges: std::ptr::null_mut(),
            length: 0,
        };
    }

    let config_ref = unsafe { &*config };
    let ranges = &config_ref.ranges;
    let length = ranges.len();

    if length == 0 {
        return CWeightedRangeArray {
            ranges: std::ptr::null_mut(),
            length: 0,
        };
    }

    let mut c_ranges: Vec<CWeightedRange> = ranges.iter().map(|r| r.clone().into()).collect();

    let ranges_ptr = c_ranges.as_mut_ptr();
    std::mem::forget(c_ranges);

    CWeightedRangeArray {
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
    ranges: *mut CWeightedRange,
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
pub extern "C" fn twitter_text_weighted_range_array_free(array: CWeightedRangeArray) {
    if !array.ranges.is_null() && array.length > 0 {
        unsafe {
            let _ = Vec::from_raw_parts(array.ranges, array.length, array.length);
        }
    }
}
