// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

#include <chrono>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "rust/cpp-bindings/twitter.h"
#include "yaml-cpp/yaml.h"

const int ITERATIONS = 1000;
const int WARMUP_ITERATIONS = 100;

struct TestCase {
    std::string description;
    std::string text;
};

std::vector<TestCase> loadTests(const YAML::Node& node) {
    std::vector<TestCase> tests;
    for (const auto& test : node) {
        TestCase tc;
        if (test["description"]) {
            tc.description = test["description"].as<std::string>();
        }
        tc.text = test["text"].as<std::string>();
        tests.push_back(tc);
    }
    return tests;
}

double benchmarkAutolink(const std::vector<TestCase>& tests) {
    twitter_text::Autolinker autolinker;
    autolinker.setNoFollow(false);

    // Warmup
    for (int i = 0; i < WARMUP_ITERATIONS; i++) {
        for (const auto& test : tests) {
            autolinker.autolink(test.text);
        }
    }

    // Benchmark
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        for (const auto& test : tests) {
            autolinker.autolink(test.text);
        }
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    return ITERATIONS / elapsed.count();
}

double benchmarkExtract(
    const std::vector<TestCase>& mentions,
    const std::vector<TestCase>& urls,
    const std::vector<TestCase>& hashtags,
    const std::vector<TestCase>& cashtags
) {
    twitter_text::Extractor<> extractor;

    // Collect all texts (like JS/Rust benchmark)
    std::vector<std::string> allTexts;
    for (const auto& test : mentions) {
        allTexts.push_back(test.text);
    }
    for (const auto& test : urls) {
        allTexts.push_back(test.text);
    }
    for (const auto& test : hashtags) {
        allTexts.push_back(test.text);
    }
    for (const auto& test : cashtags) {
        allTexts.push_back(test.text);
    }

    // Warmup - call all 4 extract functions for each text (like JS/Rust)
    for (int i = 0; i < WARMUP_ITERATIONS; i++) {
        for (const auto& text : allTexts) {
            extractor.extractMentionedScreennamesWithIndices(text);
            extractor.extractUrlsWithIndices(text);
            extractor.extractHashtagsWithIndices(text);
            extractor.extractCashtagsWithIndices(text);
        }
    }

    // Benchmark
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        for (const auto& text : allTexts) {
            extractor.extractMentionedScreennamesWithIndices(text);
            extractor.extractUrlsWithIndices(text);
            extractor.extractHashtagsWithIndices(text);
            extractor.extractCashtagsWithIndices(text);
        }
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    return ITERATIONS / elapsed.count();
}

double benchmarkValidateTweet(const std::vector<TestCase>& tweets) {
    twitter_text::Validator validator;

    // Warmup
    for (int i = 0; i < WARMUP_ITERATIONS; i++) {
        for (const auto& test : tweets) {
            validator.isValidTweet(test.text);
        }
    }

    // Benchmark
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        for (const auto& test : tweets) {
            validator.isValidTweet(test.text);
        }
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    return ITERATIONS / elapsed.count();
}

double benchmarkValidateAll(
    const std::vector<TestCase>& tweets,
    const std::vector<TestCase>& usernames,
    const std::vector<TestCase>& hashtags,
    const std::vector<TestCase>& urls
) {
    twitter_text::Validator validator;

    // Warmup - call all 4 validate functions
    for (int i = 0; i < WARMUP_ITERATIONS; i++) {
        for (const auto& test : tweets) {
            validator.isValidTweet(test.text);
        }
        for (const auto& test : usernames) {
            validator.isValidUsername(test.text);
        }
        for (const auto& test : hashtags) {
            validator.isValidHashtag(test.text);
        }
        for (const auto& test : urls) {
            validator.isValidUrl(test.text);
        }
    }

    // Benchmark
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        for (const auto& test : tweets) {
            validator.isValidTweet(test.text);
        }
        for (const auto& test : usernames) {
            validator.isValidUsername(test.text);
        }
        for (const auto& test : hashtags) {
            validator.isValidHashtag(test.text);
        }
        for (const auto& test : urls) {
            validator.isValidUrl(test.text);
        }
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    return ITERATIONS / elapsed.count();
}

double benchmarkParse(const std::vector<TestCase>& tests) {
    twitter_text::TwitterTextConfiguration config;

    // Warmup
    for (int i = 0; i < WARMUP_ITERATIONS; i++) {
        for (const auto& test : tests) {
            twitter_text::TwitterTextParser::parse(test.text, config, true);
        }
    }

    // Benchmark
    auto start = std::chrono::high_resolution_clock::now();
    for (int i = 0; i < ITERATIONS; i++) {
        for (const auto& test : tests) {
            twitter_text::TwitterTextParser::parse(test.text, config, true);
        }
    }
    auto end = std::chrono::high_resolution_clock::now();
    std::chrono::duration<double> elapsed = end - start;
    return ITERATIONS / elapsed.count();
}

int main(int argc, char* argv[]) {
    if (argc < 5) {
        std::cerr << "Usage: " << argv[0] 
                  << " <autolink.yml> <extract.yml> <validate.yml> <parse.yml>" 
                  << std::endl;
        return 1;
    }

    std::cout << "Twitter Text Benchmark: C++ (via Rust FFI)" << std::endl;
    std::cout << "===========================================" << std::endl;

    // Load YAML files
    YAML::Node autolinkYaml = YAML::LoadFile(argv[1]);
    YAML::Node extractYaml = YAML::LoadFile(argv[2]);
    YAML::Node validateYaml = YAML::LoadFile(argv[3]);
    YAML::Node parseYaml = YAML::LoadFile(argv[4]);

    // Load test cases
    auto autolinkTests = loadTests(autolinkYaml["tests"]);
    
    std::vector<TestCase> mentionTests, urlTests, hashtagTests, cashtagTests;
    if (extractYaml["tests"]["mentions"]) {
        mentionTests = loadTests(extractYaml["tests"]["mentions"]);
    }
    if (extractYaml["tests"]["urls"]) {
        urlTests = loadTests(extractYaml["tests"]["urls"]);
    }
    if (extractYaml["tests"]["hashtags"]) {
        hashtagTests = loadTests(extractYaml["tests"]["hashtags"]);
    }
    if (extractYaml["tests"]["cashtags"]) {
        cashtagTests = loadTests(extractYaml["tests"]["cashtags"]);
    }

    std::vector<TestCase> validateTweets, validateUsernames, validateHashtags, validateUrls;
    if (validateYaml["tests"]["tweets"]) {
        validateTweets = loadTests(validateYaml["tests"]["tweets"]);
    }
    if (validateYaml["tests"]["usernames"]) {
        validateUsernames = loadTests(validateYaml["tests"]["usernames"]);
    }
    if (validateYaml["tests"]["hashtags"]) {
        validateHashtags = loadTests(validateYaml["tests"]["hashtags"]);
    }
    if (validateYaml["tests"]["urls"]) {
        validateUrls = loadTests(validateYaml["tests"]["urls"]);
    }

    auto parseTests = loadTests(parseYaml["tests"]);

    // Run benchmarks
    double autolinkOps = benchmarkAutolink(autolinkTests);
    double extractOps = benchmarkExtract(mentionTests, urlTests, hashtagTests, cashtagTests);
    double validateTweetOps = benchmarkValidateTweet(validateTweets);
    double validateAllOps = benchmarkValidateAll(validateTweets, validateUsernames, validateHashtags, validateUrls);
    double parseOps = benchmarkParse(parseTests);

    // Print results
    std::cout << std::endl;
    std::cout << "Autolink (" << ITERATIONS << " iterations):" << std::endl;
    std::cout << "  C++: " << static_cast<int>(autolinkOps) << " ops/sec" << std::endl;

    std::cout << std::endl;
    std::cout << "Extract (" << ITERATIONS << " iterations):" << std::endl;
    std::cout << "  C++: " << static_cast<int>(extractOps) << " ops/sec" << std::endl;

    std::cout << std::endl;
    std::cout << "Validate Tweet (" << ITERATIONS << " iterations):" << std::endl;
    std::cout << "  C++: " << static_cast<int>(validateTweetOps) << " ops/sec" << std::endl;

    std::cout << std::endl;
    std::cout << "Validate All (" << ITERATIONS << " iterations):" << std::endl;
    std::cout << "  C++: " << static_cast<int>(validateAllOps) << " ops/sec" << std::endl;

    std::cout << std::endl;
    std::cout << "Parse Tweet (" << ITERATIONS << " iterations):" << std::endl;
    std::cout << "  C++: " << static_cast<int>(parseOps) << " ops/sec" << std::endl;

    std::cout << std::endl;
    std::cout << "Done." << std::endl;

    return 0;
}
