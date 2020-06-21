#pragma once
#include "yaml-cpp/yaml.h"
#include <iostream>

struct TestCase {
  std::string description;
  std::string text;
  std::string expected;
};

namespace YAML {
template<>
struct convert<TestCase> {
  static Node encode(const TestCase& rhs) {
    Node node;
    node["description"] = rhs.description;
    node["text"] = rhs.text;
    node["expected"] = rhs.expected;

    return node;
  }

  static bool decode(const Node& node, TestCase& rhs) {
    rhs.description = node["description"].as<std::string>();
    rhs.text = node["text"].as<std::string>();
    rhs.expected = node["expected"].as<std::string>();

    return true;
  }
};
} // namespace YAML

template<typename T>
std::vector<T>
readYaml(YAML::Node node) {
  std::vector<T> testcases;
  for (YAML::const_iterator it = node.begin(); it != node.end(); ++it) {
    const YAML::Node& test = *it;
    testcases.push_back(test.as<T>());
  }
  return testcases;
}