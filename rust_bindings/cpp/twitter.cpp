#include "twitter.h"
#include <iostream>
#include <memory>

namespace twitter_text {

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configuration_from_path(std::string path) {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::configuration_from_path(path)));
}

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configuration_from_json(std::string json) {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::configuration_from_json(json)));
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

std::vector<ffi::WeightedRange> 
TwitterTextConfiguration::getRanges() {
  std::vector<ffi::WeightedRange> stdv;
  std::copy(config->ranges.begin(), config->ranges.end(), std::back_inserter(stdv));
  return stdv;
}

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configV1() {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::config_v1()));
}

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configV2() {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::config_v2()));
}

std::unique_ptr<TwitterTextConfiguration>
TwitterTextConfiguration::configV3() {
  return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::config_v3()));
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
  return std::string(ffi::autolink(text, *config));
}

std::string
Autolinker::autolinkUsernamesAndLists(std::string &text) {
  return std::string(ffi::autolink_usernames_and_lists(text, *config));
}

std::string
Autolinker::autolinkHashtags(std::string &text) {
  return std::string(ffi::autolink_hashtags(text, *config));
}

std::string
Autolinker::autolinkUrls(std::string &text) {
  return std::string(ffi::autolink_urls(text, *config));
}

std::string
Autolinker::autolinkCashtags(std::string &text) { 
  return std::string(ffi::autolink_cashtags(text, *config));
}

// Extractor
bool
Extractor::getExtractUrlWithoutProtocol() {
  return ffi::get_extract_url_without_protocol(*extractor);
}

void
Extractor::setExtractUrlWithoutProtocol(bool extractUrlwp) {
  ffi::set_extract_url_without_protocol(*extractor, extractUrlwp);
}

std::vector<Entity>
entitiesToCpp(::rust::Vec<Entity> &rustVec) {
  std::vector<Entity> stdv;
  stdv.reserve(rustVec.size());
  std::copy(rustVec.begin(), rustVec.end(), std::back_inserter(stdv));
  return stdv;
}

std::unique_ptr<ExtractResult>
convertResult(ffi::ExtractResult &result) {
  std::unique_ptr<ExtractResult> er(new ExtractResult());
  er->parseResults = result.parse_results;
  er->entities = entitiesToCpp(result.entities);
  return er;
}

std::vector<std::string>
extractorStringsToCpp(::rust::Vec<ffi::ExtractorString> &rustVec) {
  std::vector<std::string> stdv;
  stdv.reserve(rustVec.size());
  for (ffi::ExtractorString es : rustVec) {
    stdv.push_back(std::string(es.s));
  }
  return stdv;
}

std::vector<Entity>
Extractor::extractEntitiesWithIndices(std::string &text) {
  auto entities = ffi::extract_entities_with_indices(*extractor, text);
  return entitiesToCpp(entities); 
}

std::vector<std::string>
Extractor::extractMentionedScreennames(std::string &text) {
  auto extractor_strings = ffi::extract_mentioned_screennames(*extractor, text);
  return extractorStringsToCpp(extractor_strings);
}

std::vector<Entity>
Extractor::extractMentionedScreennamesWithIndices(std::string &text) {
  auto entities = ffi::extract_mentioned_screennames_with_indices(*extractor, text);
  return entitiesToCpp(entities); 
}

std::vector<Entity>
Extractor::extractMentionsOrListsWithIndices(std::string &text) {
  auto entities = ffi::extract_mentions_or_lists_with_indices(*extractor, text);
  return entitiesToCpp(entities);
}

std::unique_ptr<Entity>
Extractor::extractReplyScreenname(std::string &text) {
  return ffi::extract_reply_username(*extractor, text);
}

std::vector<std::string>
Extractor::extractUrls(std::string &text) {
  auto extractor_strings = ffi::extract_urls(*extractor, text);
  return extractorStringsToCpp(extractor_strings);
}

std::vector<Entity>
Extractor::extractUrlsWithIndices(std::string &text) {
  auto entities = ffi::extract_urls_with_indices(*extractor, text);
  return entitiesToCpp(entities);
}

std::vector<std::string>
Extractor::extractHashtags(std::string &text) {
  auto extractor_strings = ffi::extract_hashtags(*extractor, text);
  return extractorStringsToCpp(extractor_strings); 
}

std::vector<Entity>
Extractor::extractHashtagsWithIndices(std::string &text) {
  auto entities = ffi::extract_hashtags_with_indices(*extractor, text);
  return entitiesToCpp(entities);
}

std::vector<std::string>
Extractor::extractCashtags(std::string &text) {
  auto extractor_strings = ffi::extract_cashtags(*extractor, text);
  return extractorStringsToCpp(extractor_strings); 
}

std::vector<Entity>
Extractor::extractCashtagsWithIndices(std::string &text) {
  auto entities = ffi::extract_cashtags_with_indices(*extractor, text);
  return entitiesToCpp(entities);
}

// ValidatingExtractor
bool
ValidatingExtractor::getExtractUrlWithoutProtocol() {
  return ffi::get_extract_url_without_protocol_validated(*extractor);
}

void
ValidatingExtractor::setExtractUrlWithoutProtocol(bool extractUrlwp) {
  ffi::set_extract_url_without_protocol_validated(*extractor, extractUrlwp);
}

bool
ValidatingExtractor::getNormalize() {
  return ffi::get_normalize(*extractor);
}

void
ValidatingExtractor::setNormalize(bool normalize) {
  ffi::set_normalize(*extractor, normalize);
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::extractEntitiesWithIndices(std::string &text) {
  auto result = ffi::extract_entities_with_indices_validated(*extractor, text);
  return convertResult(result);
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::extractMentionedScreennamesWithIndices(std::string &text) {
  auto result = ffi::extract_mentioned_screennames_with_indices_validated(*extractor, text);
  return convertResult(result);
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::extractMentionsOrListsWithIndices(std::string &text) {
  auto result = ffi::extract_mentions_or_lists_with_indices_validated(*extractor, text);
  return convertResult(result);
}

std::unique_ptr<MentionResult>
ValidatingExtractor::extractReplyScreenname(std::string &text) {
  return ffi::extract_reply_username_validated(*extractor, text);
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::extractUrlsWithIndices(std::string &text) {
  auto result = ffi::extract_urls_with_indices_validated(*extractor, text);
  return convertResult(result);
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::extractHashtagsWithIndices(std::string &text) {
  auto result = ffi::extract_hashtags_with_indices_validated(*extractor, text);
  return convertResult(result);
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::extractCashtagsWithIndices(std::string &text) {
  auto result = ffi::extract_cashtags_with_indices_validated(*extractor, text);
  return convertResult(result);
}

// HitHighlighter
std::string
HitHighlighter::highlight(std::string &text, std::vector<Hit> &hits) {
  return std::string(ffi::hit_highlight(*highlighter, text, hits));
}

// Validator
bool 
Validator::isValidTweet(std::string &text) {
  return ffi::is_valid_tweet(*validator, text);
}

bool 
Validator::isValidUsername(std::string &text) {
  return ffi::is_valid_username(*validator, text);
}

bool 
Validator::isValidList(std::string &text) {
  return ffi::is_valid_list(*validator, text);
}

bool
Validator::isValidHashtag(std::string &text) {
  return ffi::is_valid_hashtag(*validator, text);
}

bool
Validator::isValidUrl(std::string &text) {
  return ffi::is_valid_url(*validator, text);
}

bool
Validator::isValidUrlWithoutProtocol(std::string &text) {
  return ffi::is_valid_url_without_protocol(*validator, text);
}

int32_t
Validator::getMaxTweetLength() {
  return ffi::get_max_tweet_length();
}

int32_t
Validator::getShortUrlLength() {
  return ffi::get_short_url_length(*validator);
}

void
Validator::setShortUrlLength(int32_t i) {
  return ffi::set_short_url_length(*validator, i);
}

int32_t
Validator::getShortUrlLengthHttps() {
  return ffi::get_short_url_length_https(*validator);
}

void
Validator::setShortUrlLengthHttps(int32_t i) {
  return ffi::set_short_url_length_https(*validator, i);
}

TwitterTextParseResults
TwitterTextParser::parse(std::string &text, TwitterTextConfiguration &ttc, bool parseUrls) {
  return ffi::parse_ffi(text, *ttc.config, parseUrls);
}


} // twitter_text