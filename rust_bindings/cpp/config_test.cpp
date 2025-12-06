#include "twitter.h"
#include "gtest/gtest.h"

#include <string>
#include <fstream>
#include <streambuf>

namespace twitter_text {

TEST(TwitterTextConfigurationTest, Ctor) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_NE(config, nullptr);
  delete config;
}

TEST(TwitterTextConfigurationTest, Path) {
  auto config = TwitterTextConfiguration::configurationFromPath("rust_bindings/cpp/test_data/test_config.json");
  ASSERT_NE(config, nullptr);
  ASSERT_EQ(config->getVersion(), 42);
  ASSERT_EQ(config->getMaxWeightedTweetLength(), 400);
  ASSERT_EQ(config->getScale(), 43);
  ASSERT_EQ(config->getDefaultWeight(), 213);
  ASSERT_EQ(config->getTransformedUrlLength(), 32);
  std::vector<WeightedRange> stdv = config->getRanges();
  ASSERT_EQ(stdv.size(), 1);
  WeightedRange wr = stdv[0];
  ASSERT_EQ(wr.range.start, 0);
  ASSERT_EQ(wr.range.end, 4351);
  ASSERT_EQ(wr.weight, 200);
  delete config;
}

TEST(TwitterTextConfigurationTest, Json) {
  std::ifstream t("rust_bindings/cpp/test_data/test_config.json");
  std::string str((std::istreambuf_iterator<char>(t)),
                   std::istreambuf_iterator<char>());
  auto config = TwitterTextConfiguration::configurationFromJson(str);
  ASSERT_NE(config, nullptr);
  ASSERT_EQ(config->getVersion(), 42);
  ASSERT_EQ(config->getMaxWeightedTweetLength(), 400);
  ASSERT_EQ(config->getScale(), 43);
  ASSERT_EQ(config->getDefaultWeight(), 213);
  ASSERT_EQ(config->getTransformedUrlLength(), 32);
  std::vector<WeightedRange> stdv = config->getRanges();
  ASSERT_EQ(stdv.size(), 1);
  WeightedRange wr = stdv[0];
  ASSERT_EQ(wr.range.start, 0);
  ASSERT_EQ(wr.range.end, 4351);
  ASSERT_EQ(wr.weight, 200);
  delete config;
}

TEST(TwitterTextConfigurationTest, Version) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_EQ(config->getVersion(), 3);
  config->setVersion(199);
  ASSERT_EQ(config->getVersion(), 199);
  delete config;
}

TEST(TwitterTextConfigurationTest, MaxWeightedTweetLength) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_EQ(config->getMaxWeightedTweetLength(), 280);
  config->setMaxWeightedTweetLength(199);
  ASSERT_EQ(config->getMaxWeightedTweetLength(), 199);
  delete config;
}

TEST(TwitterTextConfigurationTest, Scale) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_EQ(config->getScale(), 100);
  config->setScale(199);
  ASSERT_EQ(config->getScale(), 199);
  delete config;
}

TEST(TwitterTextConfigurationTest, DefaultWeight) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_EQ(config->getDefaultWeight(), 200);
  config->setDefaultWeight(199);
  ASSERT_EQ(config->getDefaultWeight(), 199);
  delete config;
}

TEST(TwitterTextConfigurationTest, TransformedUrlLength) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_EQ(config->getTransformedUrlLength(), 23);
  config->setTransformedUrlLength(199);
  ASSERT_EQ(config->getTransformedUrlLength(), 199);
  delete config;
}

TEST(TwitterTextConfigurationTest, EmojiParsingEnabled) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  ASSERT_EQ(config->getEmojiParsingEnabled(), true);
  config->setEmojiParsingEnabled(false);
  ASSERT_EQ(config->getEmojiParsingEnabled(), false);
  delete config;
}

TEST(TwitterTextConfigurationTest, Ranges) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  std::vector<WeightedRange> stdv = config->getRanges();
  ASSERT_EQ(stdv.size(), 4);
  WeightedRange wr = stdv[0];
  ASSERT_EQ(wr.range.start, 0);
  ASSERT_EQ(wr.range.end, 4351);
  ASSERT_EQ(wr.weight, 100);
  wr = stdv[1];
  ASSERT_EQ(wr.range.start, 8192);
  ASSERT_EQ(wr.range.end, 8205);
  ASSERT_EQ(wr.weight, 100);
  wr = stdv[2];
  ASSERT_EQ(wr.range.start, 8208);
  ASSERT_EQ(wr.range.end, 8223);
  ASSERT_EQ(wr.weight, 100);
  wr = stdv[3];
  ASSERT_EQ(wr.range.start, 8242);
  ASSERT_EQ(wr.range.end, 8247);
  ASSERT_EQ(wr.weight, 100);
  delete config;
}

TEST(TwitterTextConfigurationTest, V2) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration(config_v2());
  ASSERT_EQ(config->getVersion(), 2);
  ASSERT_EQ(config->getEmojiParsingEnabled(), false);
  std::vector<WeightedRange> stdv = config->getRanges();
  ASSERT_EQ(stdv.size(), 4);
  delete config;
}

TEST(TwitterTextConfigurationTest, V1) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration(config_v1());
  ASSERT_EQ(config->getVersion(), 1);
  ASSERT_EQ(config->getEmojiParsingEnabled(), false);
  std::vector<WeightedRange> stdv = config->getRanges();
  ASSERT_EQ(stdv.size(), 0);
  delete config;
}

} // twitter_text
