import Foundation
import TwitterText

// Simple test runner for Linux (no XCTest dependency)
func assert(_ condition: Bool, _ message: String = "") {
    if !condition {
        print("❌ FAILED: \(message)")
        Foundation.exit(1)
    }
}

func assertEqual<T: Equatable>(_ lhs: T, _ rhs: T, _ message: String = "") {
    if lhs != rhs {
        print("❌ FAILED: \(message) - Expected \(rhs), got \(lhs)")
        Foundation.exit(1)
    }
}

func testConfigurationDefault() {
    let config = Configuration.default
    assert(config.maxWeightedTweetLength > 0, "maxWeightedTweetLength should be > 0")
    assert(config.scale > 0, "scale should be > 0")
    print("✅ testConfigurationDefault passed")
}

func testConfigurationV3() {
    let v3 = Configuration.v3
    assert(v3.maxWeightedTweetLength > 0, "v3 maxWeightedTweetLength should be > 0")
    print("✅ testConfigurationV3 passed")
}

func testExtractorURLs() {
    let extractor = Extractor()
    let text = "Check out https://example.com and http://test.org"
    let urls = extractor.extractURLs(from: text)

    assertEqual(urls.count, 2, "Should extract 2 URLs")
    assertEqual(urls[0].type, .url, "First entity should be URL type")
    assertEqual(urls[0].value, "https://example.com", "First URL value")
    assertEqual(urls[1].value, "http://test.org", "Second URL value")
    print("✅ testExtractorURLs passed")
}

func testExtractorHashtags() {
    let extractor = Extractor()
    let text = "Tweet about #swift and #coding"
    let hashtags = extractor.extractHashtags(from: text)

    assertEqual(hashtags.count, 2, "Should extract 2 hashtags")
    assertEqual(hashtags[0].type, .hashtag, "First entity should be hashtag type")
    assert(hashtags[0].value.contains("swift"), "First hashtag should contain 'swift'")
    assert(hashtags[1].value.contains("coding"), "Second hashtag should contain 'coding'")
    print("✅ testExtractorHashtags passed")
}

func testExtractorMentions() {
    let extractor = Extractor()
    let text = "Hello @user1 and @user2"
    let mentions = extractor.extractMentions(from: text)

    assertEqual(mentions.count, 2, "Should extract 2 mentions")
    assertEqual(mentions[0].type, .mention, "First entity should be mention type")
    assert(mentions[0].value.contains("user1"), "First mention should contain 'user1'")
    assert(mentions[1].value.contains("user2"), "Second mention should contain 'user2'")
    print("✅ testExtractorMentions passed")
}

func testExtractorCashtags() {
    let extractor = Extractor()
    let text = "Stock symbols: $AAPL and $GOOGL"
    let cashtags = extractor.extractCashtags(from: text)

    assertEqual(cashtags.count, 2, "Should extract 2 cashtags")
    assertEqual(cashtags[0].type, .cashtag, "First entity should be cashtag type")
    assert(cashtags[0].value.contains("AAPL"), "First cashtag should contain 'AAPL'")
    assert(cashtags[1].value.contains("GOOGL"), "Second cashtag should contain 'GOOGL'")
    print("✅ testExtractorCashtags passed")
}

func testAutolinker() {
    let autolinker = Autolinker()
    let text = "Check out https://example.com"
    let linked = autolinker.autolink(text)

    assert(linked.contains("<a"), "Should contain anchor tag")
    assert(linked.contains("href"), "Should contain href attribute")
    assert(linked.contains("https://example.com"), "Should contain the URL")
    print("✅ testAutolinker passed")
}

func testAutolinkerHashtags() {
    let autolinker = Autolinker()
    let text = "Tweet about #swift"
    let linked = autolinker.autolinkHashtags(text)

    assert(linked.contains("<a"), "Should contain anchor tag")
    assert(linked.contains("#swift"), "Should contain the hashtag")
    print("✅ testAutolinkerHashtags passed")
}

func testAutolinkerMentions() {
    let autolinker = Autolinker()
    let text = "Hello @user"
    let linked = autolinker.autolinkUsernamesAndLists(text)

    assert(linked.contains("<a"), "Should contain anchor tag")
    assert(linked.contains("@user"), "Should contain the mention")
    print("✅ testAutolinkerMentions passed")
}

// Main entry point
@main
struct TestRunner {
    static func main() {
        testConfigurationDefault()
        testConfigurationV3()
        testExtractorURLs()
        testExtractorHashtags()
        testExtractorMentions()
        testExtractorCashtags()
        testAutolinker()
        testAutolinkerHashtags()
        testAutolinkerMentions()

        print("\n🎉 All tests passed!")
    }
}
