package com.sayrer.twitter_text.examples;

import com.sayrer.twitter_text.Autolink;
import com.sayrer.twitter_text.Extractor;
import com.sayrer.twitter_text.Validator;

/**
 * Example demonstrating how to use the twitter-text FFM wrappers.
 *
 * This example shows:
 * - Autolinking entities in text
 * - Extracting URLs, hashtags, mentions, and cashtags
 * - Validating tweets, usernames, and other entities
 */
public class Example {

    public static void main(String[] args) {
        System.out.println("=== Twitter Text FFM Examples ===\n");

        autolinkExample();
        extractorExample();
        validatorExample();
    }

    private static void autolinkExample() {
        System.out.println("--- Autolinker Example ---");

        String text =
            "Check out https://example.com, follow @twitter, use #java, and invest in $TWTR!";
        System.out.println("Input: " + text);

        try (Autolink autolink = Autolink.create(true)) {
            // Configure the autolinker
            autolink.setUrlClass("tweet-url");
            autolink.setHashtagClass("tweet-hashtag");
            autolink.setUsernameClass("tweet-mention");
            autolink.setCashtagClass("tweet-cashtag");

            // Auto-link all entities
            String linked = autolink.autolink(text);
            System.out.println("Output: " + linked);

            // Auto-link only URLs
            String urlsLinked = autolink.autolinkUrls(text);
            System.out.println("URLs only: " + urlsLinked);
        }

        System.out.println();
    }

    private static void extractorExample() {
        System.out.println("--- Extractor Example ---");

        String text =
            "RT @user: Check out https://example.com and https://test.org! #awesome #cool $AAPL $GOOGL";
        System.out.println("Input: " + text);

        try (Extractor extractor = Extractor.create()) {
            extractor.setExtractUrlWithoutProtocol(true);

            // Extract URLs
            String[] urls = extractor.extractUrls(text);
            System.out.println("URLs: ");
            for (String url : urls) {
                System.out.println("  - " + url);
            }

            // Extract hashtags
            String[] hashtags = extractor.extractHashtags(text);
            System.out.println("Hashtags: ");
            for (String hashtag : hashtags) {
                System.out.println("  - #" + hashtag);
            }

            // Extract mentions
            String[] mentions = extractor.extractMentionedScreennames(text);
            System.out.println("Mentions: ");
            for (String mention : mentions) {
                System.out.println("  - @" + mention);
            }

            // Extract cashtags
            String[] cashtags = extractor.extractCashtags(text);
            System.out.println("Cashtags: ");
            for (String cashtag : cashtags) {
                System.out.println("  - $" + cashtag);
            }

            // Extract reply username
            String replyText = "@user this is a reply";
            String replyUsername = extractor.extractReplyUsername(replyText);
            System.out.println("Reply to: @" + replyUsername);
        }

        System.out.println();
    }

    private static void validatorExample() {
        System.out.println("--- Validator Example ---");

        try (Validator validator = Validator.create()) {
            // Validate tweets
            String validTweet = "This is a valid tweet!";
            String tooLong = "a".repeat(300);

            System.out.println(
                "Is valid tweet '" +
                    validTweet +
                    "': " +
                    validator.isValidTweet(validTweet)
            );
            System.out.println(
                "Is valid tweet (300 chars): " + validator.isValidTweet(tooLong)
            );

            // Validate usernames
            System.out.println(
                "Is valid username 'twitter': " +
                    validator.isValidUsername("twitter")
            );
            System.out.println(
                "Is valid username 'inv@lid': " +
                    validator.isValidUsername("inv@lid")
            );

            // Validate hashtags
            System.out.println(
                "Is valid hashtag 'java': " + validator.isValidHashtag("java")
            );
            System.out.println(
                "Is valid hashtag '#invalid': " +
                    validator.isValidHashtag("#invalid")
            );

            // Validate URLs
            System.out.println(
                "Is valid URL 'https://example.com': " +
                    validator.isValidUrl("https://example.com")
            );
            System.out.println(
                "Is valid URL 'not a url': " + validator.isValidUrl("not a url")
            );

            // Validate URLs without protocol
            System.out.println(
                "Is valid URL without protocol 'example.com': " +
                    validator.isValidUrlWithoutProtocol("example.com")
            );

            // Get configuration
            System.out.println(
                "Max tweet length: " + validator.getMaxTweetLength()
            );
            System.out.println(
                "Short URL length: " + validator.getShortUrlLength()
            );
            System.out.println(
                "Short URL length (HTTPS): " +
                    validator.getShortUrlLengthHttps()
            );
        }

        System.out.println();
    }
}
