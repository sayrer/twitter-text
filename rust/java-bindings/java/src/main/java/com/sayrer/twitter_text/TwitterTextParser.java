package com.sayrer.twitter_text;

import com.sayrer.twitter_text.twitter_text_c_h;
import com.sayrer.twitter_text.configuration_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

/**
 * Main parser for tweet text.
 * Provides static methods to parse tweets and get validation results.
 *
 * Example usage:
 * <pre>
 * ParseResults result = TwitterTextParser.parseTweetWithoutUrlExtraction("Hello, world!");
 * System.out.println("Weighted length: " + result.weightedLength);
 * System.out.println("Is valid: " + result.isValid);
 * </pre>
 */
public final class TwitterTextParser {

    /**
     * Configuration with code point counting (legacy v1 behavior).
     */
    public static final TwitterTextConfiguration TWITTER_TEXT_CODE_POINT_COUNT_CONFIG =
        TwitterTextConfiguration.configurationFromJson("v1.json", true);

    /**
     * Configuration for v2 weighted tweet counting.
     */
    public static final TwitterTextConfiguration TWITTER_TEXT_WEIGHTED_CHAR_COUNT_CONFIG =
        TwitterTextConfiguration.configurationFromJson("v2.json", true);

    /**
     * Configuration with emoji character counting discounted.
     */
    public static final TwitterTextConfiguration TWITTER_TEXT_EMOJI_CHAR_COUNT_CONFIG =
        TwitterTextConfiguration.configurationFromJson("v3.json", true);

    private TwitterTextParser() {
        // Utility class, no instantiation
    }

    /**
     * Parse tweet text without URL extraction.
     * URLs are counted by their actual length, not transformed.
     *
     * @param text the tweet text to parse
     * @return ParseResults containing weighted length, validity, and ranges
     */
    public static ParseResults parseTweetWithoutUrlExtraction(String text) {
        return parse(text, null, false);
    }

    /**
     * Parse tweet text with URL extraction.
     * URLs are weighted using the configuration's transformed URL length.
     *
     * @param text the tweet text to parse
     * @return ParseResults containing weighted length, validity, and ranges
     */
    public static ParseResults parseTweetWithUrlExtraction(String text) {
        return parse(text, null, true);
    }

    /**
     * Parse tweet text (compatibility method).
     * Uses default configuration with URL extraction.
     *
     * @param text the tweet text to parse
     * @return ParseResults containing weighted length, validity, and ranges
     */
    public static ParseResults parseTweet(String text) {
        return parseTweetWithUrlExtraction(text);
    }

    /**
     * Parse tweet text with custom configuration (compatibility method).
     *
     * @param text the tweet text to parse
     * @param config the TwitterTextConfiguration to use
     * @return ParseResults containing weighted length, validity, and ranges
     */
    public static ParseResults parseTweet(String text, TwitterTextConfiguration config) {
        if (config == null) {
            return parseTweetWithUrlExtraction(text);
        }
        return parse(text, config.getConfig(), true);
    }

    /**
     * Parse tweet text with a custom configuration.
     *
     * @param text the tweet text to parse
     * @param config the Configuration to use (null for default)
     * @param extractUrls whether to extract and weight URLs
     * @return ParseResults containing weighted length, validity, and ranges
     */
    public static ParseResults parse(String text, Configuration config, boolean extractUrls) {
        try (Arena arena = Arena.ofConfined()) {
            // Handle null text
            MemorySegment textSegment;
            if (text == null) {
                textSegment = MemorySegment.NULL;
            } else {
                textSegment = arena.allocateFrom(text);
            }

            // Get or create config
            MemorySegment configSegment;
            boolean needToFreeConfig = false;

            if (config == null) {
                // Create default config temporarily
                configSegment = (MemorySegment) configuration_h
                    .twitter_text_config_default$handle()
                    .invoke();
                needToFreeConfig = true;
            } else {
                configSegment = config.getHandle();
            }

            try {
                // Call the C function
                // The function returns a struct by value, so we need to allocate space for it
                // and pass a SegmentAllocator
                MemorySegment resultSegment = (MemorySegment) twitter_text_c_h
                    .twitter_text_parse$handle()
                    .invoke(arena, textSegment, configSegment, extractUrls);

                // Extract the fields from the returned struct
                // The struct layout is:
                // struct TwitterTextParseResults {
                //     int32_t weighted_length;
                //     int32_t permillage;
                //     bool is_valid;
                //     TwitterTextRange display_text_range;
                //     TwitterTextRange valid_text_range;
                // }
                //
                // struct TwitterTextRange {
                //     int32_t start;
                //     int32_t end;
                // }

                int weightedLength = resultSegment.get(ValueLayout.JAVA_INT, 0);
                int permillage = resultSegment.get(ValueLayout.JAVA_INT, 4);
                boolean isValid = resultSegment.get(ValueLayout.JAVA_BOOLEAN, 8);

                // Display text range starts at offset 12 (after padding)
                int displayStart = resultSegment.get(ValueLayout.JAVA_INT, 12);
                int displayEnd = resultSegment.get(ValueLayout.JAVA_INT, 16);

                // Valid text range starts at offset 20
                int validStart = resultSegment.get(ValueLayout.JAVA_INT, 20);
                int validEnd = resultSegment.get(ValueLayout.JAVA_INT, 24);

                return new ParseResults(
                    weightedLength,
                    permillage,
                    isValid,
                    new Range(displayStart, displayEnd),
                    new Range(validStart, validEnd)
                );
            } finally {
                // Free the temporary config if we created one
                if (needToFreeConfig) {
                    configuration_h
                        .twitter_text_config_free$handle()
                        .invoke(configSegment);
                }
            }
        } catch (Throwable t) {
            throw new RuntimeException("Failed to parse tweet", t);
        }
    }
}
