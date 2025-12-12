// Copyright 2018 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

package com.sayrer.twitter_text;

import junit.framework.TestCase;

public class TwitterTextParserTest extends TestCase {
  public void testparseTweetWithoutUrlExtraction() {
    assertEquals("Handle null input", 0,
        TwitterTextParser.parseTweetWithoutUrlExtraction(null).weightedLength);
    assertEquals("Handle empty input", 0,
        TwitterTextParser.parseTweetWithoutUrlExtraction("").weightedLength);
    assertEquals("Count Latin chars normally", 11,
        TwitterTextParser.parseTweetWithoutUrlExtraction("Normal Text").weightedLength);
    assertEquals("Count hashtags, @mentions and cashtags normally", 38,
        TwitterTextParser.parseTweetWithoutUrlExtraction("Text with #hashtag, @mention and $CASH")
            .weightedLength);
    assertEquals("Count CJK chars with their appropriate weights", 94,
        TwitterTextParser.parseTweetWithoutUrlExtraction("CJK Weighted chars: " +
            "あいうえおかきくけこあいうえおかきくけこあいうえおかきくけこあいうえおかき").weightedLength);
    assertEquals("URLs should be counted without transformation", 69,
        TwitterTextParser.parseTweetWithoutUrlExtraction("Text with url: " +
            "a.com http://abc.com https://longurllongurllongurl.com").weightedLength);
    assertEquals("t.co URLs should be counted without transformation", 39,
        TwitterTextParser.parseTweetWithoutUrlExtraction("Text with t.co url: https://t.co/foobar")
            .weightedLength);
  }

  public void testWeightedLengthMixedUnicodeAndEmojiV2() {
    // Test case from conformance suite that requires v2 config
    // Text: "H🐱☺👨‍👩‍👧‍👦"
    // Expected weighted_length: 16 (emoji families counted as separate components)
    ParseResults result = TwitterTextParser.parseTweet("H🐱☺👨‍👩‍👧‍👦",
        TwitterTextParser.TWITTER_TEXT_WEIGHTED_CHAR_COUNT_CONFIG);
    assertEquals("V2: Mixed single/double byte Unicode and emoji family counting", 16,
        result.weightedLength);
    assertTrue("V2: Should be valid", result.isValid);
    assertEquals("V2: Permillage", 57, result.permillage);
  }

  public void testWeightedLengthEmojiWithSkinToneModifiersV2() {
    // Test case from conformance suite that requires v2 config
    // Text: "🙋🏽👨‍🎤"
    // Expected weighted_length: 9 (skin tone modifiers counted separately)
    ParseResults result = TwitterTextParser.parseTweet("🙋🏽👨‍🎤",
        TwitterTextParser.TWITTER_TEXT_WEIGHTED_CHAR_COUNT_CONFIG);
    assertEquals("V2: Emoji with skin tone modifiers counting", 9, result.weightedLength);
    assertTrue("V2: Should be valid", result.isValid);
    assertEquals("V2: Permillage", 32, result.permillage);
  }

  public void testWeightedLengthMixedUnicodeAndEmojiV3() {
    // Same test as above but with v3 config (emoji parsing enabled)
    // Text: "H🐱☺👨‍👩‍👧‍👦"
    // With v3 config, emoji families are counted as single units
    ParseResults result = TwitterTextParser.parseTweet("H🐱☺👨‍👩‍👧‍👦",
        TwitterTextParser.TWITTER_TEXT_EMOJI_CHAR_COUNT_CONFIG);
    assertEquals("V3: Mixed single/double byte Unicode and emoji family counting", 7,
        result.weightedLength);
    assertTrue("V3: Should be valid", result.isValid);
    assertEquals("V3: Permillage", 25, result.permillage);
  }

  public void testWeightedLengthEmojiWithSkinToneModifiersV3() {
    // Same test as above but with v3 config (emoji parsing enabled)
    // Text: "🙋🏽👨‍🎤"
    // With v3 config, emojis with modifiers are counted as single units
    ParseResults result = TwitterTextParser.parseTweet("🙋🏽👨‍🎤",
        TwitterTextParser.TWITTER_TEXT_EMOJI_CHAR_COUNT_CONFIG);
    assertEquals("V3: Emoji with skin tone modifiers counting", 4, result.weightedLength);
    assertTrue("V3: Should be valid", result.isValid);
    assertEquals("V3: Permillage", 14, result.permillage);
  }
}
