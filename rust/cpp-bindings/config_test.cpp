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
  auto config = TwitterTextConfiguration::configurationFromPath("rust/cpp-bindings/test_data/test_config.json");
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
}

TEST(TwitterTextConfigurationTest, Json) {
  std::ifstream t("rust/cpp-bindings/test_data/test_config.json");
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
}

TEST(TwitterTextConfigurationTest, Version) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration();
  // The default configuration is v3; v4 is opt-in.
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
  // The default configuration is v3; v4 is opt-in.
  ASSERT_EQ(config->getVersion(), 3);
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

TEST(TwitterTextConfigurationTest, V4) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration(config_v4());
  ASSERT_EQ(config->getVersion(), 4);
  ASSERT_EQ(config->getEmojiParsingEnabled(), true);
  std::vector<WeightedRange> stdv = config->getRanges();
  TwitterTextConfiguration *v3 = new TwitterTextConfiguration(config_v3());
  std::vector<WeightedRange> v3_ranges = v3->getRanges();
  // v4 is v3 plus the alphabetic scripts above U+10FF.
  ASSERT_GT(stdv.size(), v3_ranges.size());
  for (const WeightedRange &v3_range : v3_ranges) {
    bool found = false;
    for (const WeightedRange &r : stdv) {
      found |= r.range.start == v3_range.range.start &&
               r.range.end == v3_range.range.end && r.weight == v3_range.weight;
    }
    ASSERT_TRUE(found) << "v3 range " << v3_range.range.start << ".." << v3_range.range.end;
  }
  auto covers = [&stdv](int32_t first, int32_t last) {
    for (const WeightedRange &r : stdv) {
      if (r.range.start <= first && r.range.end >= last && r.weight == 100) {
        return true;
      }
    }
    return false;
  };
  ASSERT_TRUE(covers(0x16A0, 0x16FF));     // Runic
  ASSERT_TRUE(covers(0x1200, 0x137F));     // Ethiopic
  ASSERT_TRUE(covers(0x1780, 0x17FF));     // Khmer
  ASSERT_TRUE(covers(0x1E00, 0x1EFF));     // Latin Extended Additional
  ASSERT_TRUE(covers(0x1F00, 0x1FFF));     // Greek Extended
  ASSERT_TRUE(covers(0xFB50, 0xFDFF));     // Arabic Presentation Forms-A
  ASSERT_TRUE(covers(0x1E900, 0x1E95F));   // Adlam
  ASSERT_FALSE(covers(0x4E00, 0x9FFF));    // CJK Unified Ideographs
  ASSERT_FALSE(covers(0xAC00, 0xD7AF));    // Hangul Syllables
  ASSERT_FALSE(covers(0x1D400, 0x1D7FF));  // Mathematical Alphanumeric Symbols
  delete v3;
  delete config;
}

TEST(TwitterTextConfigurationTest, V3) {
  TwitterTextConfiguration *config = new TwitterTextConfiguration(config_v3());
  ASSERT_EQ(config->getVersion(), 3);
  ASSERT_EQ(config->getEmojiParsingEnabled(), true);
  ASSERT_EQ(config->getRanges().size(), 4);
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
