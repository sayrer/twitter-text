#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct TwitterTextConfiguration TwitterTextConfiguration;

TwitterTextConfiguration* twitter_text_config_default(void);
TwitterTextConfiguration* twitter_text_config_v3(void);
TwitterTextConfiguration* twitter_text_config_from_json(const char* json);
void twitter_text_config_free(TwitterTextConfiguration* config);

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

/* WeightedRange struct - for Configuration */
typedef struct {
    TwitterTextRange range;
    int32_t weight;
} TwitterTextWeightedRange;

/* WeightedRange array */
typedef struct {
    TwitterTextWeightedRange* ranges;
    size_t length;
} TwitterTextWeightedRangeArray;

/* Configuration getters - since Configuration is opaque, provide accessors */
int32_t twitter_text_config_get_version(TwitterTextConfiguration* config);
int32_t twitter_text_config_get_max_weighted_tweet_length(TwitterTextConfiguration* config);
int32_t twitter_text_config_get_scale(TwitterTextConfiguration* config);
int32_t twitter_text_config_get_default_weight(TwitterTextConfiguration* config);
int32_t twitter_text_config_get_transformed_url_length(TwitterTextConfiguration* config);
bool twitter_text_config_get_emoji_parsing_enabled(TwitterTextConfiguration* config);
TwitterTextWeightedRangeArray twitter_text_config_get_ranges(TwitterTextConfiguration* config);

/* Configuration setters - for building configurations programmatically */
void twitter_text_config_set_version(TwitterTextConfiguration* config, int32_t version);
void twitter_text_config_set_max_weighted_tweet_length(TwitterTextConfiguration* config, int32_t length);
void twitter_text_config_set_scale(TwitterTextConfiguration* config, int32_t scale);
void twitter_text_config_set_default_weight(TwitterTextConfiguration* config, int32_t weight);
void twitter_text_config_set_transformed_url_length(TwitterTextConfiguration* config, int32_t length);
void twitter_text_config_set_emoji_parsing_enabled(TwitterTextConfiguration* config, bool enabled);
void twitter_text_config_set_ranges(
    TwitterTextConfiguration* config,
    TwitterTextWeightedRange* ranges,
    size_t length
);

/* Create a new empty configuration (not initialized with defaults) */
TwitterTextConfiguration* twitter_text_config_new(void);

/* Free function for weighted range array */
void twitter_text_weighted_range_array_free(TwitterTextWeightedRangeArray array);
