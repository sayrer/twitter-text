#include "twitter.h"
#include "gtest/gtest.h"
#include "test_helpers.h"

namespace twitter_text {

TEST(AutolinkTest, Ctor) {
  Autolinker *autolinker = new Autolinker();
  ASSERT_NE(autolinker, nullptr);
  delete autolinker;
}

TEST(AutolinkTest, CtorWithConfig) {
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

} // twitter_text