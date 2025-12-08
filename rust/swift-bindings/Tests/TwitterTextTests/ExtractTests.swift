import Foundation
import TwitterText

#if os(macOS)
import XCTest

final class ExtractTests: XCTestCase {
    var extractor: Extractor!
    var yaml: [String: Any]!

    override func setUp() {
        super.setUp()
        extractor = Extractor()
        yaml = loadYAML(from: "extract.yml")
        XCTAssertNotNil(yaml, "Could not load extract.yml")
    }

    func testMentions() {
        let tests = parseExtractTests(from: yaml, key: "mentions")
        XCTAssertGreaterThan(tests.count, 0, "No mention tests found")
        for test in tests {
            let entities = extractor.extractMentions(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }

    func testURLs() {
        let tests = parseExtractTests(from: yaml, key: "urls")
        XCTAssertGreaterThan(tests.count, 0, "No URL tests found")
        for test in tests {
            let entities = extractor.extractURLs(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }

    func testHashtags() {
        let tests = parseExtractTests(from: yaml, key: "hashtags")
        XCTAssertGreaterThan(tests.count, 0, "No hashtag tests found")
        for test in tests {
            let entities = extractor.extractHashtags(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }

    func testCashtags() {
        let tests = parseExtractTests(from: yaml, key: "cashtags")
        XCTAssertGreaterThan(tests.count, 0, "No cashtag tests found")
        for test in tests {
            let entities = extractor.extractCashtags(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }
}

#else

class ExtractTests {
    var extractor: Extractor!
    var yaml: [String: Any]!

    func setUp() {
        extractor = Extractor()
        yaml = loadYAML(from: "extract.yml")
    }

    func run() {
        print("\n=== Extract Conformance Tests ===")
        setUp()
        XCTAssertNotNil(yaml, "Could not load extract.yml")
        guard yaml != nil else { return }

        testMentions()
        testURLs()
        testHashtags()
        testCashtags()
    }

    func testMentions() {
        let tests = parseExtractTests(from: yaml, key: "mentions")
        print("\nMentions (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No mention tests found")
        for test in tests {
            let entities = extractor.extractMentions(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }

    func testURLs() {
        let tests = parseExtractTests(from: yaml, key: "urls")
        print("\nURLs (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No URL tests found")
        for test in tests {
            let entities = extractor.extractURLs(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }

    func testHashtags() {
        let tests = parseExtractTests(from: yaml, key: "hashtags")
        print("\nHashtags (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No hashtag tests found")
        for test in tests {
            let entities = extractor.extractHashtags(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }

    func testCashtags() {
        let tests = parseExtractTests(from: yaml, key: "cashtags")
        print("\nCashtags (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No cashtag tests found")
        for test in tests {
            let entities = extractor.extractCashtags(from: test.text)
            let values = entities.map { $0.value }
            XCTAssertEqual(values, test.expected, test.description)
        }
    }
}

#endif
