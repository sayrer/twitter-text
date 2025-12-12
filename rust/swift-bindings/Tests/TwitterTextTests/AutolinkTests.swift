import Foundation
import TwitterText

#if os(macOS)
import XCTest

final class AutolinkTests: XCTestCase {
    var autolinker: Autolinker!
    var yaml: [String: Any]!

    override func setUp() {
        super.setUp()
        autolinker = Autolinker(noFollow: false)
        yaml = loadYAML(from: "autolink.yml")
        XCTAssertNotNil(yaml, "Could not load autolink.yml")
    }

    func testUsernames() {
        let tests = parseAutolinkTests(from: yaml, key: "usernames")
        XCTAssertGreaterThan(tests.count, 0, "No username tests found")
        for test in tests {
            let result = autolinker.autolinkUsernamesAndLists(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testLists() {
        let tests = parseAutolinkTests(from: yaml, key: "lists")
        XCTAssertGreaterThan(tests.count, 0, "No list tests found")
        for test in tests {
            let result = autolinker.autolinkUsernamesAndLists(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testHashtags() {
        let tests = parseAutolinkTests(from: yaml, key: "hashtags")
        XCTAssertGreaterThan(tests.count, 0, "No hashtag tests found")
        for test in tests {
            let result = autolinker.autolinkHashtags(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testURLs() {
        let tests = parseAutolinkTests(from: yaml, key: "urls")
        XCTAssertGreaterThan(tests.count, 0, "No URL tests found")
        for test in tests {
            let result = autolinker.autolinkURLs(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testCashtags() {
        let tests = parseAutolinkTests(from: yaml, key: "cashtags")
        XCTAssertGreaterThan(tests.count, 0, "No cashtag tests found")
        for test in tests {
            let result = autolinker.autolinkCashtags(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testAll() {
        let tests = parseAutolinkTests(from: yaml, key: "all")
        XCTAssertGreaterThan(tests.count, 0, "No combined autolink tests found")
        for test in tests {
            let result = autolinker.autolink(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }
}

#else

class AutolinkTests {
    var autolinker: Autolinker!
    var yaml: [String: Any]!

    func setUp() {
        autolinker = Autolinker(noFollow: false)
        yaml = loadYAML(from: "autolink.yml")
    }

    func run() {
        print("\n=== Autolink Conformance Tests ===")
        setUp()
        XCTAssertNotNil(yaml, "Could not load autolink.yml")
        guard yaml != nil else { return }

        testUsernames()
        testLists()
        testHashtags()
        testURLs()
        testCashtags()
        testAll()
    }

    func testUsernames() {
        let tests = parseAutolinkTests(from: yaml, key: "usernames")
        print("\nUsernames (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No username tests found")
        for test in tests {
            let result = autolinker.autolinkUsernamesAndLists(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testLists() {
        let tests = parseAutolinkTests(from: yaml, key: "lists")
        print("\nLists (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No list tests found")
        for test in tests {
            let result = autolinker.autolinkUsernamesAndLists(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testHashtags() {
        let tests = parseAutolinkTests(from: yaml, key: "hashtags")
        print("\nHashtags (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No hashtag tests found")
        for test in tests {
            let result = autolinker.autolinkHashtags(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testURLs() {
        let tests = parseAutolinkTests(from: yaml, key: "urls")
        print("\nURLs (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No URL tests found")
        for test in tests {
            let result = autolinker.autolinkURLs(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testCashtags() {
        let tests = parseAutolinkTests(from: yaml, key: "cashtags")
        print("\nCashtags (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No cashtag tests found")
        for test in tests {
            let result = autolinker.autolinkCashtags(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testAll() {
        let tests = parseAutolinkTests(from: yaml, key: "all")
        print("\nAll (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No combined autolink tests found")
        for test in tests {
            let result = autolinker.autolink(test.text)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }
}

#endif
