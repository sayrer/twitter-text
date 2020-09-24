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
  return ::twitter_text::autolink_all(text, *config);
}

::rust::String
Autolinker::autolinkUsernamesAndLists(const std::string &text) {
  return ::twitter_text::autolink_usernames_and_lists(text, *config);
}

::rust::String
Autolinker::autolinkHashtags(const std::string &text) {
  return ::twitter_text::autolink_hashtags(text, *config);
}

::rust::String
Autolinker::autolinkUrls(const std::string &text) {
  return ::twitter_text::autolink_urls(text, *config);
}

::rust::String
Autolinker::autolinkCashtags(const std::string &text) { 
  return ::twitter_text::autolink_cashtags(text, *config);
}

// Extractor

template<typename T>
std::vector<T> vecToCpp(::rust::Vec<T> &rustVec) {
  std::vector<T> stdv;
  stdv.reserve(rustVec.size());
  std::copy(rustVec.begin(), rustVec.end(), std::back_inserter(stdv));
  return stdv;
}

template <>
::rust::Vec<Entity> 
Extractor<>::extractEntitiesWithIndices(std::string text) {
  return extract_entities_with_indices(*extractor, text);
}

template <>
std::vector<Entity>
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractEntitiesWithIndices(std::string text) {
  auto vec = extract_entities_with_indices(*extractor, text);
  return vecToCpp(vec);
}

template <>
::rust::Vec<::rust::String> 
Extractor<>::extractMentionedScreennames(std::string text) {
  return extract_mentioned_screennames(*extractor, text);
}

template <>
std::vector<::rust::String> 
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractMentionedScreennames(std::string text) {
  auto vec = extract_mentioned_screennames(*extractor, text);
  return vecToCpp(vec);
}

template <>
::rust::Vec<Entity> 
Extractor<>::extractMentionedScreennamesWithIndices(std::string text) {
  return extract_mentioned_screennames_with_indices(*extractor, text);
}

template <>
std::vector<Entity>
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractMentionedScreennamesWithIndices(std::string text) {
  auto vec = extract_mentioned_screennames_with_indices(*extractor, text);
  return vecToCpp(vec);
}

template <>
::rust::Vec<Entity> 
Extractor<>::extractMentionsOrListsWithIndices(std::string text) {
  return extract_mentions_or_lists_with_indices(*extractor, text);
}

template <>
std::vector<Entity> 
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractMentionsOrListsWithIndices(std::string text) {
  auto vec = extract_mentions_or_lists_with_indices(*extractor, text);
  return vecToCpp(vec);
}

template <>
std::shared_ptr<Entity>
Extractor<>::extractReplyScreenname(std::string text) {
  return extract_reply_username(*extractor, text);
}

template<>
::rust::Vec<::rust::String>
Extractor<>::extractUrls(std::string text) {
    return extract_urls(*extractor, text);
}

template <>
std::vector<::rust::String> 
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractUrls(std::string text) {
  auto vec = extract_urls(*extractor, text);
  return vecToCpp(vec);
}

template <>
Entity*
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractReplyScreenname(std::string text) {
  return extract_reply_username(*extractor, text).release();
}

template <>
::rust::Vec<Entity> 
Extractor<>::extractUrlsWithIndices(std::string text) {
  return extract_urls_with_indices(*extractor, text);
}

template <>
std::vector<Entity> 
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractUrlsWithIndices(std::string text) {
  auto vec = extract_urls_with_indices(*extractor, text);
  return vecToCpp(vec);
}

template<>
::rust::Vec<::rust::String>
Extractor<>::extractHashtags(std::string text) {
    return extract_hashtags(*extractor, text);
}

template <>
std::vector<::rust::String> 
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractHashtags(std::string text) {
  auto vec = extract_hashtags(*extractor, text);
  return vecToCpp(vec);
}

template <>
::rust::Vec<Entity>
Extractor<>::extractHashtagsWithIndices(std::string text) {
  return extract_hashtags_with_indices(*extractor, text);
}

template <>
std::vector<Entity>
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractHashtagsWithIndices(std::string text) {
  auto vec = extract_hashtags_with_indices(*extractor, text);
  return vecToCpp(vec);
}

template<>
::rust::Vec<::rust::String>
Extractor<>::extractCashtags(std::string text) {
    return extract_cashtags(*extractor, text);
}

template <>
std::vector<::rust::String>
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractCashtags(std::string text) {
  auto vec = extract_cashtags(*extractor, text);
  return vecToCpp(vec);
}

template <>
::rust::Vec<Entity>
Extractor<>::extractCashtagsWithIndices(std::string text) {
  return extract_cashtags_with_indices(*extractor, text);
}

template <>
std::vector<Entity>
Extractor<std::vector<Entity>, Entity*, std::vector<::rust::String>>::extractCashtagsWithIndices(std::string text) {
  auto vec = extract_cashtags_with_indices(*extractor, text);
  return vecToCpp(vec);
}

// ValidatingExtractor

template <>
std::unique_ptr<ExtractResult>
ValidatingExtractor<>::extractEntitiesWithIndices(const std::string &text) {
  return extract_entities_with_indices_validated(*extractor, text);
}

template <>
SwigExtractResult*
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractEntitiesWithIndices(const std::string &text) {
  auto result = extract_entities_with_indices_validated(*extractor, text);
  return convertResult(*result);
}

template <>
std::unique_ptr<ExtractResult>
ValidatingExtractor<>::extractMentionedScreennamesWithIndices(const std::string &text) {
  return extract_mentioned_screennames_with_indices_validated(*extractor, text);
}

template <>
SwigExtractResult* 
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractMentionedScreennamesWithIndices(const std::string &text) {
  auto result = extract_mentioned_screennames_with_indices_validated(*extractor, text);
  return convertResult(*result);
}

template <>
std::unique_ptr<ExtractResult>
ValidatingExtractor<>::extractMentionsOrListsWithIndices(const std::string &text) {
  return extract_mentions_or_lists_with_indices_validated(*extractor, text);
}

template <>
SwigExtractResult* 
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractMentionsOrListsWithIndices(const std::string &text) {
  auto result = extract_mentions_or_lists_with_indices_validated(*extractor, text);
  return convertResult(*result);
}

template <>
std::unique_ptr<MentionResult>
ValidatingExtractor<>::extractReplyScreenname(const std::string &text) {
  return extract_reply_username_validated(*extractor, text);
}

template <>
SwigMentionResult* 
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractReplyScreenname(const std::string &text) {
  auto result = extract_reply_username_validated(*extractor, text);
  auto swmr = new SwigMentionResult();
  swmr->parseResults = result->parse_results;
  swmr->entity = result->mention.release();
  return swmr;
}

template <>
std::unique_ptr<ExtractResult>
ValidatingExtractor<>::extractUrlsWithIndices(const std::string &text) {
  return extract_urls_with_indices_validated(*extractor, text);
}

template <>
SwigExtractResult* 
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractUrlsWithIndices(const std::string &text) {
  auto result = extract_urls_with_indices_validated(*extractor, text);
  return convertResult(*result);
}

template <>
std::unique_ptr<ExtractResult>
ValidatingExtractor<>::extractHashtagsWithIndices(const std::string &text) {
  return extract_hashtags_with_indices_validated(*extractor, text);
}

template <>
SwigExtractResult* 
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractHashtagsWithIndices(const std::string &text) {
  auto result = extract_hashtags_with_indices_validated(*extractor, text);
  return convertResult(*result);
}

template <>
std::unique_ptr<ExtractResult>
ValidatingExtractor<>::extractCashtagsWithIndices(const std::string &text) {
  return extract_cashtags_with_indices_validated(*extractor, text);
}

template <>
SwigExtractResult* 
ValidatingExtractor<SwigExtractResult*, SwigMentionResult*>::extractCashtagsWithIndices(const std::string &text) {
  auto result = extract_cashtags_with_indices_validated(*extractor, text);
  return convertResult(*result);
}

} // twitter_text