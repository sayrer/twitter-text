#pragma once

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct TwitterTextHitHighlighter TwitterTextHitHighlighter;

/* Hit struct - represents a range to highlight (start, end) */
typedef struct {
    size_t start;
    size_t end;
} TwitterTextHit;

/* Array of hits */
typedef struct {
    TwitterTextHit* hits;
    size_t length;
} TwitterTextHitArray;

/* HitHighlighter API */
TwitterTextHitHighlighter* twitter_text_hit_highlighter_new(void);
TwitterTextHitHighlighter* twitter_text_hit_highlighter_new_with_tag(const char* highlight_tag);
void twitter_text_hit_highlighter_free(TwitterTextHitHighlighter* highlighter);

/* Set the highlight tag (default is "em") */
void twitter_text_hit_highlighter_set_tag(
    TwitterTextHitHighlighter* highlighter,
    const char* highlight_tag
);

/* Highlight the given text with the provided hit ranges
 * Returns HTML string with highlight tags around the hit ranges
 * The returned string must be freed with twitter_text_string_free()
 */
char* twitter_text_hit_highlighter_highlight(
    TwitterTextHitHighlighter* highlighter,
    const char* text,
    const TwitterTextHit* hits,
    size_t hits_length
);

/* Free function for returned strings */
void twitter_text_string_free(char* str);

/* Default constant */
extern const char* TWITTER_TEXT_DEFAULT_HIGHLIGHT_TAG;
