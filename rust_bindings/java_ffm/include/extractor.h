#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#include "configuration.h"

typedef struct TwitterTextExtractor TwitterTextExtractor;

/* Extractor API */
TwitterTextExtractor* twitter_text_extractor_new(TwitterTextConfiguration* config);
void twitter_text_extractor_free(TwitterTextExtractor* extractor);

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

/* Extractor API */
TwitterTextExtractor* twitter_text_extractor_new(TwitterTextConfiguration* config);
void twitter_text_extractor_free(TwitterTextExtractor* extractor);

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

/* Free functions for complex types */
void twitter_text_entity_free(TwitterTextEntity* entity);
void twitter_text_entity_array_free(TwitterTextEntityArray array);
void twitter_text_string_array_free(TwitterTextStringArray array);
