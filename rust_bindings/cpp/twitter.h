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
  static std::unique_ptr<TwitterTextConfiguration> configuration_from_path(std::string path);
  static std::unique_ptr<TwitterTextConfiguration> configuration_from_json(std::string json);

  int32_t getVersion();
  void setVersion(int32_t version);

  int32_t getMaxWeightedTweetLength();
  void setMaxWeightedTweetLength(int32_t max);

  int32_t getScale();
  void setScale(int32_t scale);

  int32_t getDefaultWeight();
  void setDefaultWeight(int32_t weight);

  int32_t getTransformedUrlLength();
  void setTransformedUrlLength(int32_t length);

  bool getEmojiParsingEnabled();
  void setEmojiParsingEnabled(bool enabled);

  std::vector<ffi::WeightedRange> getRanges();

private:
  std::unique_ptr<ffi::Configuration> config;
};

class Autolinker {
public:
  Autolinker(): 
    config(ffi::autolink_default_config()) 
  {}

  bool getNoFollow();
  void setNoFollow(bool noFollow);

  std::string getUrlClass();
  void setUrlClass(std::string urlClass);

  std::string getUrlTarget();
  void setUrlTarget(std::string urlTarget);

  std::string getSymbolTag();
  void setSymbolTag(std::string symbolTag);

  std::string getTextWithSymbolTag();
  void setTextWithSymbolTag(std::string textWithSymbolTag);

  std::string getListClass();
  void setListClass(std::string listClass);

  std::string getUsernameClass();
  void setUsernameClass(std::string usernameClass);

  std::string getHashtagClass();
  void setHashtagClass(std::string hashtagClass);

  std::string getCashtagClass();
  void setCashtagClass(std::string cashtagClass);

  std::string getUsernameUrlBase();
  void setUsernameUrlBase(std::string usernameUrlBase);

  std::string getListUrlBase();
  void setListUrlBase(std::string listUrlBase);

  std::string getHashtagUrlBase();
  void setHashtagUrlBase(std::string hashtagUrlBase);

  std::string getCashtagUrlBase();
  void setCashtagUrlBase(std::string cashtagUrlBase);

  std::string getInvisibleTagAttrs();
  void setInvisibleTagAttrs(std::string invisibleTagAttrs);

  bool getUsernameIncludeSymbol();
  void setUsernameIncludeSymbol(bool usernameIncludeSymbol);

  std::string autolink(std::string &text);
  std::string autolinkUsernamesAndLists(std::string &text);
  std::string autolinkHashtags(std::string &text);
  std::string autolinkUrls(std::string &text);
  std::string autolinkCashtags(std::string &text);

private:
  std::unique_ptr<ffi::AutolinkerConfig> config;
};

typedef ffi::Entity Entity;

class Extractor {
public:
  Extractor():
    extractor(ffi::make_extractor()) 
  {}

  bool get_extract_url_without_protocol();
  void set_extract_url_without_protocol(bool extractUrlwp);
  std::vector<Entity> extractEntitiesWithIndices(std::string &text);
  std::vector<std::string> extractMentionedScreennames(std::string &text);
  std::vector<Entity> extractMentionedScreennamesWithIndices(std::string &text);
  std::vector<Entity> extractMentionsOrListsWithIndices(std::string &text);
  std::vector<Entity> extractReplyScreenname(std::string &text);
  std::vector<std::string> extractURLs(std::string &text);
  std::vector<Entity> extractURLsWithIndices(std::string &text);
  std::vector<std::string> extractHashtags(std::string &text);
  std::vector<Entity> extractHashtagsWithIndices(std::string &text);
  std::vector<std::string> extractCashtags(std::string &text);
  std::vector<Entity> extractCashtagsWithIndices(std::string &text);

private:
  ::rust::Box<ffi::Extractor> extractor;
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

  bool isValidTweet(std::string &text);
  bool isValidUsername(std::string &text);
  bool isValidList(std::string &text);
  bool isValidHashtag(std::string &text);
  bool isValidUrl(std::string &text);
  bool isValidUrlWithoutProtocol(std::string &text);

  int32_t getMaxTweetLength();
  int32_t getShortUrlLength();
  void setShortUrlLength(int32_t i);
  int32_t getShortUrlLengthHttps();
  void setShortUrlLengthHttps(int32_t i);

private:
  ::rust::Box<ffi::Validator> validator;
}; 

} // twitter_text

