use std::ffi::CStr;
use std::os::raw::c_char;
use twitter_text::validator::Validator;
use twitter_text_config::Configuration;

/* ============================================================================
 * Validator API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_validator_new() -> *mut Validator {
    Box::into_raw(Box::new(Validator::new()))
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_with_config(
    config: *const Configuration,
) -> *mut Validator {
    if config.is_null() {
        return Box::into_raw(Box::new(Validator::new()));
    }

    let config_ref = unsafe { &*config };
    Box::into_raw(Box::new(Validator::with_config(config_ref.clone())))
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_free(validator: *mut Validator) {
    if !validator.is_null() {
        unsafe {
            let _ = Box::from_raw(validator);
        }
    }
}

/* ============================================================================
 * Validation functions
 * ========================================================================= */

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
    match c_str.to_str() {
        Ok(s) => validator_ref.is_valid_tweet(s),
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_is_valid_username(
    validator: *mut Validator,
    username: *const c_char,
) -> bool {
    if validator.is_null() || username.is_null() {
        return false;
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(username) };
    match c_str.to_str() {
        Ok(s) => validator_ref.is_valid_username(s),
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_is_valid_list(
    validator: *mut Validator,
    list: *const c_char,
) -> bool {
    if validator.is_null() || list.is_null() {
        return false;
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(list) };
    match c_str.to_str() {
        Ok(s) => validator_ref.is_valid_list(s),
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_is_valid_hashtag(
    validator: *mut Validator,
    hashtag: *const c_char,
) -> bool {
    if validator.is_null() || hashtag.is_null() {
        return false;
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(hashtag) };
    match c_str.to_str() {
        Ok(s) => validator_ref.is_valid_hashtag(s),
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_is_valid_url(
    validator: *mut Validator,
    url: *const c_char,
) -> bool {
    if validator.is_null() || url.is_null() {
        return false;
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(url) };
    match c_str.to_str() {
        Ok(s) => validator_ref.is_valid_url(s),
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_is_valid_url_without_protocol(
    validator: *mut Validator,
    url: *const c_char,
) -> bool {
    if validator.is_null() || url.is_null() {
        return false;
    }

    let validator_ref = unsafe { &*validator };
    let c_str = unsafe { CStr::from_ptr(url) };
    match c_str.to_str() {
        Ok(s) => validator_ref.is_valid_url_without_protocol(s),
        Err(_) => false,
    }
}

/* ============================================================================
 * Getters
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_validator_get_max_tweet_length(validator: *mut Validator) -> i32 {
    if validator.is_null() {
        return 0;
    }

    let validator_ref = unsafe { &*validator };
    validator_ref.get_max_tweet_length()
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_get_short_url_length(validator: *mut Validator) -> i32 {
    if validator.is_null() {
        return 0;
    }

    let validator_ref = unsafe { &*validator };
    validator_ref.get_short_url_length()
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_get_short_url_length_https(
    validator: *mut Validator,
) -> i32 {
    if validator.is_null() {
        return 0;
    }

    let validator_ref = unsafe { &*validator };
    validator_ref.get_short_url_length_https()
}

/* ============================================================================
 * Setters
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_validator_set_short_url_length(
    validator: *mut Validator,
    length: i32,
) {
    if !validator.is_null() {
        let validator_ref = unsafe { &mut *validator };
        validator_ref.set_short_url_length(length);
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_validator_set_short_url_length_https(
    validator: *mut Validator,
    length: i32,
) {
    if !validator.is_null() {
        let validator_ref = unsafe { &mut *validator };
        validator_ref.set_short_url_length_https(length);
    }
}

/* ============================================================================
 * Default constant
 * ========================================================================= */

#[no_mangle]
pub static TWITTER_TEXT_MAX_TWEET_LENGTH: i32 = twitter_text::validator::MAX_TWEET_LENGTH;
