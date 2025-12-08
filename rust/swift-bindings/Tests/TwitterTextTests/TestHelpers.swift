import Foundation
import Yams

// MARK: - YAML Test Case Structures

struct AutolinkTestCase {
    let description: String
    let text: String
    let expected: String
}

struct ExtractTestCase {
    let description: String
    let text: String
    let expected: [String]
}

struct ValidateTestCase {
    let description: String
    let text: String
    let expected: Bool
}

struct HitHighlightTestCase {
    let description: String
    let text: String
    let hits: [[Int]]
    let expected: String
}

struct TldTestCase {
    let description: String
    let text: String
    let expected: String
}

// MARK: - File Loading

func loadTextFile(from filename: String) -> String? {
    let possiblePaths = [
        "rust/conformance/tests/\(filename)",
        "../../../rust/conformance/tests/\(filename)",
        "external/_main/rust/conformance/tests/\(filename)",
        "\(filename)"
    ]

    for path in possiblePaths {
        if let contents = try? String(contentsOfFile: path, encoding: .utf8) {
            return contents
        }
    }

    if let runfilesDir = ProcessInfo.processInfo.environment["RUNFILES_DIR"] {
        let runfilesPath = "\(runfilesDir)/_main/rust/conformance/tests/\(filename)"
        if let contents = try? String(contentsOfFile: runfilesPath, encoding: .utf8) {
            return contents
        }
    }

    return nil
}

// MARK: - YAML Loading

func loadYAML(from filename: String) -> [String: Any]? {
    let possiblePaths = [
        "conformance/\(filename)",
        "../../../conformance/\(filename)",
        "external/_main/conformance/\(filename)",
        "\(filename)"
    ]

    for path in possiblePaths {
        if let contents = try? String(contentsOfFile: path, encoding: .utf8),
           let yaml = try? Yams.load(yaml: contents) as? [String: Any] {
            return yaml
        }
    }

    if let runfilesDir = ProcessInfo.processInfo.environment["RUNFILES_DIR"] {
        let runfilesPath = "\(runfilesDir)/_main/conformance/\(filename)"
        if let contents = try? String(contentsOfFile: runfilesPath, encoding: .utf8),
           let yaml = try? Yams.load(yaml: contents) as? [String: Any] {
            return yaml
        }
    }

    return nil
}

// MARK: - YAML Parsers

func parseAutolinkTests(from yaml: [String: Any], key: String) -> [AutolinkTestCase] {
    guard let tests = yaml["tests"] as? [String: Any],
          let testList = tests[key] as? [[String: Any]] else {
        return []
    }

    return testList.compactMap { test in
        guard let description = test["description"] as? String,
              let text = test["text"] as? String,
              let expected = test["expected"] as? String else {
            return nil
        }
        return AutolinkTestCase(description: description, text: text, expected: expected)
    }
}

func parseExtractTests(from yaml: [String: Any], key: String) -> [ExtractTestCase] {
    guard let tests = yaml["tests"] as? [String: Any],
          let testList = tests[key] as? [[String: Any]] else {
        return []
    }

    return testList.compactMap { test in
        guard let description = test["description"] as? String,
              let text = test["text"] as? String,
              let expected = test["expected"] as? [String] else {
            return nil
        }
        return ExtractTestCase(description: description, text: text, expected: expected)
    }
}

func parseValidateTests(from yaml: [String: Any], key: String) -> [ValidateTestCase] {
    guard let tests = yaml["tests"] as? [String: Any],
          let testList = tests[key] as? [[String: Any]] else {
        return []
    }

    return testList.compactMap { test in
        guard let description = test["description"] as? String,
              let text = test["text"] as? String,
              let expected = test["expected"] as? Bool else {
            return nil
        }
        return ValidateTestCase(description: description, text: text, expected: expected)
    }
}

func parseHitHighlightTests(from yaml: [String: Any], key: String) -> [HitHighlightTestCase] {
    guard let tests = yaml["tests"] as? [String: Any],
          let testList = tests[key] as? [[String: Any]] else {
        return []
    }

    return testList.compactMap { test in
        guard let description = test["description"] as? String,
              let text = test["text"] as? String,
              let hits = test["hits"] as? [[Int]],
              let expected = test["expected"] as? String else {
            return nil
        }
        return HitHighlightTestCase(description: description, text: text, hits: hits, expected: expected)
    }
}

func parseTldTests(from yaml: [String: Any], key: String) -> [TldTestCase] {
    guard let tests = yaml["tests"] as? [String: Any],
          let testList = tests[key] as? [[String: Any]] else {
        return []
    }

    return testList.compactMap { test in
        guard let description = test["description"] as? String,
              let text = test["text"] as? String,
              let expected = test["expected"] as? [Any],
              let firstExpected = expected.first as? String else {
            return nil
        }
        return TldTestCase(description: description, text: text, expected: firstExpected)
    }
}
