import Foundation
import TwitterText

#if os(macOS)
import XCTest

final class TldTests: XCTestCase {
    var extractor: Extractor!
    var yaml: [String: Any]!

    override func setUp() {
        super.setUp()
        extractor = Extractor()
        yaml = loadYAML(from: "tlds.yml")
        XCTAssertNotNil(yaml, "Could not load tlds.yml")
    }

    func testCountryTLDs() {
        let tests = parseTldTests(from: yaml, key: "country")
        XCTAssertGreaterThan(tests.count, 0, "No country TLD tests found")
        for test in tests {
            let urls = extractor.extractURLs(from: test.text)
            XCTAssertFalse(urls.isEmpty, "\(test.description): no URLs extracted")
            if !urls.isEmpty {
                XCTAssertEqual(urls[0].value, test.expected, test.description)
            }
        }
    }

    func testGenericTLDs() {
        let tests = parseTldTests(from: yaml, key: "generic")
        XCTAssertGreaterThan(tests.count, 0, "No generic TLD tests found")
        for test in tests {
            let urls = extractor.extractURLs(from: test.text)
            XCTAssertFalse(urls.isEmpty, "\(test.description): no URLs extracted")
            if !urls.isEmpty {
                XCTAssertEqual(urls[0].value, test.expected, test.description)
            }
        }
    }
}

#else

class TldTests {
    var extractor: Extractor!
    var yaml: [String: Any]!

    func setUp() {
        extractor = Extractor()
        yaml = loadYAML(from: "tlds.yml")
    }

    func run() {
        print("\n=== TLD Conformance Tests ===")
        setUp()
        XCTAssertNotNil(yaml, "Could not load tlds.yml")
        guard yaml != nil else { return }

        testCountryTLDs()
        testGenericTLDs()
    }

    func testCountryTLDs() {
        let tests = parseTldTests(from: yaml, key: "country")
        print("\nCountry TLDs (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No country TLD tests found")
        for test in tests {
            let urls = extractor.extractURLs(from: test.text)
            XCTAssertFalse(urls.isEmpty, "\(test.description): no URLs extracted")
            if !urls.isEmpty {
                XCTAssertEqual(urls[0].value, test.expected, test.description)
            }
        }
    }

    func testGenericTLDs() {
        let tests = parseTldTests(from: yaml, key: "generic")
        print("\nGeneric TLDs (\(tests.count) tests):")
        XCTAssertGreaterThan(tests.count, 0, "No generic TLD tests found")
        for test in tests {
            let urls = extractor.extractURLs(from: test.text)
            XCTAssertFalse(urls.isEmpty, "\(test.description): no URLs extracted")
            if !urls.isEmpty {
                XCTAssertEqual(urls[0].value, test.expected, test.description)
            }
        }
    }
}

#endif
