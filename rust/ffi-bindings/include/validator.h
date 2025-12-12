#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct TwitterTextValidator TwitterTextValidator;
typedef struct TwitterTextConfiguration TwitterTextConfiguration;

/* Validator API */
TwitterTextValidator* twitter_text_validator_new(void);
TwitterTextValidator* twitter_text_validator_with_config(const TwitterTextConfiguration* config);
void twitter_text_validator_free(TwitterTextValidator* validator);

/* Validation functions - return true if valid, false otherwise */
bool twitter_text_validator_is_valid_tweet(
    TwitterTextValidator* validator,
    const char* text
);

bool twitter_text_validator_is_valid_username(
    TwitterTextValidator* validator,
    const char* username
);

bool twitter_text_validator_is_valid_list(
    TwitterTextValidator* validator,
    const char* list
);

bool twitter_text_validator_is_valid_hashtag(
    TwitterTextValidator* validator,
    const char* hashtag
);

bool twitter_text_validator_is_valid_url(
    TwitterTextValidator* validator,
    const char* url
);

bool twitter_text_validator_is_valid_url_without_protocol(
    TwitterTextValidator* validator,
    const char* url
);

/* Getters */
int32_t twitter_text_validator_get_max_tweet_length(
    TwitterTextValidator* validator
);

int32_t twitter_text_validator_get_short_url_length(
    TwitterTextValidator* validator
);

int32_t twitter_text_validator_get_short_url_length_https(
    TwitterTextValidator* validator
);

/* Setters */
void twitter_text_validator_set_short_url_length(
    TwitterTextValidator* validator,
    int32_t length
);

void twitter_text_validator_set_short_url_length_https(
    TwitterTextValidator* validator,
    int32_t length
);

/* Default constant */
extern const int32_t TWITTER_TEXT_MAX_TWEET_LENGTH;
