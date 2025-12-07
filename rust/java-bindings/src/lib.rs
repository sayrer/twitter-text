/* Rust C-ABI exports for Java FFM bindings */

use std::ffi::CStr;
use std::os::raw::c_char;
use twitter_text_config::Configuration;

// Module declarations for the separate C API implementations
pub mod autolink;
pub mod configuration;
pub mod extractor;
pub mod hit_highlighter;
pub mod validator;

// Re-export commonly used types
pub use configuration::*;
pub use extractor::*;

/* ============================================================================
 * Main Parse API (from twitter-text lib.rs)
 * ========================================================================= */

// Re-export the types from configuration module that match the C header
pub use configuration::{TwitterTextParseResults, TwitterTextRange};

/// Parse tweet text and return validation results
///
/// This is the main entry point from twitter_text::parse()
#[no_mangle]
pub extern "C" fn twitter_text_parse(
    text: *const c_char,
    config: *const Configuration,
    extract_urls: bool,
) -> TwitterTextParseResults {
    // Handle null inputs
    if text.is_null() || config.is_null() {
        return TwitterTextParseResults {
            weighted_length: 0,
            permillage: 0,
            is_valid: false,
            display_text_range: TwitterTextRange { start: 0, end: 0 },
            valid_text_range: TwitterTextRange { start: 0, end: 0 },
        };
    }

    // Convert C string to Rust str
    let c_str = unsafe { CStr::from_ptr(text) };
    let text_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            return TwitterTextParseResults {
                weighted_length: 0,
                permillage: 0,
                is_valid: false,
                display_text_range: TwitterTextRange { start: 0, end: 0 },
                valid_text_range: TwitterTextRange { start: 0, end: 0 },
            }
        }
    };

    // Get config reference
    let config_ref = unsafe { &*config };

    // Call the Rust parse function
    let results = twitter_text::parse(text_str, config_ref, extract_urls);

    // Convert to C-compatible representation
    results.into()
}
