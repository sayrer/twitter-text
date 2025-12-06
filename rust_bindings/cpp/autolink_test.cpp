#include "twitter.h"
#include "twitter_text_c.h"
#include "twitter_text_modifiers.h"
#include "gtest/gtest.h"
#include "test_helpers.h"

namespace twitter_text {

TEST(AutolinkTest, Ctor) {
  Autolinker *autolinker = new Autolinker();
  ASSERT_NE(autolinker, nullptr);
  delete autolinker;
}

TEST(AutolinkTest, Accessor) {
  Autolinker *autolinker = new Autolinker();

  ASSERT_EQ(autolinker->getNoFollow(), false);
  autolinker->setNoFollow(true);
  ASSERT_EQ(autolinker->getNoFollow(), true);
 
  ASSERT_EQ(std::string(autolinker->getUrlClass()), "");
  autolinker->setUrlClass("foo");
  ASSERT_EQ(std::string(autolinker->getUrlClass()), "foo");

  ASSERT_EQ(std::string(autolinker->getUrlTarget()), "");
  autolinker->setUrlTarget("bar");
  ASSERT_EQ(std::string(autolinker->getUrlTarget()), "bar");

  ASSERT_EQ(std::string(autolinker->getSymbolTag()), "");
  autolinker->setSymbolTag("baz");
  ASSERT_EQ(std::string(autolinker->getSymbolTag()), "baz");

  ASSERT_EQ(std::string(autolinker->getTextWithSymbolTag()), "");
  autolinker->setTextWithSymbolTag("qux");
  ASSERT_EQ(std::string(autolinker->getTextWithSymbolTag()), "qux");

  ASSERT_EQ(std::string(autolinker->getListClass()), "tweet-url list-slug");
  autolinker->setListClass("tweet-url list-slug foo");
  ASSERT_EQ(std::string(autolinker->getListClass()), "tweet-url list-slug foo");

  ASSERT_EQ(std::string(autolinker->getUsernameClass()), "tweet-url username");
  autolinker->setUsernameClass("tweet-url username bar");
  ASSERT_EQ(std::string(autolinker->getUsernameClass()), "tweet-url username bar");

  ASSERT_EQ(std::string(autolinker->getHashtagClass()), "tweet-url hashtag");
  autolinker->setHashtagClass("tweet-url hashtag baz");
  ASSERT_EQ(std::string(autolinker->getHashtagClass()), "tweet-url hashtag baz");

  ASSERT_EQ(std::string(autolinker->getCashtagClass()), "tweet-url cashtag");
  autolinker->setCashtagClass("tweet-url cashtag qux");
  ASSERT_EQ(std::string(autolinker->getCashtagClass()), "tweet-url cashtag qux");

  ASSERT_EQ(std::string(autolinker->getUsernameUrlBase()), "https://twitter.com/");
  autolinker->setUsernameUrlBase("https://example.com/");
  ASSERT_EQ(std::string(autolinker->getUsernameUrlBase()), "https://example.com/");

  ASSERT_EQ(std::string(autolinker->getListUrlBase()), "https://twitter.com/");
  autolinker->setListUrlBase("https://example.com/");
  ASSERT_EQ(std::string(autolinker->getListUrlBase()), "https://example.com/");

  ASSERT_EQ(std::string(autolinker->getHashtagUrlBase()), "https://twitter.com/search?q=%23");
  autolinker->setHashtagUrlBase("https://example.com/search?q=%23");
  ASSERT_EQ(std::string(autolinker->getHashtagUrlBase()), "https://example.com/search?q=%23");

  ASSERT_EQ(std::string(autolinker->getCashtagUrlBase()), "https://twitter.com/search?q=%24");
  autolinker->setCashtagUrlBase("https://example.com/search?q=%24");
  ASSERT_EQ(std::string(autolinker->getCashtagUrlBase()), "https://example.com/search?q=%24");

  ASSERT_EQ(std::string(autolinker->getInvisibleTagAttrs()), "style='position:absolute;left:-9999px;'");
  autolinker->setInvisibleTagAttrs("");
  ASSERT_EQ(std::string(autolinker->getInvisibleTagAttrs()), "");

  ASSERT_EQ(autolinker->getUsernameIncludeSymbol(), false);
  autolinker->setUsernameIncludeSymbol(true);
  ASSERT_EQ(autolinker->getUsernameIncludeSymbol(), true);

  delete autolinker;
}

TEST(AutolinkTest, Yaml) {
  Autolinker *autolinker = new Autolinker();
  YAML::Node map = YAML::LoadFile("rust/conformance/tests/autolink.yml");

  auto usernames = readYaml<TestCase>(map["tests"]["usernames"]);
  for (TestCase test : usernames) {
    ASSERT_EQ(test.expected, std::string(autolinker->autolinkUsernamesAndLists(test.text)));
  }

  auto lists = readYaml<TestCase>(map["tests"]["lists"]);
  for (TestCase test : lists) {
    ASSERT_EQ(test.expected, std::string(autolinker->autolinkUsernamesAndLists(test.text)));
  }

  auto hashtags = readYaml<TestCase>(map["tests"]["hashtags"]);
  for (TestCase test : hashtags) {
    ASSERT_EQ(test.expected, std::string(autolinker->autolinkHashtags(test.text)));
  }

  auto urls = readYaml<TestCase>(map["tests"]["urls"]);
  for (TestCase test : urls) {
    ASSERT_EQ(test.expected, std::string(autolinker->autolinkUrls(test.text)));
  }

  auto cashtags = readYaml<TestCase>(map["tests"]["cashtags"]);
  for (TestCase test : cashtags) {
    ASSERT_EQ(test.expected, std::string(autolinker->autolinkCashtags(test.text)));
  }

  auto all = readYaml<TestCase>(map["tests"]["all"]);
  for (TestCase test : all) {
    ASSERT_EQ(test.expected, std::string(autolinker->autolink(test.text)));
  }

  delete autolinker;
}

TEST(AutolinkTest, AddAttributeModifierCAPI) {
  // Test using the C API for LinkAttributeModifier
  TwitterTextAutolinker* autolinker = twitter_text_autolinker_new(false);
  ASSERT_NE(autolinker, nullptr);

  // Create a modifier that adds data-custom="test" to hashtags
  TwitterTextEntityType types[] = {TWITTER_TEXT_ENTITY_HASHTAG};
  TwitterTextAddAttributeModifier* modifier = twitter_text_add_attribute_modifier_new(
      types, 1, "data-custom", "test");
  ASSERT_NE(modifier, nullptr);

  twitter_text_autolinker_set_add_attribute_modifier(autolinker, modifier);

  char* result = twitter_text_autolinker_autolink(autolinker, "#test @mention");
  ASSERT_NE(result, nullptr);
  
  std::string result_str(result);
  ASSERT_TRUE(result_str.find("data-custom=\"test\"") != std::string::npos);
  ASSERT_TRUE(result_str.find("#test") != std::string::npos);
  
  twitter_text_string_free(result);
  twitter_text_add_attribute_modifier_free(modifier);
  twitter_text_autolinker_free(autolinker);
}

TEST(AutolinkTest, ReplaceClassModifierCAPI) {
  // Test using the C API for ReplaceClassModifier
  TwitterTextAutolinker* autolinker = twitter_text_autolinker_new(false);
  ASSERT_NE(autolinker, nullptr);

  TwitterTextReplaceClassModifier* modifier = 
      twitter_text_replace_class_modifier_new("custom-link");
  ASSERT_NE(modifier, nullptr);

  twitter_text_autolinker_set_replace_class_modifier(autolinker, modifier);

  char* result = twitter_text_autolinker_autolink_hashtags(autolinker, "#test");
  ASSERT_NE(result, nullptr);
  
  std::string result_str(result);
  ASSERT_TRUE(result_str.find("class=\"custom-link\"") != std::string::npos);
  ASSERT_TRUE(result_str.find("tweet-url hashtag") == std::string::npos);
  
  twitter_text_string_free(result);
  twitter_text_replace_class_modifier_free(modifier);
  twitter_text_autolinker_free(autolinker);
}

// Callback function for link text modifier test
char* link_text_modifier_callback(const TwitterTextCEntity* entity, const char* text, void* user_data) {
  if (entity == nullptr || text == nullptr) {
    return nullptr;
  }
  
  std::string result;
  if (entity->entity_type == TWITTER_TEXT_ENTITY_HASHTAG) {
    result = "#replaced";
  } else {
    result = std::string("pre_") + text + "_post";
  }
  
  // Allocate and return a new C string
  char* output = (char*)malloc(result.length() + 1);
  if (output != nullptr) {
    strcpy(output, result.c_str());
  }
  return output;
}

TEST(AutolinkTest, LinkTextModifierCAPI) {
  // Test using the C API for LinkTextModifier
  TwitterTextAutolinker* autolinker = twitter_text_autolinker_new(false);
  ASSERT_NE(autolinker, nullptr);

  // Set the link text modifier callback
  twitter_text_autolinker_set_link_text_modifier(autolinker, link_text_modifier_callback, nullptr);

  char* result = twitter_text_autolinker_autolink(autolinker, "#hash @mention");
  ASSERT_NE(result, nullptr);
  
  std::string result_str(result);
  ASSERT_TRUE(result_str.find("#replaced") != std::string::npos);
  ASSERT_TRUE(result_str.find("pre_mention_post") != std::string::npos);
  
  twitter_text_string_free(result);
  twitter_text_autolinker_free(autolinker);
}

// Callback function for link text modifier that adds asterisks
char* link_text_modifier_asterisks_callback(const TwitterTextCEntity* entity, const char* text, void* user_data) {
  if (entity == nullptr || text == nullptr) {
    return nullptr;
  }
  
  std::string result = std::string("**") + text + "**";
  
  // Allocate and return a new C string
  char* output = (char*)malloc(result.length() + 1);
  if (output != nullptr) {
    strcpy(output, result.c_str());
  }
  return output;
}

TEST(AutolinkTest, LinkTextModifierAsterisksCAPI) {
  // Test using the C API for LinkTextModifier with asterisks
  TwitterTextAutolinker* autolinker = twitter_text_autolinker_new(false);
  ASSERT_NE(autolinker, nullptr);

  // Set the link text modifier callback
  twitter_text_autolinker_set_link_text_modifier(autolinker, link_text_modifier_asterisks_callback, nullptr);

  char* result = twitter_text_autolinker_autolink(autolinker, "#hash @mention");
  ASSERT_NE(result, nullptr);
  
  std::string result_str(result);
  ASSERT_TRUE(result_str.find("**#hash**") != std::string::npos);
  ASSERT_TRUE(result_str.find("**mention**") != std::string::npos);
  
  twitter_text_string_free(result);
  twitter_text_autolinker_free(autolinker);
}

} // twitter_text