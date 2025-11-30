use magnus::{function, method, prelude::*, Error, Ruby};

mod autolinker;
mod configuration;
mod extractor;
mod hithighlighter;
mod parser;
mod validator;

use autolinker::Autolinker;
use configuration::{RubyRange, TwitterTextConfiguration, WeightedRange};
use extractor::{Entity, ExtractResult, Extractor, MentionResult, ValidatingExtractor};
use hithighlighter::{Hit, HitHighlighter, Hits};
use parser::{ParseRange, TwitterTextParseResult, TwitterTextParser};
use validator::Validator;

#[magnus::init(name = "twittertext")]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Twittertext")?;

    // Register classes
    let extractor_class = module.define_class("Extractor", ruby.class_object())?;
    extractor_class.define_singleton_method("new", function!(Extractor::ruby_new, 0))?;
    extractor_class.define_method(
        "get_extract_url_without_protocol",
        method!(Extractor::get_extract_url_without_protocol, 0),
    )?;
    extractor_class.define_method(
        "set_extract_url_without_protocol",
        method!(Extractor::set_extract_url_without_protocol, 1),
    )?;
    extractor_class.define_method(
        "extract_mentioned_screennames",
        method!(Extractor::extract_mentioned_screennames, 1),
    )?;
    extractor_class.define_method(
        "extract_mentioned_screennames_with_indices",
        method!(Extractor::extract_mentioned_screennames_with_indices, 1),
    )?;
    extractor_class.define_method(
        "extract_mentions_or_lists_with_indices",
        method!(Extractor::extract_mentions_or_lists_with_indices, 1),
    )?;
    extractor_class.define_method(
        "extract_reply_screenname",
        method!(Extractor::extract_reply_screenname, 1),
    )?;
    extractor_class.define_method("extract_urls", method!(Extractor::extract_urls, 1))?;
    extractor_class.define_method(
        "extract_urls_with_indices",
        method!(Extractor::extract_urls_with_indices, 1),
    )?;
    extractor_class.define_method("extract_hashtags", method!(Extractor::extract_hashtags, 1))?;
    extractor_class.define_method(
        "extract_hashtags_with_indices",
        method!(Extractor::extract_hashtags_with_indices, 1),
    )?;
    extractor_class.define_method("extract_cashtags", method!(Extractor::extract_cashtags, 1))?;
    extractor_class.define_method(
        "extract_cashtags_with_indices",
        method!(Extractor::extract_cashtags_with_indices, 1),
    )?;
    extractor_class.define_method(
        "extract_entities_with_indices",
        method!(Extractor::extract_entities_with_indices, 1),
    )?;

    let entity_class = module.define_class("Entity", ruby.class_object())?;
    entity_class.define_method("entity_type", method!(Entity::entity_type, 0))?;
    entity_class.define_method("start", method!(Entity::start, 0))?;
    entity_class.define_method("end", method!(Entity::end, 0))?;
    entity_class.define_method("value", method!(Entity::value, 0))?;
    entity_class.define_method("list_slug", method!(Entity::list_slug, 0))?;
    entity_class.define_method("display_url", method!(Entity::display_url, 0))?;
    entity_class.define_method("expanded_url", method!(Entity::expanded_url, 0))?;

    let validating_extractor_class =
        module.define_class("ValidatingExtractor", ruby.class_object())?;
    validating_extractor_class
        .define_singleton_method("new", function!(ValidatingExtractor::ruby_new, 1))?;
    validating_extractor_class.define_method(
        "extract_mentioned_screennames_with_indices",
        method!(
            ValidatingExtractor::extract_mentioned_screennames_with_indices,
            1
        ),
    )?;
    validating_extractor_class.define_method(
        "extract_mentions_or_lists_with_indices",
        method!(
            ValidatingExtractor::extract_mentions_or_lists_with_indices,
            1
        ),
    )?;
    validating_extractor_class.define_method(
        "extract_reply_screenname",
        method!(ValidatingExtractor::extract_reply_screenname, 1),
    )?;
    validating_extractor_class.define_method(
        "extract_urls_with_indices",
        method!(ValidatingExtractor::extract_urls_with_indices, 1),
    )?;
    validating_extractor_class.define_method(
        "extract_hashtags_with_indices",
        method!(ValidatingExtractor::extract_hashtags_with_indices, 1),
    )?;
    validating_extractor_class.define_method(
        "extract_cashtags_with_indices",
        method!(ValidatingExtractor::extract_cashtags_with_indices, 1),
    )?;
    validating_extractor_class.define_method(
        "get_extract_url_without_protocol",
        method!(ValidatingExtractor::get_extract_url_without_protocol, 0),
    )?;
    validating_extractor_class.define_method(
        "set_extract_url_without_protocol",
        method!(ValidatingExtractor::set_extract_url_without_protocol, 1),
    )?;
    validating_extractor_class.define_method(
        "get_normalize",
        method!(ValidatingExtractor::get_normalize, 0),
    )?;
    validating_extractor_class.define_method(
        "set_normalize",
        method!(ValidatingExtractor::set_normalize, 1),
    )?;

    let extract_result_class = module.define_class("ExtractResult", ruby.class_object())?;
    extract_result_class.define_method("entities", method!(ExtractResult::get_entities, 0))?;

    let mention_result_class = module.define_class("MentionResult", ruby.class_object())?;
    mention_result_class.define_method("entity", method!(MentionResult::get_entity, 0))?;

    let validator_class = module.define_class("Validator", ruby.class_object())?;
    validator_class.define_singleton_method("new", function!(Validator::ruby_new, 0))?;
    validator_class.define_method(
        "get_max_tweet_length",
        method!(Validator::get_max_tweet_length, 0),
    )?;
    validator_class.define_method(
        "get_short_url_length",
        method!(Validator::get_short_url_length, 0),
    )?;
    validator_class.define_method(
        "set_short_url_length",
        method!(Validator::set_short_url_length, 1),
    )?;
    validator_class.define_method(
        "get_short_url_length_https",
        method!(Validator::get_short_url_length_https, 0),
    )?;
    validator_class.define_method(
        "set_short_url_length_https",
        method!(Validator::set_short_url_length_https, 1),
    )?;
    validator_class.define_method("is_valid_tweet", method!(Validator::is_valid_tweet, 1))?;
    validator_class.define_method(
        "is_valid_username",
        method!(Validator::is_valid_username, 1),
    )?;
    validator_class.define_method("is_valid_list", method!(Validator::is_valid_list, 1))?;
    validator_class.define_method("is_valid_hashtag", method!(Validator::is_valid_hashtag, 1))?;
    validator_class.define_method("is_valid_url", method!(Validator::is_valid_url, 1))?;
    validator_class.define_method(
        "is_valid_url_without_protocol",
        method!(Validator::is_valid_url_without_protocol, 1),
    )?;

    let autolinker_class = module.define_class("Autolinker", ruby.class_object())?;
    autolinker_class.define_singleton_method("new", function!(Autolinker::ruby_new, 0))?;

    // Accessor methods
    autolinker_class.define_method("get_no_follow", method!(Autolinker::get_no_follow, 0))?;
    autolinker_class.define_method("set_no_follow", method!(Autolinker::set_no_follow, 1))?;
    autolinker_class.define_method("get_url_class", method!(Autolinker::get_url_class, 0))?;
    autolinker_class.define_method("set_url_class", method!(Autolinker::set_url_class, 1))?;
    autolinker_class.define_method("get_url_target", method!(Autolinker::get_url_target, 0))?;
    autolinker_class.define_method("set_url_target", method!(Autolinker::set_url_target, 1))?;
    autolinker_class.define_method("get_symbol_tag", method!(Autolinker::get_symbol_tag, 0))?;
    autolinker_class.define_method("set_symbol_tag", method!(Autolinker::set_symbol_tag, 1))?;
    autolinker_class.define_method(
        "get_text_with_symbol_tag",
        method!(Autolinker::get_text_with_symbol_tag, 0),
    )?;
    autolinker_class.define_method(
        "set_text_with_symbol_tag",
        method!(Autolinker::set_text_with_symbol_tag, 1),
    )?;
    autolinker_class.define_method("get_list_class", method!(Autolinker::get_list_class, 0))?;
    autolinker_class.define_method("set_list_class", method!(Autolinker::set_list_class, 1))?;
    autolinker_class.define_method(
        "get_username_class",
        method!(Autolinker::get_username_class, 0),
    )?;
    autolinker_class.define_method(
        "set_username_class",
        method!(Autolinker::set_username_class, 1),
    )?;
    autolinker_class.define_method(
        "get_hashtag_class",
        method!(Autolinker::get_hashtag_class, 0),
    )?;
    autolinker_class.define_method(
        "set_hashtag_class",
        method!(Autolinker::set_hashtag_class, 1),
    )?;
    autolinker_class.define_method(
        "get_cashtag_class",
        method!(Autolinker::get_cashtag_class, 0),
    )?;
    autolinker_class.define_method(
        "set_cashtag_class",
        method!(Autolinker::set_cashtag_class, 1),
    )?;
    autolinker_class.define_method(
        "get_username_url_base",
        method!(Autolinker::get_username_url_base, 0),
    )?;
    autolinker_class.define_method(
        "set_username_url_base",
        method!(Autolinker::set_username_url_base, 1),
    )?;
    autolinker_class.define_method(
        "get_list_url_base",
        method!(Autolinker::get_list_url_base, 0),
    )?;
    autolinker_class.define_method(
        "set_list_url_base",
        method!(Autolinker::set_list_url_base, 1),
    )?;
    autolinker_class.define_method(
        "get_hashtag_url_base",
        method!(Autolinker::get_hashtag_url_base, 0),
    )?;
    autolinker_class.define_method(
        "set_hashtag_url_base",
        method!(Autolinker::set_hashtag_url_base, 1),
    )?;
    autolinker_class.define_method(
        "get_cashtag_url_base",
        method!(Autolinker::get_cashtag_url_base, 0),
    )?;
    autolinker_class.define_method(
        "set_cashtag_url_base",
        method!(Autolinker::set_cashtag_url_base, 1),
    )?;
    autolinker_class.define_method(
        "get_invisible_tag_attrs",
        method!(Autolinker::get_invisible_tag_attrs, 0),
    )?;
    autolinker_class.define_method(
        "set_invisible_tag_attrs",
        method!(Autolinker::set_invisible_tag_attrs, 1),
    )?;
    autolinker_class.define_method(
        "get_username_include_symbol",
        method!(Autolinker::get_username_include_symbol, 0),
    )?;
    autolinker_class.define_method(
        "set_username_include_symbol",
        method!(Autolinker::set_username_include_symbol, 1),
    )?;

    // Autolink methods
    autolinker_class.define_method("autolink", method!(Autolinker::autolink, 1))?;
    autolinker_class.define_method(
        "autolink_usernames_and_lists",
        method!(Autolinker::autolink_usernames_and_lists, 1),
    )?;
    autolinker_class.define_method(
        "autolink_hashtags",
        method!(Autolinker::autolink_hashtags, 1),
    )?;
    autolinker_class.define_method("autolink_urls", method!(Autolinker::autolink_urls, 1))?;
    autolinker_class.define_method(
        "autolink_cashtags",
        method!(Autolinker::autolink_cashtags, 1),
    )?;

    let hit_class = module.define_class("Hit", ruby.class_object())?;
    hit_class.define_singleton_method("new", function!(Hit::ruby_new, 0))?;
    hit_class.define_method("start", method!(Hit::get_start, 0))?;
    hit_class.define_method("start=", method!(Hit::set_start, 1))?;
    hit_class.define_method("end", method!(Hit::get_end, 0))?;
    hit_class.define_method("end=", method!(Hit::set_end, 1))?;

    let hits_class = module.define_class("Hits", ruby.class_object())?;
    hits_class.define_singleton_method("new", function!(Hits::ruby_new, 0))?;
    hits_class.define_method("append", method!(Hits::append, 1))?;
    hits_class.define_method("push", method!(Hits::append, 1))?;

    let highlighter_class = module.define_class("HitHighlighter", ruby.class_object())?;
    highlighter_class.define_singleton_method("new", function!(HitHighlighter::ruby_new, -1))?;
    highlighter_class.define_method(
        "get_highlight_tag",
        method!(HitHighlighter::get_highlight_tag, 0),
    )?;
    highlighter_class.define_method(
        "set_highlight_tag",
        method!(HitHighlighter::set_highlight_tag, 1),
    )?;
    highlighter_class.define_method("hit_highlight", method!(HitHighlighter::hit_highlight, 2))?;
    highlighter_class.define_method("highlight", method!(HitHighlighter::hit_highlight, 2))?;

    // Register Range class
    let range_class = module.define_class("Range", ruby.class_object())?;
    range_class.define_method("start", method!(RubyRange::get_start, 0))?;
    range_class.define_method("start=", method!(RubyRange::set_start, 1))?;
    range_class.define_method("end", method!(RubyRange::get_end, 0))?;
    range_class.define_method("end=", method!(RubyRange::set_end, 1))?;

    // Register WeightedRange class
    let weighted_range_class = module.define_class("WeightedRange", ruby.class_object())?;
    weighted_range_class.define_method("range", method!(WeightedRange::get_range, 0))?;
    weighted_range_class.define_method("weight", method!(WeightedRange::get_weight, 0))?;
    weighted_range_class.define_method("weight=", method!(WeightedRange::set_weight, 1))?;

    // Register TwitterTextConfiguration class
    let config_class = module.define_class("TwitterTextConfiguration", ruby.class_object())?;
    config_class
        .define_singleton_method("new", function!(TwitterTextConfiguration::ruby_new, 0))?;
    config_class.define_singleton_method(
        "configuration_from_path",
        function!(TwitterTextConfiguration::configuration_from_path, 1),
    )?;
    config_class.define_singleton_method(
        "configuration_from_json",
        function!(TwitterTextConfiguration::configuration_from_json, 1),
    )?;
    config_class.define_singleton_method(
        "config_v1",
        function!(TwitterTextConfiguration::config_v1, 0),
    )?;
    config_class.define_singleton_method(
        "config_v2",
        function!(TwitterTextConfiguration::config_v2, 0),
    )?;
    config_class.define_singleton_method(
        "config_v3",
        function!(TwitterTextConfiguration::config_v3, 0),
    )?;

    config_class.define_method(
        "get_version",
        method!(TwitterTextConfiguration::get_version, 0),
    )?;
    config_class.define_method(
        "set_version",
        method!(TwitterTextConfiguration::set_version, 1),
    )?;
    config_class.define_method(
        "get_max_weighted_tweet_length",
        method!(TwitterTextConfiguration::get_max_weighted_tweet_length, 0),
    )?;
    config_class.define_method(
        "set_max_weighted_tweet_length",
        method!(TwitterTextConfiguration::set_max_weighted_tweet_length, 1),
    )?;
    config_class.define_method("get_scale", method!(TwitterTextConfiguration::get_scale, 0))?;
    config_class.define_method("set_scale", method!(TwitterTextConfiguration::set_scale, 1))?;
    config_class.define_method(
        "get_default_weight",
        method!(TwitterTextConfiguration::get_default_weight, 0),
    )?;
    config_class.define_method(
        "set_default_weight",
        method!(TwitterTextConfiguration::set_default_weight, 1),
    )?;
    config_class.define_method(
        "get_transformed_url_length",
        method!(TwitterTextConfiguration::get_transformed_url_length, 0),
    )?;
    config_class.define_method(
        "set_transformed_url_length",
        method!(TwitterTextConfiguration::set_transformed_url_length, 1),
    )?;
    config_class.define_method(
        "get_emoji_parsing_enabled",
        method!(TwitterTextConfiguration::get_emoji_parsing_enabled, 0),
    )?;
    config_class.define_method(
        "set_emoji_parsing_enabled",
        method!(TwitterTextConfiguration::set_emoji_parsing_enabled, 1),
    )?;
    config_class.define_method(
        "get_ranges",
        method!(TwitterTextConfiguration::get_ranges, 0),
    )?;

    // Also register as Configuration for compatibility with existing code
    let config_alias = module.define_class("Configuration", ruby.class_object())?;
    config_alias.define_singleton_method(
        "configuration_from_path",
        function!(TwitterTextConfiguration::configuration_from_path, 1),
    )?;

    // Register TwitterTextParser and related classes
    let parser_range_class = module.define_class("ParseRange", ruby.class_object())?;
    parser_range_class.define_method("start", method!(ParseRange::get_start, 0))?;
    parser_range_class.define_method("start=", method!(ParseRange::set_start, 1))?;
    parser_range_class.define_method("end", method!(ParseRange::get_end, 0))?;
    parser_range_class.define_method("end=", method!(ParseRange::set_end, 1))?;

    let parse_result_class = module.define_class("TwitterTextParseResult", ruby.class_object())?;
    parse_result_class.define_method(
        "weighted_length",
        method!(TwitterTextParseResult::get_weighted_length, 0),
    )?;
    parse_result_class.define_method(
        "permillage",
        method!(TwitterTextParseResult::get_permillage, 0),
    )?;
    parse_result_class
        .define_method("is_valid", method!(TwitterTextParseResult::get_is_valid, 0))?;
    parse_result_class.define_method(
        "display_text_range",
        method!(TwitterTextParseResult::get_display_text_range, 0),
    )?;
    parse_result_class.define_method(
        "valid_text_range",
        method!(TwitterTextParseResult::get_valid_text_range, 0),
    )?;

    let parser_class = module.define_class("TwitterTextParser", ruby.class_object())?;
    parser_class.define_singleton_method("parse", function!(TwitterTextParser::parse, 3))?;

    Ok(())
}
