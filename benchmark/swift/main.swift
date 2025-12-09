// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

import Foundation
import TwitterText
import Yams

let ITERATIONS = 1000
let WARMUP_ITERATIONS = 100

struct TestCase: Decodable {
    let description: String?
    let text: String
}

struct ExtractTests: Decodable {
    let mentions: [TestCase]?
    let urls: [TestCase]?
    let hashtags: [TestCase]?
    let cashtags: [TestCase]?
}

struct ExtractData: Decodable {
    let tests: ExtractTests
}

struct ValidateTests: Decodable {
    let tweets: [TestCase]?
}

struct ValidateData: Decodable {
    let tests: ValidateTests
}

struct ParseData: Decodable {
    let tests: [TestCase]
}

struct AutolinkData: Decodable {
    let tests: [TestCase]
}

func loadYAML<T: Decodable>(from path: String) -> T {
    let contents = try! String(contentsOfFile: path, encoding: .utf8)
    return try! YAMLDecoder().decode(T.self, from: contents)
}

func benchmarkAutolink(_ data: AutolinkData) -> Double {
    let autolinker = Autolinker(noFollow: false)
    let tests = data.tests

    // Warmup
    for _ in 0..<WARMUP_ITERATIONS {
        for test in tests {
            _ = autolinker.autolink(test.text)
        }
    }

    // Benchmark
    let start = CFAbsoluteTimeGetCurrent()
    for _ in 0..<ITERATIONS {
        for test in tests {
            _ = autolinker.autolink(test.text)
        }
    }
    let elapsed = CFAbsoluteTimeGetCurrent() - start
    return Double(ITERATIONS) / elapsed
}

func benchmarkExtract(_ data: ExtractData) -> Double {
    let extractor = Extractor()

    // Collect all texts (like JS/Rust benchmark)
    var allTexts: [String] = []
    if let mentions = data.tests.mentions {
        allTexts.append(contentsOf: mentions.map { $0.text })
    }
    if let urls = data.tests.urls {
        allTexts.append(contentsOf: urls.map { $0.text })
    }
    if let hashtags = data.tests.hashtags {
        allTexts.append(contentsOf: hashtags.map { $0.text })
    }
    if let cashtags = data.tests.cashtags {
        allTexts.append(contentsOf: cashtags.map { $0.text })
    }

    // Warmup - call all 4 extract functions for each text (like JS/Rust)
    for _ in 0..<WARMUP_ITERATIONS {
        for text in allTexts {
            _ = extractor.extractMentions(from: text)
            _ = extractor.extractURLs(from: text)
            _ = extractor.extractHashtags(from: text)
            _ = extractor.extractCashtags(from: text)
        }
    }

    // Benchmark
    let start = CFAbsoluteTimeGetCurrent()
    for _ in 0..<ITERATIONS {
        for text in allTexts {
            _ = extractor.extractMentions(from: text)
            _ = extractor.extractURLs(from: text)
            _ = extractor.extractHashtags(from: text)
            _ = extractor.extractCashtags(from: text)
        }
    }
    let elapsed = CFAbsoluteTimeGetCurrent() - start
    return Double(ITERATIONS) / elapsed
}

func benchmarkValidate(_ data: ValidateData) -> Double {
    let validator = Validator()
    let tweets = data.tests.tweets ?? []

    // Warmup
    for _ in 0..<WARMUP_ITERATIONS {
        for test in tweets {
            _ = validator.isValidTweet(test.text)
        }
    }

    // Benchmark
    let start = CFAbsoluteTimeGetCurrent()
    for _ in 0..<ITERATIONS {
        for test in tweets {
            _ = validator.isValidTweet(test.text)
        }
    }
    let elapsed = CFAbsoluteTimeGetCurrent() - start
    return Double(ITERATIONS) / elapsed
}

func benchmarkParse(_ data: ParseData) -> Double {
    let config = Configuration.default

    // Warmup
    for _ in 0..<WARMUP_ITERATIONS {
        for test in data.tests {
            _ = TwitterTextParser.parse(text: test.text, config: config)
        }
    }

    // Benchmark
    let start = CFAbsoluteTimeGetCurrent()
    for _ in 0..<ITERATIONS {
        for test in data.tests {
            _ = TwitterTextParser.parse(text: test.text, config: config)
        }
    }
    let elapsed = CFAbsoluteTimeGetCurrent() - start
    return Double(ITERATIONS) / elapsed
}

// Main
guard CommandLine.arguments.count >= 5 else {
    fputs("Usage: \(CommandLine.arguments[0]) <autolink.yml> <extract.yml> <validate.yml> <parse.yml>\n", stderr)
    exit(1)
}

let autolinkPath = CommandLine.arguments[1]
let extractPath = CommandLine.arguments[2]
let validatePath = CommandLine.arguments[3]
let parsePath = CommandLine.arguments[4]

print("Twitter Text Benchmark: Swift (via Rust FFI)")
print("=============================================")

let autolinkData: AutolinkData = loadYAML(from: autolinkPath)
let extractData: ExtractData = loadYAML(from: extractPath)
let validateData: ValidateData = loadYAML(from: validatePath)
let parseData: ParseData = loadYAML(from: parsePath)

let autolinkOps = benchmarkAutolink(autolinkData)
let extractOps = benchmarkExtract(extractData)
let validateOps = benchmarkValidate(validateData)
let parseOps = benchmarkParse(parseData)

print("")
print("Autolink (\(ITERATIONS) iterations):")
print("  Swift: \(Int(autolinkOps)) ops/sec")

print("")
print("Extract (\(ITERATIONS) iterations):")
print("  Swift: \(Int(extractOps)) ops/sec")

print("")
print("Validate (\(ITERATIONS) iterations):")
print("  Swift: \(Int(validateOps)) ops/sec")

print("")
print("Parse Tweet (\(ITERATIONS) iterations):")
print("  Swift: \(Int(parseOps)) ops/sec")

print("")
print("Done.")
