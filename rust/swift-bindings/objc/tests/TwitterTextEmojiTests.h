// Copyright 2018 Twitter, Inc.
// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0
//
// TwitterTextEmojiTests.h
// Emoji tests for the Rust twitter-text library
//
// NOTE: The original objc emoji tests test a standalone emoji regex.
// The Rust implementation has emoji parsing built into the parser/configuration,
// so standalone emoji regex is not exposed. These tests verify emoji handling
// through the parser configuration's emoji_parsing_enabled setting.

#import <XCTest/XCTest.h>

@interface TwitterTextEmojiTests : XCTestCase
@end
