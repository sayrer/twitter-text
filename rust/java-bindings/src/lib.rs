/* Rust C-ABI exports for Java FFM bindings */

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use twitter_text::extractor::Extractor;
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
