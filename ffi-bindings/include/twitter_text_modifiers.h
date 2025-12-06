#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations for types defined in other bindings */
typedef struct TwitterTextAutolinker TwitterTextAutolinker;

/* Link Attribute Modifiers */
typedef struct TwitterTextAddAttributeModifier TwitterTextAddAttributeModifier;
typedef struct TwitterTextReplaceClassModifier TwitterTextReplaceClassModifier;

/* Entity types for modifier filtering */
#ifndef TWITTER_TEXT_ENTITY_TYPE_DEFINED
#define TWITTER_TEXT_ENTITY_TYPE_DEFINED
typedef enum {
    TWITTER_TEXT_ENTITY_URL = 0,
    TWITTER_TEXT_ENTITY_HASHTAG = 1,
    TWITTER_TEXT_ENTITY_MENTION = 2,
    TWITTER_TEXT_ENTITY_CASHTAG = 3,
} TwitterTextEntityType;
#endif

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

/* Set a modifier on an autolinker (autolinker must be created by the specific binding) */
void twitter_text_autolinker_set_add_attribute_modifier(
    TwitterTextAutolinker* autolinker,
    TwitterTextAddAttributeModifier* modifier
);

void twitter_text_autolinker_set_replace_class_modifier(
    TwitterTextAutolinker* autolinker,
    TwitterTextReplaceClassModifier* modifier
);

/* Link Text Modifier - Callback-based API */

/* Entity structure for callbacks */
typedef struct {
    TwitterTextEntityType entity_type;
    int32_t start;
    int32_t end;
} TwitterTextCEntity;

/* Callback function type for modifying link text
 * Returns a new C string that must be freed by the caller using twitter_text_string_free
 */
typedef char* (*TwitterTextLinkTextModifierCallback)(
    const TwitterTextCEntity* entity,
    const char* text,
    void* user_data
);

/* Set a link text modifier callback on an autolinker */
void twitter_text_autolinker_set_link_text_modifier(
    TwitterTextAutolinker* autolinker,
    TwitterTextLinkTextModifierCallback callback,
    void* user_data
);

#ifdef __cplusplus
}
#endif
