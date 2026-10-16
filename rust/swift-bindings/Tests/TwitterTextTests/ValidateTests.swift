import Foundation
import TwitterText

#if os(macOS)
import XCTest

final class ValidateTests: XCTestCase {
    var validator: Validator!
    var yaml: [String: Any]!

    override func setUp() {
        super.setUp()
        validator = Validator()
        yaml = loadYAML(from: "validate.yml")
        XCTAssertNotNil(yaml, "Could not load validate.yml")
    }

    func testTweets() {
        let tests = parseValidateTests(from: yaml, key: "tweets")
        XCTAssertGreaterThan(tests.count, 0, "No tweet tests found")
        for test in tests {
            let result = validator.isValidTweet(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testUsernames() {
        let tests = parseValidateTests(from: yaml, key: "usernames")
        XCTAssertGreaterThan(tests.count, 0, "No username tests found")
        for test in tests {
            let result = validator.isValidUsername(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testLists() {
        let tests = parseValidateTests(from: yaml, key: "lists")
        XCTAssertGreaterThan(tests.count, 0, "No list tests found")
        for test in tests {
            let result = validator.isValidList(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testHashtags() {
        let tests = parseValidateTests(from: yaml, key: "hashtags")
        XCTAssertGreaterThan(tests.count, 0, "No hashtag tests found")
        for test in tests {
            let result = validator.isValidHashtag(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testURLs() {
        let tests = parseValidateTests(from: yaml, key: "urls")
        XCTAssertGreaterThan(tests.count, 0, "No URL tests found")
        for test in tests {
            let result = validator.isValidURL(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testURLsWithoutProtocol() {
        let tests = parseValidateTests(from: yaml, key: "urls_without_protocol")
        XCTAssertGreaterThan(tests.count, 0, "No URL without protocol tests found")
        for test in tests {
            let result = validator.isValidURLWithoutProtocol(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testRunicWeighting() {
        // v4 weights the Runic block like Latin; v3 keeps the upstream weighting.
        // See https://github.com/twitter/twitter-text/issues/430
        let runes = "\u{16A0}\u{16A2}\u{16A6}\u{16A8}\u{16B1}\u{16B2}"

        XCTAssertEqual(TwitterTextParser.parse(text: runes, config: .v4).weightedLength, 6)
        XCTAssertEqual(TwitterTextParser.parse(text: runes, config: .v3).weightedLength, 12)
        // v4 is opt-in: the default configuration still weighs runes as logograms.
        XCTAssertEqual(TwitterTextParser.parse(text: runes).weightedLength, 12)
    }

    func testAlphabeticScriptsWeighLikeLatinInV4() {
        // Six letters from an alphabetic block above U+10FF weigh six units under
        // v4 and twelve under v3 and under the default.
        let samples: [(String, String)] = [
            ("Latin Extended Additional", "\u{1E00}\u{1E01}\u{1E02}\u{1E03}\u{1E04}\u{1E05}"),
            ("Greek Extended", "\u{1F00}\u{1F01}\u{1F02}\u{1F03}\u{1F04}\u{1F05}"),
            ("Ethiopic", "\u{1200}\u{1208}\u{1210}\u{1218}\u{1220}\u{1228}"),
            ("Cherokee", "\u{13A0}\u{13A1}\u{13A2}\u{13A3}\u{13A4}\u{13A5}"),
            ("Khmer", "\u{1780}\u{1781}\u{1782}\u{1783}\u{1784}\u{1785}"),
            ("Arabic Presentation Forms-A", "\u{FB50}\u{FB51}\u{FB52}\u{FB53}\u{FB54}\u{FB55}"),
            ("Adlam", "\u{1E900}\u{1E901}\u{1E902}\u{1E903}\u{1E904}\u{1E905}"),
        ]
        for (name, text) in samples {
            XCTAssertEqual(text.unicodeScalars.count, 6, name)
            XCTAssertEqual(TwitterTextParser.parse(text: text, config: .v4).weightedLength, 6, name)
            XCTAssertEqual(TwitterTextParser.parse(text: text, config: .v3).weightedLength, 12, name)
            XCTAssertEqual(TwitterTextParser.parse(text: text).weightedLength, 12, name)
        }
    }
}

#else

class ValidateTests {
    var validator: Validator!
    var yaml: [String: Any]!

    func setUp() {
        validator = Validator()
        yaml = loadYAML(from: "validate.yml")
    }

    func run() {
        print("\n=== Validate Conformance Tests ===")
        setUp()
        XCTAssertNotNil(yaml, "Could not load validate.yml")
        guard yaml != nil else { return }

        testTweets()
        testUsernames()
        testLists()
        testHashtags()
        testURLs()
        testURLsWithoutProtocol()
        testRunicWeighting()
        testAlphabeticScriptsWeighLikeLatinInV4()
    }

    func testTweets() {
        let tests = parseValidateTests(from: yaml, key: "tweets")
        print("\nTweets (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No tweet tests found")
        for test in tests {
            let result = validator.isValidTweet(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testUsernames() {
        let tests = parseValidateTests(from: yaml, key: "usernames")
        print("\nUsernames (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No username tests found")
        for test in tests {
            let result = validator.isValidUsername(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testLists() {
        let tests = parseValidateTests(from: yaml, key: "lists")
        print("\nLists (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No list tests found")
        for test in tests {
            let result = validator.isValidList(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testHashtags() {
        let tests = parseValidateTests(from: yaml, key: "hashtags")
        print("\nHashtags (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No hashtag tests found")
        for test in tests {
            let result = validator.isValidHashtag(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testURLs() {
        let tests = parseValidateTests(from: yaml, key: "urls")
        print("\nURLs (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No URL tests found")
        for test in tests {
            let result = validator.isValidURL(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testURLsWithoutProtocol() {
        let tests = parseValidateTests(from: yaml, key: "urls_without_protocol")
        print("\nURLs without protocol (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No URL without protocol tests found")
        for test in tests {
            let result = validator.isValidURLWithoutProtocol(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testRunicWeighting() {
        // v4 weights the Runic block like Latin; v3 keeps the upstream weighting.
        // See https://github.com/twitter/twitter-text/issues/430
        let runes = "\u{16A0}\u{16A2}\u{16A6}\u{16A8}\u{16B1}\u{16B2}"

        XCTAssertEqual(TwitterTextParser.parse(text: runes, config: .v4).weightedLength, 6)
        XCTAssertEqual(TwitterTextParser.parse(text: runes, config: .v3).weightedLength, 12)
        // v4 is opt-in: the default configuration still weighs runes as logograms.
        XCTAssertEqual(TwitterTextParser.parse(text: runes).weightedLength, 12)
    }

    func testAlphabeticScriptsWeighLikeLatinInV4() {
        // Six letters from an alphabetic block above U+10FF weigh six units under
        // v4 and twelve under v3 and under the default.
        let samples: [(String, String)] = [
            ("Latin Extended Additional", "\u{1E00}\u{1E01}\u{1E02}\u{1E03}\u{1E04}\u{1E05}"),
            ("Greek Extended", "\u{1F00}\u{1F01}\u{1F02}\u{1F03}\u{1F04}\u{1F05}"),
            ("Ethiopic", "\u{1200}\u{1208}\u{1210}\u{1218}\u{1220}\u{1228}"),
            ("Cherokee", "\u{13A0}\u{13A1}\u{13A2}\u{13A3}\u{13A4}\u{13A5}"),
            ("Khmer", "\u{1780}\u{1781}\u{1782}\u{1783}\u{1784}\u{1785}"),
            ("Arabic Presentation Forms-A", "\u{FB50}\u{FB51}\u{FB52}\u{FB53}\u{FB54}\u{FB55}"),
            ("Adlam", "\u{1E900}\u{1E901}\u{1E902}\u{1E903}\u{1E904}\u{1E905}"),
        ]
        for (name, text) in samples {
            XCTAssertEqual(text.unicodeScalars.count, 6, name)
            XCTAssertEqual(TwitterTextParser.parse(text: text, config: .v4).weightedLength, 6, name)
            XCTAssertEqual(TwitterTextParser.parse(text: text, config: .v3).weightedLength, 12, name)
            XCTAssertEqual(TwitterTextParser.parse(text: text).weightedLength, 12, name)
        }
    }
}

#endif
