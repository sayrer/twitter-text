#include "twitter.h"
#include <iostream>
#include <memory>

namespace twitter_text {

// Autolinker
bool
Autolinker::getNoFollow() {
  return config->no_follow;
}

void 
Autolinker::setNoFollow(bool noFollow) {
  config->no_follow = noFollow;
}

::rust::String
Autolinker::getUrlClass() {
  return config->url_class;
}

void
Autolinker::setUrlClass(std::string urlClass) {
  config->url_class = ::rust::String(urlClass);
}

::rust::String
Autolinker::getUrlTarget() {
  return config->url_target;
}

void
Autolinker::setUrlTarget(std::string urlTarget) {
  config->url_target = ::rust::String(urlTarget);
}

::rust::String
Autolinker::getSymbolTag() {
  return config->symbol_tag;
}

void
Autolinker::setSymbolTag(std::string symbolTag) {
  config->symbol_tag = ::rust::String(symbolTag);
}

::rust::String
Autolinker::getTextWithSymbolTag() {
  return config->text_with_symbol_tag; 
}

void
Autolinker::setTextWithSymbolTag(std::string textWithSymbolTag) {
  config->text_with_symbol_tag = ::rust::String(textWithSymbolTag);
}

::rust::String
Autolinker::getListClass() {
  return std::string(config->list_class); 
}

void
Autolinker::setListClass(std::string listClass) {
  config->list_class = rust::String(listClass);
}

::rust::String
Autolinker::getUsernameClass() {
  return std::string(config->username_class);
}

void
Autolinker::setUsernameClass(std::string usernameClass) {
  config->username_class = ::rust::String(usernameClass);
}

::rust::String
Autolinker::getHashtagClass() {
  return config->hashtag_class;
}

void
Autolinker::setHashtagClass(std::string hashtagClass) {
  config->hashtag_class = ::rust::String(hashtagClass);
}

::rust::String
Autolinker::getCashtagClass() {
  return config->cashtag_class;  
}

void
Autolinker::setCashtagClass(std::string cashtagClass) {
  config->cashtag_class = ::rust::String(cashtagClass);
}

::rust::String
Autolinker::getUsernameUrlBase() {
  return config->username_url_base;  
}

void
Autolinker::setUsernameUrlBase(std::string usernameUrlBase) {
  config->username_url_base = ::rust::String(usernameUrlBase);
}

::rust::String
Autolinker::getListUrlBase() {
  return config->list_url_base;
}

void
Autolinker::setListUrlBase(std::string listUrlBase) {
  config->list_url_base = ::rust::String(listUrlBase);
}

::rust::String
Autolinker::getHashtagUrlBase() {
  return config->hashtag_url_base;  
}

void
Autolinker::setHashtagUrlBase(std::string hashtagUrlBase) {
  config->hashtag_url_base = ::rust::String(hashtagUrlBase);
}

::rust::String
Autolinker::getCashtagUrlBase() {
  return config->cashtag_url_base;
}

void
Autolinker::setCashtagUrlBase(std::string cashtagUrlBase) {
  config->cashtag_url_base = ::rust::String(cashtagUrlBase);
}

::rust::String
Autolinker::getInvisibleTagAttrs() {
  return config->invisible_tag_attrs;
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

::rust::String
Autolinker::autolink(const std::string &text) {
  return ffi::autolink(text, *config);
}

::rust::String
Autolinker::autolinkUsernamesAndLists(const std::string &text) {
  return ffi::autolink_usernames_and_lists(text, *config);
}

::rust::String
Autolinker::autolinkHashtags(const std::string &text) {
  return ffi::autolink_hashtags(text, *config);
}

::rust::String
Autolinker::autolinkUrls(const std::string &text) {
  return ffi::autolink_urls(text, *config);
}

::rust::String
Autolinker::autolinkCashtags(const std::string &text) { 
  return ffi::autolink_cashtags(text, *config);
}

// Extractor

std::vector<std::string>
Extractor::extractorStringsToCpp(::rust::Vec<ffi::ExtractorString> &rustVec) {
  std::vector<std::string> stdv;
  stdv.reserve(rustVec.size());
  for (ffi::ExtractorString es : rustVec) {
    stdv.push_back(std::string(es.s));
  }
  return stdv;
}

// ValidatingExtractor

std::shared_ptr<MentionResult>
ValidatingExtractor::extractReplyScreenname(std::string &text) {
  return ffi::extract_reply_username_validated(*extractor, text);
}

std::vector<Entity>
ValidatingExtractor::entitiesToCpp(::rust::Vec<Entity> &rustVec) {
  std::vector<Entity> stdv;
  stdv.reserve(rustVec.size());
  std::copy(rustVec.begin(), rustVec.end(), std::back_inserter(stdv));
  return stdv;
}

std::unique_ptr<ExtractResult>
ValidatingExtractor::convertResult(ffi::ExtractResult &result) {
  std::unique_ptr<ExtractResult> er(new ExtractResult());
  er->parseResults = result.parse_results;
  er->entities = entitiesToCpp(result.entities);
  return er;
}

// HitHighlighter
std::string
HitHighlighter::highlight(std::string &text, std::vector<Hit> &hits) {
  return std::string(ffi::hit_highlight(*highlighter, text, hits));
}

} // twitter_text