%module twitter_text
%{
/* Includes the header in the wrapper code */
#include "rust/twitter-text/twitter-text.h"
#include "rust_bindings/cpp/twitter.h"
%}

%include <stdint.i>
%include <std_string.i>
%include <std_vector.i>

%rename("%(undercase)s", %$isfunction) "";

namespace std {
    %template(WeightedRangeList) vector<twitter_text::WeightedRange>;
}

namespace rust {
    class String;
    %typemap(out) String {
       $result = PyUnicode_FromStringAndSize($1.data(), $1.size());
    }

}

%ignore Box;

%ignore config_v1;
%ignore config_v2;
%ignore config_v3;
%ignore default_config;
%ignore get_config_weighted_ranges;
%ignore configuration_from_path;
%ignore configuration_from_json;
%ignore autolink_default_config;
%ignore autolink_all;
%ignore make_extractor;
%ignore extract_entities_with_indices;
%ignore extract_mentioned_screennames;
%ignore extract_mentioned_screennames_with_indices;
%ignore extract_mentions_or_lists_with_indices;
%ignore extract_reply_username;
%ignore extract_urls;
%ignore extract_urls_with_indices;
%ignore extract_hashtags;
%ignore extract_hashtags_with_indices;
%ignore extract_cashtags;
%ignore extract_cashtags_with_indices;
%ignore make_validating_extractor;
%ignore extract_entities_with_indices_validated;
%ignore extract_mentioned_screennames_with_indices_validated;
%ignore extract_mentions_or_lists_with_indices_validated;
%ignore extract_reply_username_validated;
%ignore extract_urls_with_indices_validated;
%ignore extract_hashtags_with_indices_validated;
%ignore extract_cashtags_with_indices_validated;
%ignore make_default_highlighter;
%ignore make_highlighter;
%ignore make_default_validator;

%ignore AutolinkerConfig;
%ignore Configuration;
%ignore Entity;
%ignore Extractor;
%ignore ExtractResult;
%ignore ExtractorString;
%ignore MentionResult;
%ignore ValidatingExtractor;
%ignore HitHighlighter;
%ignore TwitterTextParser;
%ignore TwitterTextParseResults;
%ignore Validator;
%ignore Vec<twitter_text::Entity>;
%ignore Vec<twitter_text::ffi::Entity>;


/* Parse the header file to generate wrappers */
%include "rust/twitter-text/twitter-text.h"
%include "rust_bindings/cpp/twitter.h"