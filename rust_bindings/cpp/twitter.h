#pragma once
#include "rust/twitter-text/twitter-text.h"

namespace twitter_text {

class TwitterTextConfiguration {
public:
  TwitterTextConfiguration(): TwitterTextConfiguration(ffi::default_config())
  {}

  TwitterTextConfiguration(std::unique_ptr<ffi::Configuration> config):
    config(std::move(config))
  {}

  // TODO: these are fallible, so the return type should change 
  static std::unique_ptr<TwitterTextConfiguration> configuration_from_path(std::string path) {
    return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::configuration_from_path(path)));
  }

  static std::unique_ptr<TwitterTextConfiguration> configuration_from_json(std::string json) {
    return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::configuration_from_json(json)));
  }

  int32_t getVersion() {
    return config->version;
  }

  void setVersion(int32_t version) {
    config->version = version;
  }

  int32_t getMaxWeightedTweetLength() {
    return config->max_weighted_tweet_length;
  }

  void setMaxWeightedTweetLength(int32_t max) {
    config->max_weighted_tweet_length = max;
  }

  int32_t getScale() {
    return config->scale;
  }

  void setScale(int32_t scale) {
    config->scale = scale;
  }

  int32_t getDefaultWeight() {
    return config->default_weight;
  }

  void setDefaultWeight(int32_t weight) {
    config->default_weight = weight;
  }

  int32_t getTransformedUrlLength() {
    return config->transformed_url_length;
  }

  void setTransformedUrlLength(int32_t length) {
    config->transformed_url_length = length;
  }

  bool getEmojiParsingEnabled() {
    return config->emoji_parsing_enabled;
  }

  void setEmojiParsingEnabled(bool enabled) {
    config->emoji_parsing_enabled = enabled;
  }

  static std::unique_ptr<TwitterTextConfiguration> configV1()  {
    return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::config_v1()));
  }

  static std::unique_ptr<TwitterTextConfiguration> configV2() {
    return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::config_v2()));
  }

  static std::unique_ptr<TwitterTextConfiguration> configV3() {
    return std::unique_ptr<TwitterTextConfiguration>(new TwitterTextConfiguration(ffi::config_v3()));
  }

  std::vector<ffi::WeightedRange> getRanges() {
    std::vector<ffi::WeightedRange> stdv;
    std::copy(config->ranges.begin(), config->ranges.end(), std::back_inserter(stdv));
    return stdv;
  }

private:
  std::unique_ptr<ffi::Configuration> config;

  friend class ValidatingExtractor;

  friend class TwitterTextParser;
};

class Autolinker {
public:
  Autolinker(): 
    config(ffi::autolink_default_config()) 
  {}

  bool getNoFollow();
  void setNoFollow(bool noFollow);

  ::rust::String getUrlClass();
  void setUrlClass(std::string urlClass);

  ::rust::String getUrlTarget();
  void setUrlTarget(std::string urlTarget);

  ::rust::String getSymbolTag();
  void setSymbolTag(std::string symbolTag);

  ::rust::String getTextWithSymbolTag();
  void setTextWithSymbolTag(std::string textWithSymbolTag);

  ::rust::String getListClass();
  void setListClass(std::string listClass);

  ::rust::String getUsernameClass();
  void setUsernameClass(std::string usernameClass);

  ::rust::String getHashtagClass();
  void setHashtagClass(std::string hashtagClass);

  ::rust::String getCashtagClass();
  void setCashtagClass(std::string cashtagClass);

  ::rust::String getUsernameUrlBase();
  void setUsernameUrlBase(std::string usernameUrlBase);

  ::rust::String getListUrlBase();
  void setListUrlBase(std::string listUrlBase);

  ::rust::String getHashtagUrlBase();
  void setHashtagUrlBase(std::string hashtagUrlBase);

  ::rust::String getCashtagUrlBase();
  void setCashtagUrlBase(std::string cashtagUrlBase);

  ::rust::String getInvisibleTagAttrs();
  void setInvisibleTagAttrs(std::string invisibleTagAttrs);

  bool getUsernameIncludeSymbol();
  void setUsernameIncludeSymbol(bool usernameIncludeSymbol);

  ::rust::String autolink(std::string &text);
  ::rust::String autolinkUsernamesAndLists(std::string &text);
  ::rust::String autolinkHashtags(std::string &text);
  ::rust::String autolinkUrls(std::string &text);
  ::rust::String autolinkCashtags(std::string &text);

private:
  std::unique_ptr<ffi::AutolinkerConfig> config;
};

typedef ffi::Entity Entity;

class Extractor {
public:
  Extractor():
    extractor(ffi::make_extractor()) 
  {}

  bool getExtractUrlWithoutProtocol() {
    return ffi::get_extract_url_without_protocol(*extractor);
  }

  void setExtractUrlWithoutProtocol(bool extractUrlwp) {
    ffi::set_extract_url_without_protocol(*extractor, extractUrlwp);
  }

  std::vector<Entity> extractEntitiesWithIndices(std::string &text) {
    auto entities = ffi::extract_entities_with_indices(*extractor, text);
    return entitiesToCpp(entities); 
  }

  std::vector<std::string> extractMentionedScreennames(std::string &text) {
    auto extractor_strings = ffi::extract_mentioned_screennames(*extractor, text);
    return extractorStringsToCpp(extractor_strings);
  }

  ::rust::Vec<Entity> extractMentionedScreennamesWithIndices(std::string &text) {
    return ffi::extract_mentioned_screennames_with_indices(*extractor, text);
  }

  ::rust::Vec<Entity> extractMentionsOrListsWithIndices(std::string &text) {
    return ffi::extract_mentions_or_lists_with_indices(*extractor, text);
  }

  std::unique_ptr<Entity> extractReplyScreenname(std::string &text) {
    return ffi::extract_reply_username(*extractor, text);
  }

  std::vector<std::string> extractUrls(std::string &text) {
    auto extractor_strings = ffi::extract_urls(*extractor, text);
    return extractorStringsToCpp(extractor_strings);
  }

  ::rust::Vec<Entity> extractUrlsWithIndices(std::string &text) {
    return ffi::extract_urls_with_indices(*extractor, text);
  }

  std::vector<std::string> extractHashtags(std::string &text) {
    auto extractor_strings = ffi::extract_hashtags(*extractor, text);
    return extractorStringsToCpp(extractor_strings); 
  }

  ::rust::Vec<Entity> extractHashtagsWithIndices(std::string &text) {
    return ffi::extract_hashtags_with_indices(*extractor, text);
  }

  std::vector<std::string> extractCashtags(std::string &text) {
    auto extractor_strings = ffi::extract_cashtags(*extractor, text);
    return extractorStringsToCpp(extractor_strings); 
  }

  ::rust::Vec<Entity> extractCashtagsWithIndices(std::string &text) {
    return ffi::extract_cashtags_with_indices(*extractor, text);
  }

private:
  std::vector<Entity> entitiesToCpp(::rust::Vec<Entity> &rustVec);
  std::vector<std::string> extractorStringsToCpp(::rust::Vec<ffi::ExtractorString> &rustVec);
  ::rust::Box<ffi::Extractor> extractor;
};

typedef ffi::TwitterTextParseResults TwitterTextParseResults;

struct ExtractResult final {
  TwitterTextParseResults parseResults;
  std::vector<Entity> entities;
};

typedef ffi::MentionResult MentionResult;

class ValidatingExtractor {
public:
  ValidatingExtractor(TwitterTextConfiguration &ttc):
    extractor(ffi::make_validating_extractor(*ttc.config)) 
  {}

  bool getExtractUrlWithoutProtocol() {
    return ffi::get_extract_url_without_protocol_validated(*extractor);
  }

  void setExtractUrlWithoutProtocol(bool extractUrlwp) {
    ffi::set_extract_url_without_protocol_validated(*extractor, extractUrlwp);
  }

  bool getNormalize() {
    return ffi::get_normalize(*extractor);
  }

  void setNormalize(bool normalize) {
    ffi::set_normalize(*extractor, normalize);
  }

  std::unique_ptr<ExtractResult> extractEntitiesWithIndices(std::string &text) {
    auto result = ffi::extract_entities_with_indices_validated(*extractor, text);
    return convertResult(result);
  }

  std::unique_ptr<ExtractResult> extractMentionedScreennamesWithIndices(std::string &text) {
    auto result = ffi::extract_mentioned_screennames_with_indices_validated(*extractor, text);
    return convertResult(result);
  }

  std::unique_ptr<ExtractResult> extractMentionsOrListsWithIndices(std::string &text) {
    auto result = ffi::extract_mentions_or_lists_with_indices_validated(*extractor, text);
    return convertResult(result);
  }

  std::unique_ptr<MentionResult> extractReplyScreenname(std::string &text);

  std::unique_ptr<ExtractResult> extractUrlsWithIndices(std::string &text) {
    auto result = ffi::extract_urls_with_indices_validated(*extractor, text);
    return convertResult(result);
  }

  std::unique_ptr<ExtractResult> extractHashtagsWithIndices(std::string &text) {
    auto result = ffi::extract_hashtags_with_indices_validated(*extractor, text);
    return convertResult(result);
  }

  std::unique_ptr<ExtractResult> extractCashtagsWithIndices(std::string &text) {
    auto result = ffi::extract_cashtags_with_indices_validated(*extractor, text);
    return convertResult(result);
  }

private:
  std::unique_ptr<ExtractResult> convertResult(ffi::ExtractResult &result);
  std::vector<Entity> entitiesToCpp(::rust::Vec<Entity> &rustVec);
  ::rust::Box<ffi::FFIValidatingExtractor> extractor;
};


typedef ffi::Hit Hit;

class HitHighlighter {
public:
  HitHighlighter():
    highlighter(ffi::make_default_highlighter()) 
  {}   
  
  HitHighlighter(std::string tag_str):
    highlighter(ffi::make_highlighter(tag_str)) 
  {}

  std::string highlight(std::string &text, std::vector<Hit> &hits);

private:
  ::rust::Box<ffi::HitHighlighter> highlighter;
};

class Validator {
public:
  Validator():
    validator(ffi::make_default_validator()) 
  {}

  bool isValidTweet(std::string &text) {
    return ffi::is_valid_tweet(*validator, text);
  }

  bool isValidUsername(std::string &text) {
    return ffi::is_valid_username(*validator, text);
  }

  bool isValidList(std::string &text) {
    return ffi::is_valid_list(*validator, text);
  }

  bool isValidHashtag(std::string &text) {
    return ffi::is_valid_hashtag(*validator, text);
  }

  bool isValidUrl(std::string &text) {
    return ffi::is_valid_url(*validator, text);
  }

  bool isValidUrlWithoutProtocol(std::string &text) {
    return ffi::is_valid_url_without_protocol(*validator, text);
  }

  int32_t getMaxTweetLength() {
    return ffi::get_max_tweet_length();
  }

  int32_t getShortUrlLength() {
    return ffi::get_short_url_length(*validator);
  }

  void setShortUrlLength(int32_t i) {
    return ffi::set_short_url_length(*validator, i);
  }

  int32_t getShortUrlLengthHttps() {
    return ffi::get_short_url_length_https(*validator);
  }

  void setShortUrlLengthHttps(int32_t i) {
    return ffi::set_short_url_length_https(*validator, i);
  }

private:
  ::rust::Box<ffi::Validator> validator;
};

class TwitterTextParser {
public:
  static TwitterTextParseResults parse(std::string &text, TwitterTextConfiguration &ttc, bool parseUrls) {
    return ffi::parse_ffi(text, *ttc.config, parseUrls);
  }
};

} // twitter_text