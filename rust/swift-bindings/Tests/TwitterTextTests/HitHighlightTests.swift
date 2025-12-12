import Foundation
import TwitterText

#if os(macOS)
import XCTest

final class HitHighlightTests: XCTestCase {
    var highlighter: HitHighlighter!
    var yaml: [String: Any]!

    override func setUp() {
        super.setUp()
        highlighter = HitHighlighter()
        yaml = loadYAML(from: "hit_highlighting.yml")
        XCTAssertNotNil(yaml, "Could not load hit_highlighting.yml")
    }

    func testPlainText() {
        let tests = parseHitHighlightTests(from: yaml, key: "plain_text")
        XCTAssertGreaterThan(tests.count, 0, "No plain text tests found")
        for test in tests {
            let hits = test.hits.map { Hit(start: $0[0], end: $0[1]) }
            let result = highlighter.highlight(test.text, hits: hits)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testWithLinks() {
        let tests = parseHitHighlightTests(from: yaml, key: "with_links")
        XCTAssertGreaterThan(tests.count, 0, "No with links tests found")
        for test in tests {
            let hits = test.hits.map { Hit(start: $0[0], end: $0[1]) }
            let result = highlighter.highlight(test.text, hits: hits)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }
}

#else

class HitHighlightTests {
    var highlighter: HitHighlighter!
    var yaml: [String: Any]!

    func setUp() {
        highlighter = HitHighlighter()
        yaml = loadYAML(from: "hit_highlighting.yml")
    }

    func run() {
        print("\n=== Hit Highlighting Conformance Tests ===")
        setUp()
        XCTAssertNotNil(yaml, "Could not load hit_highlighting.yml")
        guard yaml != nil else { return }

        testPlainText()
        testWithLinks()
    }

    func testPlainText() {
        let tests = parseHitHighlightTests(from: yaml, key: "plain_text")
        print("\nPlain text (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No plain text tests found")
        for test in tests {
            let hits = test.hits.map { Hit(start: $0[0], end: $0[1]) }
            let result = highlighter.highlight(test.text, hits: hits)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }

    func testWithLinks() {
        let tests = parseHitHighlightTests(from: yaml, key: "with_links")
        print("\nWith links (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No with links tests found")
        for test in tests {
            let hits = test.hits.map { Hit(start: $0[0], end: $0[1]) }
            let result = highlighter.highlight(test.text, hits: hits)
            XCTAssertEqual(result, test.expected, test.description)
        }
    }
}

#endif
