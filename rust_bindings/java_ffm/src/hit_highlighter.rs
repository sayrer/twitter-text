use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::hit_highlighter::HitHighlighter;

/* ============================================================================
 * HitHighlighter API
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_hit_highlighter_new() -> *mut HitHighlighter {
    Box::into_raw(Box::new(HitHighlighter::new()))
}

#[no_mangle]
pub extern "C" fn twitter_text_hit_highlighter_new_with_tag(
    highlight_tag: *const c_char,
) -> *mut HitHighlighter {
    if highlight_tag.is_null() {
        return Box::into_raw(Box::new(HitHighlighter::new()));
    }

    let c_str = unsafe { CStr::from_ptr(highlight_tag) };
    match c_str.to_str() {
        Ok(s) => Box::into_raw(Box::new(HitHighlighter::new_with_tag(s))),
        Err(_) => Box::into_raw(Box::new(HitHighlighter::new())),
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_hit_highlighter_free(highlighter: *mut HitHighlighter) {
    if !highlighter.is_null() {
        unsafe {
            let _ = Box::from_raw(highlighter);
        }
    }
}

#[no_mangle]
pub extern "C" fn twitter_text_hit_highlighter_set_tag(
    highlighter: *mut HitHighlighter,
    highlight_tag: *const c_char,
) {
    if highlighter.is_null() || highlight_tag.is_null() {
        return;
    }

    let c_str = unsafe { CStr::from_ptr(highlight_tag) };
    if let Ok(s) = c_str.to_str() {
        unsafe {
            (*highlighter).highlight_tag = s.to_string();
        }
    }
}

/* ============================================================================
 * C-compatible Hit type
 * ========================================================================= */

#[repr(C)]
pub struct CHit {
    start: usize,
    end: usize,
}

/* ============================================================================
 * Highlight function
 * ========================================================================= */

#[no_mangle]
pub extern "C" fn twitter_text_hit_highlighter_highlight(
    highlighter: *mut HitHighlighter,
    text: *const c_char,
    hits: *const CHit,
    hits_length: usize,
) -> *mut c_char {
    if highlighter.is_null() || text.is_null() {
        return std::ptr::null_mut();
    }

    let highlighter_ref = unsafe { &*highlighter };
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let hits_vec = if hits.is_null() || hits_length == 0 {
        Vec::new()
    } else {
        unsafe {
            let hits_slice = std::slice::from_raw_parts(hits, hits_length);
            hits_slice
                .iter()
                .map(|h| (h.start, h.end))
                .collect::<Vec<(usize, usize)>>()
        }
    };

    let result = highlighter_ref.highlight(text_str, hits_vec);
    CString::new(result).unwrap_or_default().into_raw()
}

/* ============================================================================
 * Free function for returned strings
 * ========================================================================= */

// Note: twitter_text_string_free is defined in autolink.rs and shared across modules

/* ============================================================================
 * Default constant
 * ========================================================================= */

#[no_mangle]
pub static TWITTER_TEXT_DEFAULT_HIGHLIGHT_TAG: &str =
    twitter_text::hit_highlighter::DEFAULT_HIGHLIGHT_TAG;
