/* C API for twitter-text library - for Java FFM (Panama) bindings */

#pragma once

#include <stdint.h>
#include <stdbool.h>

/* Include all module headers */
#include "configuration.h"
#include "autolink.h"
#include "validator.h"
#include "extractor.h"
#include "hit_highlighter.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Main Parse API (from lib.rs)
 * ========================================================================= */

/* Parse tweet text and return validation results
 *
 * This is the main entry point for parsing tweet text. It returns a
 * TwitterTextParseResults struct containing:
 * - weighted_length: The weighted character count (most chars = 2, ASCII/Latin-1 = 1)
 * - permillage: The length as a fraction of 1000 (for progress meters)
 * - is_valid: Whether the tweet is valid
 * - display_text_range: The range to display
 * - valid_text_range: The range that is valid
 *
 * Parameters:
 * - text: The tweet text to parse
 * - config: Configuration object (use twitter_text_config_default() for defaults)
 * - extract_urls: If true, URLs are weighted using config's transformed_url_length
 */
TwitterTextParseResults twitter_text_parse(
    const char* text,
    const TwitterTextConfiguration* config,
    bool extract_urls
);

/* ============================================================================
 * Utility functions
 * ========================================================================= */

/* Free a string returned by the library */
void twitter_text_string_free(char* str);

#ifdef __cplusplus
}
#endif
