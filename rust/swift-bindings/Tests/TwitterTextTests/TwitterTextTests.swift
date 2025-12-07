import XCTest
@testable import TwitterText

final class TwitterTextTests: XCTestCase {
    func testConfigurationDefault() {
        let config = Configuration.default
        XCTAssertGreaterThan(config.maxWeightedTweetLength, 0)
        XCTAssertGreaterThan(config.scale, 0)
    }

    func testConfigurationVersions() {
        let v1 = Configuration.v1
        let v2 = Configuration.v2
        let v3 = Configuration.v3

        XCTAssertGreaterThan(v1.maxWeightedTweetLength, 0)
        XCTAssertGreaterThan(v2.maxWeightedTweetLength, 0)
        XCTAssertGreaterThan(v3.maxWeightedTweetLength, 0)
    }

    func testExtractorURLs() {
        let extractor = Extractor()
        let text = "Check out https://example.com and http://test.org"
        let urls = extractor.extractURLs(from: text)

        XCTAssertEqual(urls.count, 2)
        XCTAssertEqual(urls[0].type, .url)
        XCTAssertEqual(urls[0].value, "https://example.com")
        XCTAssertEqual(urls[1].value, "http://test.org")
    }

    func testExtractorHashtags() {
        let extractor = Extractor()
        let text = "Tweet about #swift and #coding"
        let hashtags = extractor.extractHashtags(from: text)

        XCTAssertEqual(hashtags.count, 2)
        XCTAssertEqual(hashtags[0].type, .hashtag)
        XCTAssertTrue(hashtags[0].value.contains("swift"))
        XCTAssertTrue(hashtags[1].value.contains("coding"))
    }

    func testExtractorMentions() {
        let extractor = Extractor()
        let text = "Hello @user1 and @user2"
        let mentions = extractor.extractMentions(from: text)

        XCTAssertEqual(mentions.count, 2)
        XCTAssertEqual(mentions[0].type, .mention)
        XCTAssertTrue(mentions[0].value.contains("user1"))
        XCTAssertTrue(mentions[1].value.contains("user2"))
    }

    func testExtractorCashtags() {
        let extractor = Extractor()
        let text = "Stock symbols: $AAPL and $GOOGL"
        let cashtags = extractor.extractCashtags(from: text)

        XCTAssertEqual(cashtags.count, 2)
        XCTAssertEqual(cashtags[0].type, .cashtag)
        XCTAssertTrue(cashtags[0].value.contains("AAPL"))
        XCTAssertTrue(cashtags[1].value.contains("GOOGL"))
    }

    func testAutolinker() {
        let autolinker = Autolinker()
        let text = "Check out https://example.com"
        let linked = autolinker.autolink(text)

        XCTAssertTrue(linked.contains("<a"))
        XCTAssertTrue(linked.contains("href"))
        XCTAssertTrue(linked.contains("https://example.com"))
    }

    func testAutolinkerHashtags() {
        let autolinker = Autolinker()
        let text = "Tweet about #swift"
        let linked = autolinker.autolinkHashtags(text)

        XCTAssertTrue(linked.contains("<a"))
        XCTAssertTrue(linked.contains("#swift"))
    }

    func testAutolinkerMentions() {
        let autolinker = Autolinker()
        let text = "Hello @user"
        let linked = autolinker.autolinkUsernamesAndLists(text)

        XCTAssertTrue(linked.contains("<a"))
        XCTAssertTrue(linked.contains("@user"))
    }
}
