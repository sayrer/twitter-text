#include "twitter.h"
#include <iostream>
#include <memory>

namespace twitter_text {

// TwitterTextConfiguration
template<>
std::vector<ffi::WeightedRange> 
TwitterTextConfiguration<std::vector<ffi::WeightedRange>, std::string>::getRanges() {
  std::vector<ffi::WeightedRange> stdv;
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

template<>
std::unique_ptr<MentionResult>
ValidatingExtractor<std::vector<ffi::WeightedRange>, std::string>::extractReplyScreenname(std::string &text) {
  return ffi::extract_reply_username_validated(*extractor, text);
}

template<>
std::unique_ptr<ExtractResult>
ValidatingExtractor<std::vector<ffi::WeightedRange>, std::string>::convertResult(ffi::ExtractResult &result) {
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