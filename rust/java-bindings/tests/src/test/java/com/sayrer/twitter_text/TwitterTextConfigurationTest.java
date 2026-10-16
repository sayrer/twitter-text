// Copyright 2018 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

package com.sayrer.twitter_text;

import java.util.List;

import junit.framework.TestCase;

public class TwitterTextConfigurationTest extends TestCase {

  public void testDefaultVersion() {
    assertEquals(TwitterTextConfiguration.configurationFromJson("", false),
        TwitterTextConfiguration.configurationFromJson("v2.json", true));
  }

  public void testInvalidJsonPathDoesntCrash() {
    assertEquals(TwitterTextConfiguration.configurationFromJson("unknown", true),
        TwitterTextConfiguration.configurationFromJson("v2.json", true));
  }

  public void testJsonAsString() {
    final TwitterTextConfiguration configuration =
        TwitterTextConfiguration.configurationFromJson("{\"version\": 1, " +
            "\"maxWeightedTweetLength\": 30, \"scale\": 1, \"defaultWeight\": 1, " +
            "\"transformedURLLength\": 14, \"ranges\": [{\"start\": 0, \"end\": 4351, " +
            "\"weight\": 2}]}", false);
    assertEquals(configuration.getVersion(), 1);
    assertEquals(configuration.getMaxWeightedTweetLength(), 30);
    assertEquals(configuration.getScale(), 1);
    assertEquals(configuration.getDefaultWeight(), 1);
    assertEquals(configuration.getTransformedURLLength(), 14);
    final List<TwitterTextConfiguration.TwitterTextWeightedRange> ranges =
        configuration.getRanges();
    assertNotNull(ranges);
    assertEquals(ranges.size(), 1);
    final TwitterTextConfiguration.TwitterTextWeightedRange latinCharRange = ranges.get(0);
    assertEquals(latinCharRange.getRange(), new Range(0, 4351));
    assertEquals(latinCharRange.getWeight(), 2);
  }

  public void testVersion1() {
    final TwitterTextConfiguration configuration =
        TwitterTextConfiguration.configurationFromJson("v1.json", true);
    assertEquals(configuration.getVersion(), 1);
    assertEquals(configuration.getMaxWeightedTweetLength(), 140);
    assertEquals(configuration.getScale(), 1);
    assertEquals(configuration.getDefaultWeight(), 1);
    assertEquals(configuration.getTransformedURLLength(), 23);
    assertEquals(configuration.getRanges().size(), 0);
  }

  public void testVersion4() {
    final TwitterTextConfiguration configuration =
        TwitterTextConfiguration.configurationFromJson("v4.json", true);
    assertEquals(configuration.getVersion(), 4);
    assertEquals(configuration.getMaxWeightedTweetLength(), 280);
    final List<TwitterTextConfiguration.TwitterTextWeightedRange> ranges =
        configuration.getRanges();
    assertNotNull(ranges);
    final List<TwitterTextConfiguration.TwitterTextWeightedRange> v3Ranges =
        TwitterTextConfiguration.configurationFromJson("v3.json", true).getRanges();
    // v4 is v3 plus the alphabetic scripts above U+10FF.
    assertTrue(ranges.size() > v3Ranges.size());
    for (TwitterTextConfiguration.TwitterTextWeightedRange v3Range : v3Ranges) {
      boolean found = false;
      for (TwitterTextConfiguration.TwitterTextWeightedRange range : ranges) {
        found |= range.getRange().equals(v3Range.getRange())
            && range.getWeight() == v3Range.getWeight();
      }
      assertTrue("v3 range " + v3Range.getRange() + " should be in v4", found);
    }
    assertTrue(covers(ranges, 0x16A0, 0x16FF));     // Runic
    assertTrue(covers(ranges, 0x1200, 0x137F));     // Ethiopic
    assertTrue(covers(ranges, 0x1780, 0x17FF));     // Khmer
    assertTrue(covers(ranges, 0x1E00, 0x1EFF));     // Latin Extended Additional
    assertTrue(covers(ranges, 0x1F00, 0x1FFF));     // Greek Extended
    assertTrue(covers(ranges, 0xFB50, 0xFDFF));     // Arabic Presentation Forms-A
    assertTrue(covers(ranges, 0x1E900, 0x1E95F));   // Adlam
    assertFalse(covers(ranges, 0x4E00, 0x9FFF));    // CJK Unified Ideographs
    assertFalse(covers(ranges, 0xAC00, 0xD7AF));    // Hangul Syllables
    assertFalse(covers(ranges, 0x1D400, 0x1D7FF));  // Mathematical Alphanumeric Symbols
  }

  private static boolean covers(
      List<TwitterTextConfiguration.TwitterTextWeightedRange> ranges, int first, int last) {
    for (TwitterTextConfiguration.TwitterTextWeightedRange range : ranges) {
      if (range.getRange().start <= first && range.getRange().end >= last
          && range.getWeight() == 100) {
        return true;
      }
    }
    return false;
  }

  public void testVersion3RetainsUpstreamRanges() {
    final TwitterTextConfiguration configuration =
        TwitterTextConfiguration.configurationFromJson("v3.json", true);
    assertEquals(configuration.getVersion(), 3);
    assertEquals(configuration.getRanges().size(), 4);
  }

  public void testVersion2() {
    final TwitterTextConfiguration configuration =
        TwitterTextConfiguration.configurationFromJson("v2.json", true);
    assertEquals(configuration.getVersion(), 2);
    assertEquals(configuration.getMaxWeightedTweetLength(), 280);
    assertEquals(configuration.getScale(), 100);
    assertEquals(configuration.getDefaultWeight(), 200);
    assertEquals(configuration.getTransformedURLLength(), 23);
    final List<TwitterTextConfiguration.TwitterTextWeightedRange> ranges =
        configuration.getRanges();
    assertNotNull(ranges);
    assertEquals(ranges.size(), 4);
    final TwitterTextConfiguration.TwitterTextWeightedRange latinCharRange = ranges.get(0);
    assertEquals(latinCharRange.getRange(), new Range(0, 4351));
    assertEquals(latinCharRange.getWeight(), 100);
    final TwitterTextConfiguration.TwitterTextWeightedRange spacesGeneralPunctuationBlock =
        ranges.get(1);
    assertEquals(spacesGeneralPunctuationBlock.getRange(), new Range(8192, 8205));
    assertEquals(spacesGeneralPunctuationBlock.getWeight(), 100);
    final TwitterTextConfiguration.TwitterTextWeightedRange visibleGeneralPunctuationBlock1 =
        ranges.get(2);
    assertEquals(visibleGeneralPunctuationBlock1.getRange(), new Range(8208, 8223));
    assertEquals(visibleGeneralPunctuationBlock1.getWeight(), 100);
    final TwitterTextConfiguration.TwitterTextWeightedRange visibleGeneralPunctuationBlock2 =
        ranges.get(3);
    assertEquals(visibleGeneralPunctuationBlock2.getRange(), new Range(8242, 8247));
    assertEquals(visibleGeneralPunctuationBlock2.getWeight(), 100);
  }
}
