/* C API for twitter-text library - for Java FFM (Panama) bindings */

#pragma once

#include <stdint.h>
#include <stdbool.h>

#include "configuration.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque types - Java FFM will use MemorySegment to handle these */
typedef struct TwitterTextAutolinker TwitterTextAutolinker;
typedef struct TwitterTextValidator TwitterTextValidator;



/* Validator API */
TwitterTextValidator* twitter_text_validator_new(TwitterTextConfiguration* config);
void twitter_text_validator_free(TwitterTextValidator* validator);
TwitterTextParseResults twitter_text_validator_parse_tweet(
    TwitterTextValidator* validator,
    const char* text
);
bool twitter_text_validator_is_valid_tweet(
    TwitterTextValidator* validator,
    const char* text
);

/* Autolinker API */
TwitterTextAutolinker* twitter_text_autolinker_new(TwitterTextConfiguration* config);
void twitter_text_autolinker_free(TwitterTextAutolinker* autolinker);
/* Returns malloc'd string - caller must free with twitter_text_string_free */
char* twitter_text_autolinker_autolink(
    TwitterTextAutolinker* autolinker,
    const char* text
);
/* TODO: Add methods for autolink_with_json, etc. */


/* Utility functions */
void twitter_text_string_free(char* str);

#ifdef __cplusplus
}
#endif
