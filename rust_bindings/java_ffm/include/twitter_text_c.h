/* C API for twitter-text library - for Java FFM (Panama) bindings */

#ifndef TWITTER_TEXT_C_H
#define TWITTER_TEXT_C_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque types - Java FFM will use MemorySegment to handle these */
typedef struct TwitterTextConfiguration TwitterTextConfiguration;
typedef struct TwitterTextAutolinker TwitterTextAutolinker;
typedef struct TwitterTextExtractor TwitterTextExtractor;
typedef struct TwitterTextValidator TwitterTextValidator;

/* Range struct - matches Rust Range */
typedef struct {
    int32_t start;
    int32_t end;
} TwitterTextRange;

/* Parse results - matches Rust TwitterTextParseResults */
typedef struct {
    int32_t weighted_length;
    int32_t permillage;
    bool is_valid;
    TwitterTextRange display_text_range;
    TwitterTextRange valid_text_range;
} TwitterTextParseResults;

/* Configuration API */
TwitterTextConfiguration* twitter_text_config_default(void);
TwitterTextConfiguration* twitter_text_config_from_json(const char* json);
void twitter_text_config_free(TwitterTextConfiguration* config);

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

/* Extractor API */
TwitterTextExtractor* twitter_text_extractor_new(TwitterTextConfiguration* config);
void twitter_text_extractor_free(TwitterTextExtractor* extractor);
/* TODO: Add methods for extract_urls, extract_hashtags, etc.
 * These will need to return arrays/vectors - consider returning JSON strings
 * for simplicity, or use callback pattern */

/* Utility functions */
void twitter_text_string_free(char* str);

#ifdef __cplusplus
}
#endif

#endif /* TWITTER_TEXT_C_H */
