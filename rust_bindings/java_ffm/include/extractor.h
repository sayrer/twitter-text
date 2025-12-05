#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#include "configuration.h"

typedef struct TwitterTextExtractor TwitterTextExtractor;
typedef struct TwitterTextValidatingExtractor TwitterTextValidatingExtractor;

/* Entity struct - matches Rust Entity */
typedef struct {
    int32_t entity_type;
    int32_t start;
    int32_t end;
    char* value;
    char* list_slug;
    char* display_url;
    char* expanded_url;
} TwitterTextEntity;

/* Array of entities - for returning multiple results */
typedef struct {
    TwitterTextEntity* entities;
    size_t length;
} TwitterTextEntityArray;

/* Array of strings - for simple extractions */
typedef struct {
    char** strings;
    size_t length;
} TwitterTextStringArray;

/* Extract result - entities with validation data */
typedef struct {
    TwitterTextParseResults parse_results;
    TwitterTextEntityArray entities;
} TwitterTextExtractResult;

/* Mention result - single entity with validation data */
typedef struct {
    TwitterTextParseResults parse_results;
    TwitterTextEntity* mention;  /* NULL if no mention found */
} TwitterTextMentionResult;

/* ============================================================================
 * Basic Extractor API (no validation)
 * ========================================================================= */

TwitterTextExtractor* twitter_text_extractor_new(void);
void twitter_text_extractor_free(TwitterTextExtractor* extractor);

/* Set whether to extract URLs without protocol (default: true) */
void twitter_text_extractor_set_extract_url_without_protocol(
    TwitterTextExtractor* extractor,
    bool extract_url_without_protocol
);

bool twitter_text_extractor_get_extract_url_without_protocol(
    TwitterTextExtractor* extractor
);

/* Extract URLs as simple strings */
TwitterTextStringArray twitter_text_extractor_extract_urls(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract URLs with indices and metadata */
TwitterTextEntityArray twitter_text_extractor_extract_urls_with_indices(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract hashtags as simple strings */
TwitterTextStringArray twitter_text_extractor_extract_hashtags(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract hashtags with indices */
TwitterTextEntityArray twitter_text_extractor_extract_hashtags_with_indices(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract cashtags as simple strings */
TwitterTextStringArray twitter_text_extractor_extract_cashtags(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract cashtags with indices */
TwitterTextEntityArray twitter_text_extractor_extract_cashtags_with_indices(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract mentioned screennames */
TwitterTextStringArray twitter_text_extractor_extract_mentioned_screennames(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract mentioned screennames with indices */
TwitterTextEntityArray twitter_text_extractor_extract_mentioned_screennames_with_indices(
    TwitterTextExtractor* extractor,
    const char* text
);

/* Extract reply username - returns NULL if none found */
TwitterTextEntity* twitter_text_extractor_extract_reply_username(
    TwitterTextExtractor* extractor,
    const char* text
);

/* ============================================================================
 * Validating Extractor API (with parse results)
 * ========================================================================= */

TwitterTextValidatingExtractor* twitter_text_validating_extractor_new(
    TwitterTextConfiguration* config
);

void twitter_text_validating_extractor_free(
    TwitterTextValidatingExtractor* extractor
);

/* Set whether to extract URLs without protocol (default: true) */
void twitter_text_validating_extractor_set_extract_url_without_protocol(
    TwitterTextValidatingExtractor* extractor,
    bool extract_url_without_protocol
);

bool twitter_text_validating_extractor_get_extract_url_without_protocol(
    TwitterTextValidatingExtractor* extractor
);

/* Prepare input text (normalizes to NFC) - returns normalized text that must be freed */
char* twitter_text_validating_extractor_prep_input(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* Extract URLs with validation data */
TwitterTextExtractResult twitter_text_validating_extractor_extract_urls_with_indices(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* Extract hashtags with validation data */
TwitterTextExtractResult twitter_text_validating_extractor_extract_hashtags_with_indices(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* Extract cashtags with validation data */
TwitterTextExtractResult twitter_text_validating_extractor_extract_cashtags_with_indices(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* Extract mentioned screennames with validation data */
TwitterTextExtractResult twitter_text_validating_extractor_extract_mentioned_screennames_with_indices(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* Extract all entities with validation data */
TwitterTextExtractResult twitter_text_validating_extractor_extract_entities_with_indices(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* Extract reply username with validation data */
TwitterTextMentionResult twitter_text_validating_extractor_extract_reply_username(
    TwitterTextValidatingExtractor* extractor,
    const char* text
);

/* ============================================================================
 * Free functions for complex types
 * ========================================================================= */

void twitter_text_entity_free(TwitterTextEntity* entity);
void twitter_text_entity_array_free(TwitterTextEntityArray array);
void twitter_text_string_array_free(TwitterTextStringArray array);
void twitter_text_extract_result_free(TwitterTextExtractResult result);
void twitter_text_mention_result_free(TwitterTextMentionResult result);
