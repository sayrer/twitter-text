// Copyright 2018 Twitter, Inc.
// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0
//
// TwitterTextEmojiTests.m
// Emoji tests for the Rust twitter-text library
//
// NOTE: The original objc emoji tests test a standalone emoji regex.
// The Rust implementation has emoji parsing built into the parser/configuration,
// so standalone emoji regex is not exposed. These tests verify emoji handling
// through tweet length calculation with emoji parsing enabled (v3 config).

#import "TwitterTextEmojiTests.h"
#import "rust/swift-bindings/objc/TwitterTextWrapper.h"

@implementation TwitterTextEmojiTests

- (void)testEmojiParsingEnabledInV3Config
{
    // V3 configuration should have emoji parsing enabled
    TwitterTextConfiguration *configV3 = [TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3];
    XCTAssertTrue(configV3.isEmojiParsingEnabled, @"V3 config should have emoji parsing enabled");

    // V2 configuration should NOT have emoji parsing enabled
    TwitterTextConfiguration *configV2 = [TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV2];
    XCTAssertFalse(configV2.isEmojiParsingEnabled, @"V2 config should NOT have emoji parsing enabled");
}

- (void)testEmojiCountingV2VsV3
{
    // This test verifies the key difference between v2 and v3 emoji counting
    // Text: "H🐱☺👨‍👩‍👧‍👦" (H + cat + smiley + family ZWJ sequence)
    // v2: weightedLength = 16 (counts by codepoints, no emoji discounting)
    // v3: weightedLength = 7 (counts emojis as single grapheme clusters with weight 2)

    NSString *text = @"H🐱☺👨‍👩‍👧‍👦";

    // Test with v2 config
    [TwitterTextParser setDefaultParserWithConfiguration:[TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV2]];
    TwitterTextParseResults *v2Results = [[TwitterTextParser defaultParser] parseTweet:text];
    XCTAssertEqual(v2Results.weightedLength, 16, @"V2 config should count by codepoints (weightedLength=16)");
    XCTAssertEqual(v2Results.permillage, 57, @"V2 config permillage should be 57");
    XCTAssertTrue(v2Results.isValid, @"Tweet should be valid");

    // Test with v3 config (emoji discounting)
    [TwitterTextParser setDefaultParserWithConfiguration:[TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3]];
    TwitterTextParseResults *v3Results = [[TwitterTextParser defaultParser] parseTweet:text];
    XCTAssertEqual(v3Results.weightedLength, 7, @"V3 config should discount emojis (weightedLength=7)");
    XCTAssertEqual(v3Results.permillage, 25, @"V3 config permillage should be 25");
    XCTAssertTrue(v3Results.isValid, @"Tweet should be valid");
}

- (void)testEmojiWeightInV3Config
{
    // With v3 config, emojis are parsed but still have default weight
    // (The Rust implementation uses default weight 2 for emojis)
    [TwitterTextParser setDefaultParserWithConfiguration:[TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3]];

    // A single emoji should be valid
    NSString *singleEmoji = @"🤦";
    TwitterTextParseResults *results = [[TwitterTextParser defaultParser] parseTweet:singleEmoji];
    XCTAssertGreaterThan(results.weightedLength, 0, @"Single emoji should have positive weighted length");
    XCTAssertTrue(results.isValid, @"Single emoji tweet should be valid");
}

- (void)testComplexEmojiSequences
{
    [TwitterTextParser setDefaultParserWithConfiguration:[TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3]];

    // Flag emoji (England) - a complex emoji sequence
    NSString *flagEmoji = @"🏴󠁧󠁢󠁥󠁮󠁧󠁿";
    TwitterTextParseResults *flagResults = [[TwitterTextParser defaultParser] parseTweet:flagEmoji];
    XCTAssertTrue(flagResults.isValid, @"Flag emoji tweet should be valid");

    // Family emoji - ZWJ sequence
    NSString *familyEmoji = @"👨‍👨‍👧‍👧";
    TwitterTextParseResults *familyResults = [[TwitterTextParser defaultParser] parseTweet:familyEmoji];
    XCTAssertTrue(familyResults.isValid, @"Family emoji tweet should be valid");

    // Keycap emoji
    NSString *keycapEmoji = @"0️⃣";
    TwitterTextParseResults *keycapResults = [[TwitterTextParser defaultParser] parseTweet:keycapEmoji];
    XCTAssertTrue(keycapResults.isValid, @"Keycap emoji tweet should be valid");
}

- (void)testMixedTextAndEmoji
{
    [TwitterTextParser setDefaultParserWithConfiguration:[TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3]];

    // Mix of text and emojis
    NSString *mixed = @"Hello 👋 World 🌍";
    TwitterTextParseResults *results = [[TwitterTextParser defaultParser] parseTweet:mixed];
    XCTAssertTrue(results.isValid, @"Mixed text and emoji tweet should be valid");
    XCTAssertGreaterThan(results.weightedLength, 0, @"Mixed content should have positive length");
}

- (void)testEmojiWithSkinToneModifier
{
    [TwitterTextParser setDefaultParserWithConfiguration:[TwitterTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3]];

    // Emoji with skin tone modifier
    NSString *emojiWithSkinTone = @"👋🏽";
    TwitterTextParseResults *results = [[TwitterTextParser defaultParser] parseTweet:emojiWithSkinTone];
    XCTAssertTrue(results.isValid, @"Emoji with skin tone should be valid");
}

@end
