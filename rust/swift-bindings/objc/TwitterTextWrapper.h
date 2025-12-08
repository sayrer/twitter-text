// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0
//
// TwitterTextWrapper.h
// Objective-C wrapper over the Rust twitter-text C FFI
// API-compatible with the original objc/lib/TwitterText.h
//
// Note: ObjC types use "TT" prefix to avoid conflicts with C FFI types.

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

typedef NS_ENUM(NSUInteger, TTTextEntityType) {
    TTTextEntityURL,
    TTTextEntityScreenName,
    TTTextEntityHashtag,
    TTTextEntityListName,
    TTTextEntitySymbol,
    TTTextEntityTweetChar,
    TTTextEntityTweetEmojiChar
};

@interface TTTextEntity : NSObject

@property (nonatomic) TTTextEntityType type;
@property (nonatomic) NSRange range;

+ (instancetype)entityWithType:(TTTextEntityType)type range:(NSRange)range;

@end

@interface TwitterText : NSObject

+ (NSArray<TTTextEntity *> *)entitiesInText:(NSString *)text;
+ (NSArray<TTTextEntity *> *)URLsInText:(NSString *)text;
+ (NSArray<TTTextEntity *> *)hashtagsInText:(NSString *)text checkingURLOverlap:(BOOL)checkingURLOverlap;
+ (NSArray<TTTextEntity *> *)symbolsInText:(NSString *)text checkingURLOverlap:(BOOL)checkingURLOverlap;
+ (NSArray<TTTextEntity *> *)mentionedScreenNamesInText:(NSString *)text;
+ (NSArray<TTTextEntity *> *)mentionsOrListsInText:(NSString *)text;
+ (nullable TTTextEntity *)repliedScreenNameInText:(NSString *)text;

+ (NSInteger)tweetLength:(NSString *)text;
+ (NSInteger)tweetLength:(NSString *)text transformedURLLength:(NSInteger)transformedURLLength;

+ (NSInteger)remainingCharacterCount:(NSString *)text;
+ (NSInteger)remainingCharacterCount:(NSString *)text transformedURLLength:(NSInteger)transformedURLLength;

@end

FOUNDATION_EXTERN NSString * const kTwitterTextParserConfigurationClassic;
FOUNDATION_EXTERN NSString * const kTwitterTextParserConfigurationV2;
FOUNDATION_EXTERN NSString * const kTwitterTextParserConfigurationV3;

@interface TTTextWeightedRange : NSObject

@property (nonatomic, readonly) NSRange range;
@property (nonatomic, readonly) NSInteger weight;

- (instancetype)initWithRange:(NSRange)range weight:(NSInteger)weight;

@end

@interface TTTextConfiguration : NSObject

+ (instancetype)configurationFromJSONResource:(NSString *)jsonResource;
+ (instancetype)configurationFromJSONString:(NSString *)jsonString;

@property (nonatomic, readonly) NSInteger version;
@property (nonatomic, readonly) NSInteger maxWeightedTweetLength;
@property (nonatomic, readonly) NSInteger scale;
@property (nonatomic, readonly) NSInteger defaultWeight;
@property (nonatomic, readonly) NSInteger transformedURLLength;
@property (nonatomic, readonly, getter=isEmojiParsingEnabled) BOOL emojiParsingEnabled;
@property (nonatomic, readonly) NSArray<TTTextWeightedRange *> *ranges;

// Internal: get the opaque handle for use with the C API
@property (nonatomic, readonly) void *handle;

@end

@interface TTTextParseResults : NSObject

- (instancetype)initWithWeightedLength:(NSInteger)length
                            permillage:(NSInteger)permillage
                                 valid:(BOOL)valid
                          displayRange:(NSRange)displayRange
                            validRange:(NSRange)validRange;

@property (nonatomic, readonly) NSInteger weightedLength;
@property (nonatomic, readonly) NSInteger permillage;
@property (nonatomic, readonly) BOOL isValid;
@property (nonatomic, readonly) NSRange displayTextRange;
@property (nonatomic, readonly) NSRange validDisplayTextRange;

@end

@interface TTTextParser : NSObject

@property (nonatomic, readonly) TTTextConfiguration *configuration;

+ (instancetype)defaultParser;
+ (void)setDefaultParserWithConfiguration:(TTTextConfiguration *)configuration;

- (instancetype)initWithConfiguration:(TTTextConfiguration *)configuration;
- (TTTextParseResults *)parseTweet:(NSString *)text;
- (NSInteger)maxWeightedTweetLength;

@end

// Compatibility typedefs - these alias our ObjC classes to the original API names
// ONLY for use in test files that want to use the original API names
#ifndef TWITTER_TEXT_WRAPPER_IMPL
typedef TTTextEntityType TwitterTextEntityType;
#define TwitterTextEntityURL TTTextEntityURL
#define TwitterTextEntityScreenName TTTextEntityScreenName
#define TwitterTextEntityHashtag TTTextEntityHashtag
#define TwitterTextEntityListName TTTextEntityListName
#define TwitterTextEntitySymbol TTTextEntitySymbol
#define TwitterTextEntityTweetChar TTTextEntityTweetChar
#define TwitterTextEntityTweetEmojiChar TTTextEntityTweetEmojiChar
typedef TTTextEntity TwitterTextEntity;
typedef TTTextWeightedRange TwitterTextWeightedRange;
typedef TTTextConfiguration TwitterTextConfiguration;
typedef TTTextParseResults TwitterTextParseResults;
typedef TTTextParser TwitterTextParser;
#endif

NS_ASSUME_NONNULL_END
