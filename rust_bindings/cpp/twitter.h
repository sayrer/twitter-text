#pragma once
#include "rust/twitter-text/twitter-text.h"

namespace twitter_text {

class TwitterTextConfiguration {
public:
  TwitterTextConfiguration(): TwitterTextConfiguration(default_config())
  {}

  TwitterTextConfiguration(std::shared_ptr<::twitter_text::Configuration> config):
    config(config)
  {}

  // TODO: these are fallible, so the return type should change 
  static TwitterTextConfiguration* configurationFromPath(std::string path) {
    return new TwitterTextConfiguration(configuration_from_path(path));
  }

  static TwitterTextConfiguration* configurationFromJson(std::string json) {
    return new TwitterTextConfiguration(configuration_from_json(json));
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

  static TwitterTextConfiguration* configV1()  {
    return new TwitterTextConfiguration(config_v1());
  }

  static TwitterTextConfiguration* configV2() {
    return new TwitterTextConfiguration(config_v2());
  }

  static TwitterTextConfiguration* configV3() {
    return new TwitterTextConfiguration(config_v3());
  }

  std::vector<::twitter_text::WeightedRange> getRanges() {
    std::vector<WeightedRange> stdv;
    std::copy(config->ranges.begin(), config->ranges.end(), std::back_inserter(stdv));
    return stdv;
  }

private:
  std::shared_ptr<Configuration> config;

  friend class ValidatingExtractor;

  friend class TwitterTextParser;
};

class Autolinker {
public:
  Autolinker(): 
    config(::twitter_text::autolink_default_config()) 
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

  ::rust::String autolink(const std::string &text);
  ::rust::String autolinkUsernamesAndLists(const std::string &text);
  ::rust::String autolinkHashtags(const std::string &text);
  ::rust::String autolinkUrls(const std::string &text);
  ::rust::String autolinkCashtags(const std::string &text);

private:
  std::unique_ptr<::twitter_text::AutolinkerConfig> config;
};

class Extractor {
public:
  Extractor():
    extractor(::twitter_text::make_extractor()) 
  {}

  bool getExtractUrlWithoutProtocol() {
    return get_extract_url_without_protocol(*extractor);
  }

  void setExtractUrlWithoutProtocol(bool extractUrlwp) {
    set_extract_url_without_protocol(*extractor, extractUrlwp);
  }

  std::vector<Entity> extractEntitiesWithIndices(std::string text) {
    auto vec = extract_entities_with_indices(*extractor, text);
    return entitiesToCpp(vec);
  }

  std::vector<ExtractorString> extractMentionedScreennames(std::string text) {
    auto vec = extract_mentioned_screennames(*extractor, text);
    return extractorStringsToCpp(vec);
  }

  std::vector<Entity> extractMentionedScreennamesWithIndices(std::string text) {
    auto vec = extract_mentioned_screennames_with_indices(*extractor, text);
    return entitiesToCpp(vec);
  }

  std::vector<Entity> extractMentionsOrListsWithIndices(std::string text) {
    auto vec = extract_mentions_or_lists_with_indices(*extractor, text);
    return entitiesToCpp(vec);
  }

  Entity* extractReplyScreenname(std::string text) {
    return extract_reply_username(*extractor, text).release();
  }

  std::vector<ExtractorString> extractUrls(std::string text) {
    auto vec = extract_urls(*extractor, text);
    return extractorStringsToCpp(vec);
  }

  std::vector<Entity> extractUrlsWithIndices(std::string text) {
    auto vec = extract_urls_with_indices(*extractor, text);
    return entitiesToCpp(vec);
  }

  std::vector<ExtractorString> extractHashtags(std::string text) {
    auto vec = extract_hashtags(*extractor, text);
    std::vector<ExtractorString> ret = extractorStringsToCpp(vec);
    if (!ret.empty()) {
      //printf("Vec 0: %s\n", ret[0].c_str());
    }
    return ret;
  }

  std::vector<Entity> extractHashtagsWithIndices(std::string text) {
    auto vec = extract_hashtags_with_indices(*extractor, text);
    return entitiesToCpp(vec);
  }

  std::vector<ExtractorString> extractCashtags(std::string text) {
    auto vec = extract_cashtags(*extractor, text);
    return extractorStringsToCpp(vec);
  }

  std::vector<Entity> extractCashtagsWithIndices(std::string text) {
    auto vec = extract_cashtags_with_indices(*extractor, text);
    return entitiesToCpp(vec);
  }

private:
  std::vector<Entity> entitiesToCpp(::rust::Vec<Entity> &rustVec);
  std::vector<ExtractorString> extractorStringsToCpp(::rust::Vec<ExtractorString> &rustVec);
  ::rust::Box<RustExtractor> extractor;
};

class SwigExtractResult {
public:
  TwitterTextParseResults parseResults;
  std::vector<Entity> entities;
};  

class SwigMentionResult {
public:
  TwitterTextParseResults parseResults;
  Entity* entity;
};

class ValidatingExtractor {
public:
  ValidatingExtractor(TwitterTextConfiguration &ttc):
    extractor(make_validating_extractor(*ttc.config)) 
  {}

  bool getExtractUrlWithoutProtocol() {
    return get_extract_url_without_protocol_validated(*extractor);
  }

  void setExtractUrlWithoutProtocol(bool extractUrlwp) {
    set_extract_url_without_protocol_validated(*extractor, extractUrlwp);
  }

  bool getNormalize() {
    return get_normalize(*extractor);
  }

  void setNormalize(bool normalize) {
    set_normalize(*extractor, normalize);
  }

  SwigExtractResult* extractEntitiesWithIndices(const std::string &text) {
    auto result = extract_entities_with_indices_validated(*extractor, text);
    return convertResult(*result);
  }

  SwigExtractResult* extractMentionedScreennamesWithIndices(const std::string &text) {
    auto result = extract_mentioned_screennames_with_indices_validated(*extractor, text);
    return convertResult(*result);
  }

  SwigExtractResult* extractMentionsOrListsWithIndices(const std::string &text) {
    auto result = extract_mentions_or_lists_with_indices_validated(*extractor, text);
    return convertResult(*result);
  }

  SwigMentionResult* extractReplyScreenname(const std::string &text);

  SwigExtractResult* extractUrlsWithIndices(const std::string &text) {
    auto result = extract_urls_with_indices_validated(*extractor, text);
    return convertResult(*result);
  }

  SwigExtractResult* extractHashtagsWithIndices(const std::string &text) {
    auto result = extract_hashtags_with_indices_validated(*extractor, text);
    return convertResult(*result);
  }

  SwigExtractResult* extractCashtagsWithIndices(const std::string &text) {
    auto result = extract_cashtags_with_indices_validated(*extractor, text);
    return convertResult(*result);
  }

private:
  std::vector<Entity> entitiesToCpp(::rust::Vec<Entity> &rustVec);
  SwigExtractResult* convertResult(ExtractResult &result);
  ::rust::Box<RustValidatingExtractor> extractor;
};

class HitHighlighter {
public:
  HitHighlighter():
    highlighter(make_default_highlighter()) 
  {}   
  
  HitHighlighter(std::string tag_str):
    highlighter(make_highlighter(tag_str)) 
  {}

  std::string highlight(std::string text, std::vector<Hit> &hits);

private:
  ::rust::Box<RustHitHighlighter> highlighter;
};

class Validator {
public:
  Validator():
    validator(make_default_validator()) 
  {}

  bool isValidTweet(std::string text) {
    return is_valid_tweet(*validator, text);
  }

  bool isValidUsername(std::string text) {
    return is_valid_username(*validator, text);
  }

  bool isValidList(std::string text) {
    return is_valid_list(*validator, text);
  }

  bool isValidHashtag(std::string text) {
    return is_valid_hashtag(*validator, text);
  }

  bool isValidUrl(std::string text) {
    return is_valid_url(*validator, text);
  }

  bool isValidUrlWithoutProtocol(std::string text) {
    return is_valid_url_without_protocol(*validator, text);
  }

  int32_t getMaxTweetLength() {
    return get_max_tweet_length();
  }

  int32_t getShortUrlLength() {
    return get_short_url_length(*validator);
  }

  void setShortUrlLength(int32_t i) {
    return set_short_url_length(*validator, i);
  }

  int32_t getShortUrlLengthHttps() {
    return get_short_url_length_https(*validator);
  }

  void setShortUrlLengthHttps(int32_t i) {
    return set_short_url_length_https(*validator, i);
  }

private:
  ::rust::Box<RustValidator> validator;
};

class TwitterTextParser {
public:
  static TwitterTextParseResults parse(std::string text, TwitterTextConfiguration &ttc, bool parseUrls) {
    return parse_ffi(text, *ttc.config, parseUrls);
  }
};

} // twitter_text