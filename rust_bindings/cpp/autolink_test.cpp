#include "twitter.h"
#include "gtest/gtest.h"
#include "yaml-cpp/yaml.h"

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
 
  ASSERT_EQ(autolinker->getUrlClass(), "");
  autolinker->setUrlClass("foo");
  ASSERT_EQ(autolinker->getUrlClass(), "foo");

  ASSERT_EQ(autolinker->getUrlTarget(), "");
  autolinker->setUrlTarget("bar");
  ASSERT_EQ(autolinker->getUrlTarget(), "bar");

  ASSERT_EQ(autolinker->getSymbolTag(), "");
  autolinker->setSymbolTag("baz");
  ASSERT_EQ(autolinker->getSymbolTag(), "baz");

  ASSERT_EQ(autolinker->getTextWithSymbolTag(), "");
  autolinker->setTextWithSymbolTag("qux");
  ASSERT_EQ(autolinker->getTextWithSymbolTag(), "qux");

  ASSERT_EQ(autolinker->getListClass(), "tweet-url list-slug");
  autolinker->setListClass("tweet-url list-slug foo");
  ASSERT_EQ(autolinker->getListClass(), "tweet-url list-slug foo");

  ASSERT_EQ(autolinker->getUsernameClass(), "tweet-url username");
  autolinker->setUsernameClass("tweet-url username bar");
  ASSERT_EQ(autolinker->getUsernameClass(), "tweet-url username bar");

  ASSERT_EQ(autolinker->getHashtagClass(), "tweet-url hashtag");
  autolinker->setHashtagClass("tweet-url hashtag baz");
  ASSERT_EQ(autolinker->getHashtagClass(), "tweet-url hashtag baz");

  ASSERT_EQ(autolinker->getCashtagClass(), "tweet-url cashtag");
  autolinker->setCashtagClass("tweet-url cashtag qux");
  ASSERT_EQ(autolinker->getCashtagClass(), "tweet-url cashtag qux");

  ASSERT_EQ(autolinker->getUsernameUrlBase(), "https://twitter.com/");
  autolinker->setUsernameUrlBase("https://example.com/");
  ASSERT_EQ(autolinker->getUsernameUrlBase(), "https://example.com/");

  ASSERT_EQ(autolinker->getListUrlBase(), "https://twitter.com/");
  autolinker->setListUrlBase("https://example.com/");
  ASSERT_EQ(autolinker->getListUrlBase(), "https://example.com/");

  ASSERT_EQ(autolinker->getHashtagUrlBase(), "https://twitter.com/search?q=%23");
  autolinker->setHashtagUrlBase("https://example.com/search?q=%23");
  ASSERT_EQ(autolinker->getHashtagUrlBase(), "https://example.com/search?q=%23");

  ASSERT_EQ(autolinker->getCashtagUrlBase(), "https://twitter.com/search?q=%24");
  autolinker->setCashtagUrlBase("https://example.com/search?q=%24");
  ASSERT_EQ(autolinker->getCashtagUrlBase(), "https://example.com/search?q=%24");

  ASSERT_EQ(autolinker->getInvisibleTagAttrs(), "style='position:absolute;left:-9999px;'");
  autolinker->setInvisibleTagAttrs("");
  ASSERT_EQ(autolinker->getInvisibleTagAttrs(), "");

  ASSERT_EQ(autolinker->getUsernameIncludeSymbol(), false);
  autolinker->setUsernameIncludeSymbol(true);
  ASSERT_EQ(autolinker->getUsernameIncludeSymbol(), true);

  delete autolinker;
}

} // twitter_text