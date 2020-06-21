#include "twitter.h"
#include "gtest/gtest.h"
#include "test_helpers.h"

struct ExtractTestCase {
  std::string description;
  std::string text;
  std::vector<std::string> expected;
};

struct MentionIndexExpectation {
  std::string screen_name;
  std::vector<int32_t> indices;
};

struct MentionIndexTestCase {
  std::string description;
  std::string text;
  std::vector<MentionIndexExpectation> expected;
};

struct MentionOrListIndexExpectation {
  std::string screen_name;
  std::string list_slug;
  std::vector<int32_t> indices;
};

struct MentionOrListIndexTestCase {
  std::string description;
  std::string text;
  std::vector<MentionOrListIndexExpectation> expected;
};

struct ReplyTestCase {
  std::string description;
  std::string text;
  std::string *expected;
};

struct UrlIndexExpectation {
  std::string url;
  std::vector<int32_t> indices;
};

struct UrlIndexTestCase {
  std::string description;
  std::string text;
  std::vector<UrlIndexExpectation> expected;
};

struct HashtagIndexExpectation {
  std::string hashtag;
  std::vector<int32_t> indices;
};

struct HashtagIndexTestCase {
  std::string description;
  std::string text;
  std::vector<HashtagIndexExpectation> expected;
};

struct CashtagIndexExpectation {
  std::string cashtag;
  std::vector<int32_t> indices;
};

struct CashtagIndexTestCase {
  std::string description;
  std::string text;
  std::vector<CashtagIndexExpectation> expected;
};

namespace YAML {
template<>
struct convert<ExtractTestCase> {
  static Node encode(const ExtractTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, ExtractTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::vector<std::string>>();

    return true;
  }
};

template<>
struct convert<MentionIndexExpectation> {
  static Node encode(const MentionIndexExpectation& rhs) {
    Node node;
    node["screen_name"] = rhs.screen_name;
    node["indices"] = rhs.indices;

    return node;
  }

  static bool decode(const Node& node, MentionIndexExpectation& rhs) {
    rhs.screen_name = node["screen_name"].as<std::string>();
    rhs.indices = node["indices"].as<std::vector<int32_t>>();

    return true;
  }
};

template<>
struct convert<MentionIndexTestCase> {
  static Node encode(const MentionIndexTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, MentionIndexTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::vector<MentionIndexExpectation>>();

    return true;
  }
};

template<>
struct convert<MentionOrListIndexExpectation> {
  static Node encode(const MentionOrListIndexExpectation& rhs) {
    Node node;
    node["screen_name"] = rhs.screen_name;
    node["list_slug"] = rhs.list_slug;
    node["indices"] = rhs.indices;

    return node;
  }

  static bool decode(const Node& node, MentionOrListIndexExpectation& rhs) {
    rhs.screen_name = node["screen_name"].as<std::string>();
    rhs.list_slug = node["list_slug"].as<std::string>();
    rhs.indices = node["indices"].as<std::vector<int32_t>>();

    return true;
  }
};

template<>
struct convert<MentionOrListIndexTestCase> {
  static Node encode(const MentionOrListIndexTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, MentionOrListIndexTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::vector<MentionOrListIndexExpectation>>();

    return true;
  }
};

template<>
struct convert<ReplyTestCase> {
  static Node encode(const ReplyTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = *rhs.expected;

    return node;
  }

  static bool decode(const Node& node, ReplyTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();

    if (node["expected"].Type() == NodeType::Scalar) {
      rhs.expected = new std::string(node["expected"].as<std::string>());
    } else {
      rhs.expected = nullptr;
    }

    return true;
  }
};

template<>
struct convert<UrlIndexExpectation> {
  static Node encode(const UrlIndexExpectation& rhs) {
    Node node;
    node["url"] = rhs.url;
    node["indices"] = rhs.indices;

    return node;
  }

  static bool decode(const Node& node, UrlIndexExpectation& rhs) {
    rhs.url = node["url"].as<std::string>();
    rhs.indices = node["indices"].as<std::vector<int32_t>>();

    return true;
  }
};

template<>
struct convert<UrlIndexTestCase> {
  static Node encode(const UrlIndexTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, UrlIndexTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::vector<UrlIndexExpectation>>();

    return true;
  }
};

template<>
struct convert<HashtagIndexExpectation> {
  static Node encode(const HashtagIndexExpectation& rhs) {
    Node node;
    node["hashtag"] = rhs.hashtag;
    node["indices"] = rhs.indices;

    return node;
  }

  static bool decode(const Node& node, HashtagIndexExpectation& rhs) {
    rhs.hashtag = node["hashtag"].as<std::string>();
    rhs.indices = node["indices"].as<std::vector<int32_t>>();

    return true;
  }
};

template<>
struct convert<HashtagIndexTestCase> {
  static Node encode(const HashtagIndexTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, HashtagIndexTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::vector<HashtagIndexExpectation>>();

    return true;
  }
};

template<>
struct convert<CashtagIndexExpectation> {
  static Node encode(const CashtagIndexExpectation& rhs) {
    Node node;
    node["cashtag"] = rhs.cashtag;
    node["indices"] = rhs.indices;

    return node;
  }

  static bool decode(const Node& node, CashtagIndexExpectation& rhs) {
    rhs.cashtag = node["cashtag"].as<std::string>();
    rhs.indices = node["indices"].as<std::vector<int32_t>>();

    return true;
  }
};

template<>
struct convert<CashtagIndexTestCase> {
  static Node encode(const CashtagIndexTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, CashtagIndexTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::vector<CashtagIndexExpectation>>();

    return true;
  }
};

} // namespace YAML

namespace twitter_text {

TEST(ExtractorTest, Ctor) {
  Extractor *extractor = new Extractor();
  ASSERT_NE(extractor, nullptr);
  delete extractor;
}

TEST(ExtractorTest, Accessors) {
  Extractor *extractor = new Extractor();
  ASSERT_NE(extractor, nullptr);
  delete extractor;
}

TEST(ExtractorTest, Yaml) {
  Extractor *extractor = new Extractor();
  YAML::Node map = YAML::LoadFile("rust/conformance/tests/extract.yml");
  auto mentions = readYaml<ExtractTestCase>(map["tests"]["mentions"]);
  auto mentions_with_indices = readYaml<MentionIndexTestCase>(map["tests"]["mentions_with_indices"]);
  auto mentions_or_lists_with_indices = readYaml<MentionOrListIndexTestCase>(map["tests"]["mentions_or_lists_with_indices"]);
  auto replies = readYaml<ReplyTestCase>(map["tests"]["replies"]);
  auto urls = readYaml<ExtractTestCase>(map["tests"]["urls"]);
  auto urls_with_indices = readYaml<UrlIndexTestCase>(map["tests"]["urls_with_indices"]);
  auto urls_with_directional_markers = readYaml<UrlIndexTestCase>(map["tests"]["urls_with_directional_markers"]);
  auto tco_urls_with_params = readYaml<ExtractTestCase>(map["tests"]["tco_urls_with_params"]);
  auto hashtags = readYaml<ExtractTestCase>(map["tests"]["hashtags"]);
  auto hashtags_from_astral = readYaml<ExtractTestCase>(map["tests"]["hashtags_from_astral"]);
  auto hashtags_with_indices = readYaml<HashtagIndexTestCase>(map["tests"]["hashtags_with_indices"]);
  auto cashtags = readYaml<ExtractTestCase>(map["tests"]["cashtags"]);
  auto cashtags_with_indices = readYaml<CashtagIndexTestCase>(map["tests"]["cashtags_with_indices"]);

  ASSERT_TRUE(mentions.size() > 0);
  for (ExtractTestCase test : mentions) {
    ASSERT_EQ(test.expected, extractor->extractMentionedScreennames(test.text));
  }

  ASSERT_TRUE(mentions_with_indices.size() > 0);
  for (MentionIndexTestCase test : mentions_with_indices) {
    std::vector<Entity> entities = extractor->extractMentionedScreennamesWithIndices(test.text);
    ASSERT_EQ(test.expected.size(), entities.size());
    for (auto it = entities.begin(); it != entities.end(); ++it) {
      size_t index = std::distance(entities.begin(), it);
      ASSERT_EQ(std::string(entities[index].value), test.expected[index].screen_name);
      ASSERT_EQ(entities[index].start, test.expected[index].indices[0]);
      ASSERT_EQ(entities[index].end, test.expected[index].indices[1]);
    }
  }

  ASSERT_TRUE(mentions_or_lists_with_indices.size() > 0);
  for (MentionOrListIndexTestCase test : mentions_or_lists_with_indices) {
    std::vector<Entity> entities = extractor->extractMentionsOrListsWithIndices(test.text);
    ASSERT_EQ(test.expected.size(), entities.size());
    for (auto it = entities.begin(); it != entities.end(); ++it) {
      size_t index = std::distance(entities.begin(), it);
      ASSERT_EQ(std::string(entities[index].value), test.expected[index].screen_name);
      ASSERT_EQ(std::string(entities[index].list_slug), test.expected[index].list_slug);
      ASSERT_EQ(entities[index].start, test.expected[index].indices[0]);
      ASSERT_EQ(entities[index].end, test.expected[index].indices[1]);
    }
  }

  ASSERT_TRUE(replies.size() > 0);
  for (ReplyTestCase test : replies) {
    std::unique_ptr<Entity> entity = extractor->extractReplyScreenname(test.text);
    if (test.expected) {
      ASSERT_EQ(*test.expected, std::string(entity->value));
    } else {
      ASSERT_TRUE(!entity);
    }
  }

  ASSERT_TRUE(urls.size() > 0);
  for (ExtractTestCase test : urls) {
    ASSERT_EQ(test.expected, extractor->extractUrls(test.text));
  }

  ASSERT_TRUE(urls_with_indices.size() > 0);
  for (UrlIndexTestCase test : urls_with_indices) {
    std::vector<Entity> entities = extractor->extractUrlsWithIndices(test.text);
    ASSERT_EQ(test.expected.size(), entities.size());
    for (auto it = entities.begin(); it != entities.end(); ++it) {
      size_t index = std::distance(entities.begin(), it);
      ASSERT_EQ(std::string(entities[index].value), test.expected[index].url);
      ASSERT_EQ(entities[index].start, test.expected[index].indices[0]);
      ASSERT_EQ(entities[index].end, test.expected[index].indices[1]);
    }
  }

  ASSERT_TRUE(urls_with_directional_markers.size() > 0);
  for (UrlIndexTestCase test : urls_with_directional_markers) {
    std::vector<Entity> entities = extractor->extractUrlsWithIndices(test.text);
    ASSERT_EQ(test.expected.size(), entities.size());
    for (auto it = entities.begin(); it != entities.end(); ++it) {
      size_t index = std::distance(entities.begin(), it);
      ASSERT_EQ(std::string(entities[index].value), test.expected[index].url);
      ASSERT_EQ(entities[index].start, test.expected[index].indices[0]);
      ASSERT_EQ(entities[index].end, test.expected[index].indices[1]);
    }
  }

  ASSERT_TRUE(hashtags.size() > 0);
  for (ExtractTestCase test : hashtags) {
    ASSERT_EQ(test.expected, extractor->extractHashtags(test.text));
  }

  ASSERT_TRUE(hashtags_from_astral.size() > 0);
  for (ExtractTestCase test : hashtags_from_astral) {
    ASSERT_EQ(test.expected, extractor->extractHashtags(test.text));
  }

  ASSERT_TRUE(hashtags_with_indices.size() > 0);
  for (HashtagIndexTestCase test : hashtags_with_indices) {
    std::vector<Entity> entities = extractor->extractHashtagsWithIndices(test.text);
    ASSERT_EQ(test.expected.size(), entities.size());
    for (auto it = entities.begin(); it != entities.end(); ++it) {
      size_t index = std::distance(entities.begin(), it);
      ASSERT_EQ(std::string(entities[index].value), test.expected[index].hashtag);
      ASSERT_EQ(entities[index].start, test.expected[index].indices[0]);
      ASSERT_EQ(entities[index].end, test.expected[index].indices[1]);
    }
  }

  ASSERT_TRUE(cashtags.size() > 0);
  for (ExtractTestCase test : cashtags) {
    ASSERT_EQ(test.expected, extractor->extractCashtags(test.text));
  }

  ASSERT_TRUE(cashtags_with_indices.size() > 0);
  for (CashtagIndexTestCase test : cashtags_with_indices) {
    std::vector<Entity> entities = extractor->extractCashtagsWithIndices(test.text);
    ASSERT_EQ(test.expected.size(), entities.size());
    for (auto it = entities.begin(); it != entities.end(); ++it) {
      size_t index = std::distance(entities.begin(), it);
      ASSERT_EQ(std::string(entities[index].value), test.expected[index].cashtag);
      ASSERT_EQ(entities[index].start, test.expected[index].indices[0]);
      ASSERT_EQ(entities[index].end, test.expected[index].indices[1]);
    }
  }

  delete extractor;
}

}