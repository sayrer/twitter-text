#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct TwitterTextAutolinker TwitterTextAutolinker;

/* Autolinker API */
TwitterTextAutolinker* twitter_text_autolinker_new(bool no_follow);
void twitter_text_autolinker_free(TwitterTextAutolinker* autolinker);

/* Configuration setters - customize the autolinker behavior */
void twitter_text_autolinker_set_url_class(
    TwitterTextAutolinker* autolinker,
    const char* url_class
);

void twitter_text_autolinker_set_url_target(
    TwitterTextAutolinker* autolinker,
    const char* url_target
);

void twitter_text_autolinker_set_symbol_tag(
    TwitterTextAutolinker* autolinker,
    const char* symbol_tag
);

void twitter_text_autolinker_set_text_with_symbol_tag(
    TwitterTextAutolinker* autolinker,
    const char* text_with_symbol_tag
);

void twitter_text_autolinker_set_list_class(
    TwitterTextAutolinker* autolinker,
    const char* list_class
);

void twitter_text_autolinker_set_username_class(
    TwitterTextAutolinker* autolinker,
    const char* username_class
);

void twitter_text_autolinker_set_hashtag_class(
    TwitterTextAutolinker* autolinker,
    const char* hashtag_class
);

void twitter_text_autolinker_set_cashtag_class(
    TwitterTextAutolinker* autolinker,
    const char* cashtag_class
);

void twitter_text_autolinker_set_username_url_base(
    TwitterTextAutolinker* autolinker,
    const char* username_url_base
);

void twitter_text_autolinker_set_list_url_base(
    TwitterTextAutolinker* autolinker,
    const char* list_url_base
);

void twitter_text_autolinker_set_hashtag_url_base(
    TwitterTextAutolinker* autolinker,
    const char* hashtag_url_base
);

void twitter_text_autolinker_set_cashtag_url_base(
    TwitterTextAutolinker* autolinker,
    const char* cashtag_url_base
);

void twitter_text_autolinker_set_invisible_tag_attrs(
    TwitterTextAutolinker* autolinker,
    const char* invisible_tag_attrs
);

void twitter_text_autolinker_set_username_include_symbol(
    TwitterTextAutolinker* autolinker,
    bool username_include_symbol
);

void twitter_text_autolinker_set_no_follow(
    TwitterTextAutolinker* autolinker,
    bool no_follow
);

/* Autolinking functions - all return HTML strings that must be freed */

/* Auto-link all entities (URLs, hashtags, mentions, cashtags) */
char* twitter_text_autolinker_autolink(
    TwitterTextAutolinker* autolinker,
    const char* text
);

/* Auto-link only @username and @username/list references */
char* twitter_text_autolinker_autolink_usernames_and_lists(
    TwitterTextAutolinker* autolinker,
    const char* text
);

/* Auto-link only #hashtag references */
char* twitter_text_autolinker_autolink_hashtags(
    TwitterTextAutolinker* autolinker,
    const char* text
);

/* Auto-link only URLs (with protocol) */
char* twitter_text_autolinker_autolink_urls(
    TwitterTextAutolinker* autolinker,
    const char* text
);

/* Auto-link only $cashtag references */
char* twitter_text_autolinker_autolink_cashtags(
    TwitterTextAutolinker* autolinker,
    const char* text
);

/* Free function for returned strings */
void twitter_text_string_free(char* str);

/* Link Attribute Modifiers */
typedef struct TwitterTextAddAttributeModifier TwitterTextAddAttributeModifier;
typedef struct TwitterTextReplaceClassModifier TwitterTextReplaceClassModifier;

/* Entity types for modifier filtering */
typedef enum {
    TWITTER_TEXT_ENTITY_URL = 0,
    TWITTER_TEXT_ENTITY_HASHTAG = 1,
    TWITTER_TEXT_ENTITY_MENTION = 2,
    TWITTER_TEXT_ENTITY_CASHTAG = 3,
} TwitterTextEntityType;

/* Create an AddAttributeModifier that adds a custom attribute to specific entity types */
TwitterTextAddAttributeModifier* twitter_text_add_attribute_modifier_new(
    const TwitterTextEntityType* entity_types,
    size_t entity_types_count,
    const char* key,
    const char* value
);

void twitter_text_add_attribute_modifier_free(TwitterTextAddAttributeModifier* modifier);

/* Create a ReplaceClassModifier that replaces the class attribute */
TwitterTextReplaceClassModifier* twitter_text_replace_class_modifier_new(const char* new_class);

void twitter_text_replace_class_modifier_free(TwitterTextReplaceClassModifier* modifier);

/* Set a modifier on an autolinker */
void twitter_text_autolinker_set_add_attribute_modifier(
    TwitterTextAutolinker* autolinker,
    TwitterTextAddAttributeModifier* modifier
);

void twitter_text_autolinker_set_replace_class_modifier(
    TwitterTextAutolinker* autolinker,
    TwitterTextReplaceClassModifier* modifier
);

/* Default constants */
extern const char* TWITTER_TEXT_DEFAULT_LIST_CLASS;
extern const char* TWITTER_TEXT_DEFAULT_USERNAME_CLASS;
extern const char* TWITTER_TEXT_DEFAULT_HASHTAG_CLASS;
extern const char* TWITTER_TEXT_DEFAULT_CASHTAG_CLASS;
extern const char* TWITTER_TEXT_DEFAULT_USERNAME_URL_BASE;
extern const char* TWITTER_TEXT_DEFAULT_LIST_URL_BASE;
extern const char* TWITTER_TEXT_DEFAULT_HASHTAG_URL_BASE;
extern const char* TWITTER_TEXT_DEFAULT_CASHTAG_URL_BASE;
extern const char* TWITTER_TEXT_DEFAULT_INVISIBLE_TAG_ATTRS;

#ifdef __cplusplus
}
#endif
