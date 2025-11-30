use magnus::{function, method, prelude::*, Error, Ruby};

mod autolinker;
mod configuration;
mod extractor;
mod hithighlighter;
mod validator;

use autolinker::Autolinker;
use configuration::TwitterTextConfiguration;
use extractor::{Entity, Extractor, ValidatingExtractor};
use hithighlighter::HitHighlighter;
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

    let highlighter_class = module.define_class("HitHighlighter", ruby.class_object())?;
    highlighter_class.define_singleton_method("new", function!(HitHighlighter::ruby_new, 0))?;
    highlighter_class.define_method("hit_highlight", method!(HitHighlighter::hit_highlight, 1))?;

    let config_class = module.define_class("Configuration", ruby.class_object())?;
    config_class.define_singleton_method(
        "configuration_from_path",
        function!(TwitterTextConfiguration::configuration_from_path, 1),
    )?;

    // Also register as TwitterTextConfiguration for compatibility
    let config_class2 = module.define_class("TwitterTextConfiguration", ruby.class_object())?;
    config_class2
        .define_singleton_method("new", function!(TwitterTextConfiguration::ruby_new, 0))?;
    config_class2.define_singleton_method(
        "configuration_from_path",
        function!(TwitterTextConfiguration::configuration_from_path, 1),
    )?;

    Ok(())
}
