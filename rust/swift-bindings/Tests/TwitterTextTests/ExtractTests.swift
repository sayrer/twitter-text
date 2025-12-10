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

    func testFederatedMentions() {
        // Simple federated mention
        let entities1 = extractor.extractFederatedMentions(from: "Hello @user@mastodon.social!")
        XCTAssertEqual(entities1.count, 1)
        XCTAssertEqual(entities1[0].value, "@user@mastodon.social")
        XCTAssertEqual(entities1[0].type, .federatedMention)
        XCTAssertEqual(entities1[0].start, 6)
        XCTAssertEqual(entities1[0].end, 27)

        // Multiple federated mentions
        let entities2 = extractor.extractFederatedMentions(from: "@alice@example.com and @bob@other.org")
        XCTAssertEqual(entities2.count, 2)
        XCTAssertEqual(entities2[0].value, "@alice@example.com")
        XCTAssertEqual(entities2[1].value, "@bob@other.org")

        // Mixed with regular mentions (extractFederatedMentions includes both)
        let entities3 = extractor.extractFederatedMentions(from: "@regular and @federated@domain.com")
        XCTAssertEqual(entities3.count, 2)
        XCTAssertEqual(entities3[0].value, "regular")
        XCTAssertEqual(entities3[0].type, .mention)
        XCTAssertEqual(entities3[1].value, "@federated@domain.com")
        XCTAssertEqual(entities3[1].type, .federatedMention)
    }

    func testEntitiesWithIndicesFederated() {
        // Test that extractEntitiesWithIndicesFederated includes federated mentions
        let entities = extractor.extractEntitiesWithIndicesFederated(
            from: "Check @user@mastodon.social and https://example.com #hashtag $CASH"
        )

        // Should have: URL, hashtag, cashtag, federated mention
        XCTAssertEqual(entities.count, 4)

        // Find the federated mention
        let federated = entities.first { $0.type == .federatedMention }
        XCTAssertNotNil(federated)
        XCTAssertEqual(federated?.value, "@user@mastodon.social")

        // Regular extractEntities should NOT include federated mentions
        let regularEntities = extractor.extractEntities(
            from: "Check @user@mastodon.social and https://example.com #hashtag $CASH"
        )
        let federatedInRegular = regularEntities.first { $0.type == .federatedMention }
        XCTAssertNil(federatedInRegular)
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
        testFederatedMentions()
        testEntitiesWithIndicesFederated()
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

    func testFederatedMentions() {
        print("\nFederated Mentions:")
        // Simple federated mention
        let entities1 = extractor.extractFederatedMentions(from: "Hello @user@mastodon.social!")
        XCTAssertEqual(entities1.count, 1, "Simple federated mention count")
        XCTAssertEqual(entities1[0].value, "@user@mastodon.social", "Simple federated mention value")
        XCTAssertEqual(entities1[0].type, .federatedMention, "Simple federated mention type")

        // Multiple federated mentions
        let entities2 = extractor.extractFederatedMentions(from: "@alice@example.com and @bob@other.org")
        XCTAssertEqual(entities2.count, 2, "Multiple federated mentions count")
        XCTAssertEqual(entities2[0].value, "@alice@example.com", "First federated mention")
        XCTAssertEqual(entities2[1].value, "@bob@other.org", "Second federated mention")

        // Mixed with regular mentions
        let entities3 = extractor.extractFederatedMentions(from: "@regular and @federated@domain.com")
        XCTAssertEqual(entities3.count, 2, "Mixed mentions count")
        XCTAssertEqual(entities3[0].type, .mention, "Regular mention type")
        XCTAssertEqual(entities3[1].type, .federatedMention, "Federated mention type")
    }

    func testEntitiesWithIndicesFederated() {
        print("\nEntities With Indices Federated:")
        let entities = extractor.extractEntitiesWithIndicesFederated(
            from: "Check @user@mastodon.social and https://example.com #hashtag $CASH"
        )
        XCTAssertEqual(entities.count, 4, "All entities including federated")

        let federated = entities.first { $0.type == .federatedMention }
        XCTAssertNotNil(federated, "Should find federated mention")

        let regularEntities = extractor.extractEntities(
            from: "Check @user@mastodon.social and https://example.com #hashtag $CASH"
        )
        let federatedInRegular = regularEntities.first { $0.type == .federatedMention }
        XCTAssertNil(federatedInRegular, "Regular entities should not include federated")
    }
}

#endif
