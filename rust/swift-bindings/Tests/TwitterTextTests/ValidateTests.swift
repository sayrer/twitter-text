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
}

#endif
