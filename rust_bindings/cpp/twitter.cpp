#include "twitter.h"

namespace twitter_text {

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configuration_from_path(std::string path) {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(twitter_text_ffi::configuration_from_path(path)));
}

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configuration_from_json(std::string json) {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(twitter_text_ffi::configuration_from_json(json)));
}

int32_t
TwitterTextConfiguration::getVersion() {
  return config->version;
}

void
TwitterTextConfiguration::setVersion(int32_t version) {
  config->version = version;
}

int32_t
TwitterTextConfiguration::getMaxWeightedTweetLength() {
  return config->max_weighted_tweet_length;
}

void
TwitterTextConfiguration::setMaxWeightedTweetLength(int32_t max) {
  config->max_weighted_tweet_length = max;
}

int32_t
TwitterTextConfiguration::getScale() {
  return config->scale;
}

void
TwitterTextConfiguration::setScale(int32_t scale) {
  config->scale = scale;
}

int32_t
TwitterTextConfiguration::getDefaultWeight() {
  return config->default_weight;
}

void
TwitterTextConfiguration::setDefaultWeight(int32_t weight) {
  config->default_weight = weight;
}

int32_t
TwitterTextConfiguration::getTransformedUrlLength() {
  return config->transformed_url_length;
}

void
TwitterTextConfiguration::setTransformedUrlLength(int32_t length) {
  config->transformed_url_length = length;
}

bool
TwitterTextConfiguration::getEmojiParsingEnabled() {
  return config->emoji_parsing_enabled;
}

void
TwitterTextConfiguration::setEmojiParsingEnabled(bool enabled) {
  config->emoji_parsing_enabled = enabled;
}

std::vector<twitter_text_ffi::WeightedRange> 
TwitterTextConfiguration::getRanges() {
  std::vector<twitter_text_ffi::WeightedRange> stdv;
  std::copy(config->ranges.begin(), config->ranges.end(), std::back_inserter(stdv));
  return stdv;
}

// Autolinker
bool
Autolinker::getNoFollow() {
  return config->no_follow;
}

void 
Autolinker::setNoFollow(bool noFollow) {
  config->no_follow = noFollow;
}

std::string
Autolinker::getUrlClass() {
  return std::string(config->url_class);
}

void
Autolinker::setUrlClass(std::string urlClass) {
  config->url_class = ::rust::String(urlClass);
}

std::string
Autolinker::getUrlTarget() {
  return std::string(config->url_target);
}

void
Autolinker::setUrlTarget(std::string urlTarget) {
  config->url_target = ::rust::String(urlTarget);
}

std::string
Autolinker::getSymbolTag() {
  return std::string(config->symbol_tag);
}

void
Autolinker::setSymbolTag(std::string symbolTag) {
  config->symbol_tag = ::rust::String(symbolTag);
}

std::string
Autolinker::getTextWithSymbolTag() {
  return std::string(config->text_with_symbol_tag); 
}

void
Autolinker::setTextWithSymbolTag(std::string textWithSymbolTag) {
  config->text_with_symbol_tag = ::rust::String(textWithSymbolTag);
}

std::string
Autolinker::getListClass() {
  return std::string(config->list_class); 
}

void
Autolinker::setListClass(std::string listClass) {
  config->list_class = rust::String(listClass);
}

std::string
Autolinker::getUsernameClass() {
  return std::string(config->username_class);
}

void
Autolinker::setUsernameClass(std::string usernameClass) {
  config->username_class = ::rust::String(usernameClass);
}

std::string
Autolinker::getHashtagClass() {
  return std::string(config->hashtag_class);  
}

void
Autolinker::setHashtagClass(std::string hashtagClass) {
  config->hashtag_class = ::rust::String(hashtagClass);
}

std::string
Autolinker::getCashtagClass() {
  return std::string(config->cashtag_class);  
}

void
Autolinker::setCashtagClass(std::string cashtagClass) {
  config->cashtag_class = ::rust::String(cashtagClass);
}

std::string
Autolinker::getUsernameUrlBase() {
  return std::string(config->username_url_base);  
}

void
Autolinker::setUsernameUrlBase(std::string usernameUrlBase) {
  config->username_url_base = ::rust::String(usernameUrlBase);
}

std::string
Autolinker::getListUrlBase() {
  return std::string(config->list_url_base);
}

void
Autolinker::setListUrlBase(std::string listUrlBase) {
  config->list_url_base = ::rust::String(listUrlBase);
}

std::string
Autolinker::getHashtagUrlBase() {
  return std::string(config->hashtag_url_base);  
}

void
Autolinker::setHashtagUrlBase(std::string hashtagUrlBase) {
  config->hashtag_url_base = ::rust::String(hashtagUrlBase);
}

std::string
Autolinker::getCashtagUrlBase() {
  return std::string(config->cashtag_url_base);
}

void
Autolinker::setCashtagUrlBase(std::string cashtagUrlBase) {
  config->cashtag_url_base = ::rust::String(cashtagUrlBase);
}

std::string
Autolinker::getInvisibleTagAttrs() {
  return std::string(config->invisible_tag_attrs);
}

void
Autolinker::setInvisibleTagAttrs(std::string invisibleTagAttrs) {
  config->invisible_tag_attrs = ::rust::String(invisibleTagAttrs);
}

bool
Autolinker::getUsernameIncludeSymbol() {
  return config->username_include_symbol;
}

void
Autolinker::setUsernameIncludeSymbol(bool usernameIncludeSymbol) {
  config->username_include_symbol = usernameIncludeSymbol;
}

std::string
Autolinker::autolink(std::string &text) {
  return std::string(twitter_text_ffi::autolink(text, *config));
}

std::string
Autolinker::autolinkUsernamesAndLists(std::string &text) {
  return std::string(twitter_text_ffi::autolink_usernames_and_lists(text, *config));
}

std::string
Autolinker::autolinkHashtags(std::string &text) {
  return std::string(twitter_text_ffi::autolink_hashtags(text, *config));
}

std::string
Autolinker::autolinkUrls(std::string &text) {
  return std::string(twitter_text_ffi::autolink_urls(text, *config));
}

std::string
Autolinker::autolinkCashtags(std::string &text) { 
  return std::string(twitter_text_ffi::autolink_cashtags(text, *config));
}

// HitHighlighter
std::string
HitHighlighter::highlight(std::string &text, std::vector<Hit> &hits) {
    return std::string(twitter_text_ffi::hit_highlight(text, hits, *highlighter));
}

// Validator
bool 
Validator::isValidTweet(std::string &text) {
  return twitter_text_ffi::is_valid_tweet(*validator, text);
}

bool 
Validator::isValidUsername(std::string &text) {
  return twitter_text_ffi::is_valid_username(*validator, text);
}

bool 
Validator::isValidList(std::string &text) {
  return twitter_text_ffi::is_valid_list(*validator, text);
}

bool
Validator::isValidHashtag(std::string &text) {
  return twitter_text_ffi::is_valid_hashtag(*validator, text);
}

bool
Validator::isValidUrl(std::string &text) {
  return twitter_text_ffi::is_valid_url(*validator, text);
}

bool
Validator::isValidUrlWithoutProtocol(std::string &text) {
  return twitter_text_ffi::is_valid_url_without_protocol(*validator, text);
}

int32_t
Validator::getMaxTweetLength() {
  return twitter_text_ffi::get_max_tweet_length();
}

int32_t
Validator::getShortUrlLength() {
  return twitter_text_ffi::get_short_url_length(*validator);
}

void
Validator::setShortUrlLength(int32_t i) {
  return twitter_text_ffi::set_short_url_length(*validator, i);
}

int32_t
Validator::getShortUrlLengthHttps() {
  return twitter_text_ffi::get_short_url_length_https(*validator);
}

void
Validator::setShortUrlLengthHttps(int32_t i) {
  return twitter_text_ffi::set_short_url_length_https(*validator, i);
}



























} // twitter_text