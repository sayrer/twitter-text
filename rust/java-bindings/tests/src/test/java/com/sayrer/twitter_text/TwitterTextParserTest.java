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

  public void testRunicWeighting() {
    // Runes weigh one unit each under v4, two under v3 and under the default.
    // https://github.com/twitter/twitter-text/issues/430
    final String runes = "\u16A0\u16A2\u16A6\u16A8\u16B1\u16B2";

    ParseResults v4 = TwitterTextParser.parseTweet(runes, TwitterTextParser.TWITTER_TEXT_V4_CONFIG);
    assertEquals("V4: runes weigh one unit each", 6, v4.weightedLength);
    assertTrue("V4: Should be valid", v4.isValid);

    ParseResults v3 =
        TwitterTextParser.parseTweet(runes, TwitterTextParser.TWITTER_TEXT_EMOJI_CHAR_COUNT_CONFIG);
    assertEquals("V3: runes take the default weight", 12, v3.weightedLength);

    // v4 is opt-in: the default configuration still weighs runes as logograms.
    assertEquals("Default: runes take the default weight", 12,
        TwitterTextParser.parseTweet(runes).weightedLength);
  }

  public void testAlphabeticScriptsWeighLikeLatinInV4() {
    // Six letters from an alphabetic block above U+10FF weigh six units under
    // v4 and twelve under v3 and under the default.
    final String[][] samples = {
      {"Latin Extended Additional", "\u1E00\u1E01\u1E02\u1E03\u1E04\u1E05"},
      {"Greek Extended", "\u1F00\u1F01\u1F02\u1F03\u1F04\u1F05"},
      {"Ethiopic", "\u1200\u1208\u1210\u1218\u1220\u1228"},
      {"Cherokee", "\u13A0\u13A1\u13A2\u13A3\u13A4\u13A5"},
      {"Khmer", "\u1780\u1781\u1782\u1783\u1784\u1785"},
      {"Arabic Presentation Forms-A", "\uFB50\uFB51\uFB52\uFB53\uFB54\uFB55"},
      // Adlam is outside the BMP, so each letter is a surrogate pair in Java.
      {"Adlam", "\uD83A\uDD00\uD83A\uDD01\uD83A\uDD02\uD83A\uDD03\uD83A\uDD04\uD83A\uDD05"},
    };
    for (String[] sample : samples) {
      final String name = sample[0];
      final String text = sample[1];
      assertEquals(name + ": six code points", 6, text.codePointCount(0, text.length()));

      ParseResults v4 = TwitterTextParser.parseTweet(text, TwitterTextParser.TWITTER_TEXT_V4_CONFIG);
      assertEquals(name + ": V4 weighs one unit per letter", 6, v4.weightedLength);
      assertTrue(name + ": V4 should be valid", v4.isValid);

      ParseResults v3 =
          TwitterTextParser.parseTweet(text, TwitterTextParser.TWITTER_TEXT_EMOJI_CHAR_COUNT_CONFIG);
      assertEquals(name + ": V3 takes the default weight", 12, v3.weightedLength);

      assertEquals(name + ": Default takes the default weight", 12,
          TwitterTextParser.parseTweet(text).weightedLength);
    }
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
