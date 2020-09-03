#include "twitter.h"
#include "gtest/gtest.h"
#include "yaml-cpp/yaml.h"

struct HighlightTestCase {
	std::string description;
	std::string text;
	std::vector<twitter_text::Hit> hits;
	std::string expected;
};

namespace YAML {
template<>
struct convert<HighlightTestCase> {
  static Node encode(const HighlightTestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;

    Node hits;
    for(twitter_text::Hit h : rhs.hits) {
    	Node hit;
    	hit.push_back(h.start);
    	hit.push_back(h.end);
    	hits.push_back(hit);
    }

    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, HighlightTestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();

    Node hits = node["hits"];
    for (const_iterator it = hits.begin(); it != hits.end(); ++it) {
    	const YAML::Node& h = *it;
    	twitter_text::Hit hit;
    	hit.start = h[0].as<int32_t>();
    	hit.end = h[1].as<int32_t>();
    	rhs.hits.push_back(hit);
    }

    rhs.expected = node["expected"].as<std::string>();

    return true;
  }
};
} // namespace YAML

std::vector<HighlightTestCase>
readYaml(YAML::Node node) {
	std::vector<HighlightTestCase> testcases;
	for (YAML::const_iterator it = node.begin(); it != node.end(); ++it) {
    const YAML::Node& test = *it;
		testcases.push_back(test.as<HighlightTestCase>());
	}
	return testcases;
}

namespace twitter_text {

TEST(HithighlighterTest, Ctor) {
  HitHighlighter *highlighter = new HitHighlighter();
  ASSERT_NE(highlighter, nullptr);
}

TEST(HithighlighterTest, CtorWithString) {
  HitHighlighter *highlighter = new HitHighlighter("hmm");
  ASSERT_NE(highlighter, nullptr);
}

TEST(HithighlighterTest, Yaml) {
  HitHighlighter *highlighter = new HitHighlighter();
  YAML::Node map = YAML::LoadFile("rust/conformance/tests/hit_highlighting.yml");
  auto plain_text = readYaml(map["tests"]["plain_text"]);
  auto with_links = readYaml(map["tests"]["with_links"]);

  for (HighlightTestCase test : plain_text) {
  	ASSERT_EQ(test.expected, std::string(highlighter->highlight(test.text, test.hits)));
  }

  for (HighlightTestCase test : with_links) {
  	ASSERT_EQ(test.expected, std::string(highlighter->highlight(test.text, test.hits)));
  }
}

} // twitter_text