#include "twitter.h"
#include "gtest/gtest.h"
#include "test_helpers.h"
#include "yaml-cpp/yaml.h"

// TODO Weighted Tweets
struct WeightedTweetExpectation {
	int32_t weightedLength;
	bool valid;
	int32_t permillage;
	int32_t displayRangeStart;
	int32_t displayRangeEnd;
	int32_t validRangeStart;
	int32_t validRangeEnd;
};

struct WeightedTweetTestCase {
	std::string description;
	std::string text;
	WeightedTweetExpectation expected;
};

namespace YAML {
template<>
struct convert<WeightedTweetExpectation> {
  static Node encode(const WeightedTweetExpectation& rhs) {
    Node node;
    node["weightedLength"] = rhs.weightedLength;
    node["valid"] = rhs.valid;
    node["permillage"] = rhs.permillage;
    node["displayRangeStart"] = rhs.displayRangeStart;
    node["displayRangeEnd"] = rhs.displayRangeEnd;
    node["validRangeStart"] = rhs.validRangeStart;
    node["validRangeEnd"] = rhs.validRangeEnd;

    return node;
  }

  static bool decode(const Node& node, WeightedTweetExpectation& rhs) {
    rhs.weightedLength = node["weightedLength"].as<int32_t>();
    rhs.valid = node["valid"].as<bool>();
    rhs.permillage = node["permillage"].as<int32_t>();
    rhs.displayRangeStart = node["displayRangeStart"].as<int32_t>();
    rhs.displayRangeEnd = node["displayRangeEnd"].as<int32_t>();
    rhs.validRangeStart = node["validRangeStart"].as<int32_t>();
    rhs.validRangeEnd = node["validRangeEnd"].as<int32_t>();

    return true;
  }
};

template<>
struct convert<WeightedTweetTestCase> {
  static Node encode(const WeightedTweetTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, WeightedTweetTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<WeightedTweetExpectation>();

    return true;
  }
};
} // namespace YAML

namespace twitter_text {

TEST(ValidatorTest, Ctor) {
  Validator *validator = new Validator();
  ASSERT_NE(validator, nullptr);
  delete validator;
}

TEST(ValidatorTest, Accessors) {
  Validator *validator = new Validator();
  ASSERT_NE(validator, nullptr);

  int32_t mtl = validator->getMaxTweetLength();
  ASSERT_EQ(mtl, 280);

  int32_t sul = validator->getShortUrlLength();
  ASSERT_EQ(sul, 23);
  validator->setShortUrlLength(42);
  sul = validator->getShortUrlLength();
  ASSERT_EQ(sul, 42);

  int32_t sul_https = validator->getShortUrlLengthHttps();
  ASSERT_EQ(sul_https, 23);
  validator->setShortUrlLengthHttps(42);
  sul_https = validator->getShortUrlLengthHttps();
  ASSERT_EQ(sul_https, 42);

  delete validator;
}

const char * const boolToString(bool b)
{
  return b ? "true" : "false";
}

TEST(ValidatorTest, Yaml) {
  Validator *validator = new Validator();
  YAML::Node map = YAML::LoadFile("rust/conformance/tests/validate.yml");
  auto tweets = readYaml<TestCase>(map["tests"]["tweets"]);
  auto usernames = readYaml<TestCase>(map["tests"]["usernames"]);
  auto lists = readYaml<TestCase>(map["tests"]["lists"]);
  auto hashtags = readYaml<TestCase>(map["tests"]["hashtags"]);
  auto urls = readYaml<TestCase>(map["tests"]["urls"]);
  auto urls_without_protocol = readYaml<TestCase>(map["tests"]["urls_without_protocol"]);

  for (TestCase test : tweets) {
    ASSERT_EQ(test.expected, boolToString(validator->isValidTweet(test.text)));
  }

  for (TestCase test : usernames) {
    ASSERT_EQ(test.expected, boolToString(validator->isValidUsername(test.text)));
  }

  for (TestCase test : lists) {
    ASSERT_EQ(test.expected, boolToString(validator->isValidList(test.text)));
  }

  for (TestCase test : hashtags) {
    ASSERT_EQ(test.expected, boolToString(validator->isValidHashtag(test.text)));
  }

  for (TestCase test : urls) {
    ASSERT_EQ(test.expected, boolToString(validator->isValidUrl(test.text)));
  }

  for (TestCase test : urls_without_protocol) {
    ASSERT_EQ(test.expected, boolToString(validator->isValidUrlWithoutProtocol(test.text)));
  }

  delete validator;
}

void
validateWeighting(std::vector<WeightedTweetTestCase> &tests, TwitterTextConfiguration &config) {
  ASSERT_TRUE(tests.size() > 0);
  for (WeightedTweetTestCase test : tests) {
    auto result = TwitterTextParser::parse(test.text, config, true);
    ASSERT_EQ(test.expected.weightedLength, result.weighted_length);
    ASSERT_EQ(test.expected.valid, result.is_valid);
    ASSERT_EQ(test.expected.permillage, result.permillage);
    ASSERT_EQ(test.expected.displayRangeStart, result.display_text_range.start);
    ASSERT_EQ(test.expected.displayRangeEnd, result.display_text_range.end);
    ASSERT_EQ(test.expected.validRangeStart, result.valid_text_range.start);
    ASSERT_EQ(test.expected.validRangeEnd, result.valid_text_range.end);
  }
}

TEST(ValidatorTest, Weighted) {
  YAML::Node map = YAML::LoadFile("rust/conformance/tests/validate.yml");
  auto counter_tests = readYaml<WeightedTweetTestCase>(map["tests"]["WeightedTweetsCounterTest"]);
  auto emoji_tests = readYaml<WeightedTweetTestCase>(map["tests"]["WeightedTweetsWithDiscountedEmojiCounterTest"]);
  auto directional_marker_tests = readYaml<WeightedTweetTestCase>(map["tests"]["UnicodeDirectionalMarkerCounterTest"]);

  auto config_v2 = TwitterTextConfiguration::configV2();
  auto config_v3 = TwitterTextConfiguration::configV3();

  validateWeighting(counter_tests, *config_v2);
  validateWeighting(emoji_tests, *config_v3);
  validateWeighting(directional_marker_tests, *config_v3);

  delete config_v2;
  delete config_v3;
}

// Runes weigh one unit each under v4, two under v3 and under the default.
// https://github.com/twitter/twitter-text/issues/430
TEST(ValidatorTest, RunicWeighting) {
  const std::string runes = u8"\u16A0\u16A2\u16A6\u16A8\u16B1\u16B2";

  auto config_v4 = TwitterTextConfiguration::configV4();
  auto result_v4 = TwitterTextParser::parse(runes, *config_v4, true);
  ASSERT_EQ(result_v4.weighted_length, 6);
  ASSERT_TRUE(result_v4.is_valid);
  delete config_v4;

  auto config_v3 = TwitterTextConfiguration::configV3();
  auto result_v3 = TwitterTextParser::parse(runes, *config_v3, true);
  ASSERT_EQ(result_v3.weighted_length, 12);
  delete config_v3;

  // v4 is opt-in: the default configuration still weighs runes as logograms.
  TwitterTextConfiguration default_config;
  auto result_default = TwitterTextParser::parse(runes, default_config, true);
  ASSERT_EQ(result_default.weighted_length, 12);
}

// Six letters from an alphabetic block above U+10FF weigh six units under v4
// and twelve under v3 and under the default.
TEST(ValidatorTest, AlphabeticScriptsWeighLikeLatinInV4) {
  const std::pair<const char *, std::string> samples[] = {
    {"Latin Extended Additional", u8"\u1E00\u1E01\u1E02\u1E03\u1E04\u1E05"},
    {"Greek Extended", u8"\u1F00\u1F01\u1F02\u1F03\u1F04\u1F05"},
    {"Ethiopic", u8"\u1200\u1208\u1210\u1218\u1220\u1228"},
    {"Cherokee", u8"\u13A0\u13A1\u13A2\u13A3\u13A4\u13A5"},
    {"Khmer", u8"\u1780\u1781\u1782\u1783\u1784\u1785"},
    {"Arabic Presentation Forms-A", u8"\uFB50\uFB51\uFB52\uFB53\uFB54\uFB55"},
    {"Adlam", u8"\U0001E900\U0001E901\U0001E902\U0001E903\U0001E904\U0001E905"},
  };

  auto config_v4 = TwitterTextConfiguration::configV4();
  auto config_v3 = TwitterTextConfiguration::configV3();
  TwitterTextConfiguration default_config;
  for (const auto &sample : samples) {
    auto result_v4 = TwitterTextParser::parse(sample.second, *config_v4, true);
    ASSERT_EQ(result_v4.weighted_length, 6) << sample.first;
    ASSERT_TRUE(result_v4.is_valid) << sample.first;

    auto result_v3 = TwitterTextParser::parse(sample.second, *config_v3, true);
    ASSERT_EQ(result_v3.weighted_length, 12) << sample.first;

    auto result_default = TwitterTextParser::parse(sample.second, default_config, true);
    ASSERT_EQ(result_default.weighted_length, 12) << sample.first;
  }
  delete config_v4;
  delete config_v3;
}

} // twitter_text
