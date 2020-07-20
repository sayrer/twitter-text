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
  auto tweets = readYaml<TestCase>(map["tweets"]["plain_text"]);
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
validateWeighting(std::vector<WeightedTweetTestCase> &tests, TwitterTextConfiguration<> &config) {
  ASSERT_TRUE(tests.size() > 0);
  for (WeightedTweetTestCase test : tests) {
    auto result = TwitterTextParser<>::parse(test.text, config, true);
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

  validateWeighting(counter_tests, *TwitterTextConfiguration<>::configV2());
  validateWeighting(emoji_tests, *TwitterTextConfiguration<>::configV3());
  validateWeighting(directional_marker_tests, *TwitterTextConfiguration<>::configV3());
}

} // twitter_text