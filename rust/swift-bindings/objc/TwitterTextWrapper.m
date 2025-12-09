// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0
//
// TwitterTextWrapper.m
// Objective-C wrapper over the Rust twitter-text C FFI

// Define this BEFORE including our header to prevent compatibility typedefs
#define TWITTER_TEXT_WRAPPER_IMPL 1

#import "TwitterTextWrapper.h"
#import <twitter_text_c.h>

NSString * const kTwitterTextParserConfigurationClassic = @"v1";
NSString * const kTwitterTextParserConfigurationV2 = @"v2";
NSString * const kTwitterTextParserConfigurationV3 = @"v3";

#pragma mark - TTTextEntity

@implementation TTTextEntity

+ (instancetype)entityWithType:(TTTextEntityType)type range:(NSRange)range {
    TTTextEntity *entity = [[TTTextEntity alloc] init];
    entity.type = type;
    entity.range = range;
    return entity;
}

- (NSString *)description {
    NSString *typeName;
    switch (self.type) {
        case TTTextEntityURL: typeName = @"URL"; break;
        case TTTextEntityScreenName: typeName = @"ScreenName"; break;
        case TTTextEntityHashtag: typeName = @"Hashtag"; break;
        case TTTextEntityListName: typeName = @"ListName"; break;
        case TTTextEntitySymbol: typeName = @"Symbol"; break;
        case TTTextEntityTweetChar: typeName = @"TweetChar"; break;
        case TTTextEntityTweetEmojiChar: typeName = @"TweetEmojiChar"; break;
        default: typeName = @"Unknown"; break;
    }
    return [NSString stringWithFormat:@"<%@: %@ %@>",
            NSStringFromClass([self class]),
            typeName,
            NSStringFromRange(self.range)];
}

@end

#pragma mark - TwitterText

@implementation TwitterText

+ (NSArray<TTTextEntity *> *)entitiesFromEntityArray:(TwitterTextEntityArray)array
                                              inText:(NSString *)text {
    NSMutableArray<TTTextEntity *> *results = [NSMutableArray arrayWithCapacity:array.length];

    for (size_t i = 0; i < array.length; i++) {
        TwitterTextEntity ffiEntity = array.entities[i];
        TTTextEntityType type;

        // Map FFI entity types to ObjC entity types
        switch (ffiEntity.entity_type) {
            case 0: type = TTTextEntityURL; break;
            case 1: type = TTTextEntityHashtag; break;
            case 2: type = TTTextEntityScreenName; break;
            case 3: type = TTTextEntitySymbol; break;
            default: type = TTTextEntityURL; break;
        }

        NSRange range = NSMakeRange(ffiEntity.start, ffiEntity.end - ffiEntity.start);
        TTTextEntity *entity = [TTTextEntity entityWithType:type range:range];
        [results addObject:entity];
    }

    twitter_text_entity_array_free(array);
    return results;
}

+ (NSArray<TTTextEntity *> *)entitiesInText:(NSString *)text {
    if (!text) return @[];

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return @[];

    const char *cText = [text UTF8String];

    // Extract all entity types
    NSMutableArray<TTTextEntity *> *allEntities = [NSMutableArray array];

    // URLs
    TwitterTextEntityArray urls = twitter_text_extractor_extract_urls_with_indices(extractor, cText);
    [allEntities addObjectsFromArray:[self entitiesFromEntityArray:urls inText:text]];

    // Mentions
    TwitterTextEntityArray mentions = twitter_text_extractor_extract_mentioned_screennames_with_indices(extractor, cText);
    [allEntities addObjectsFromArray:[self entitiesFromEntityArray:mentions inText:text]];

    // Hashtags
    TwitterTextEntityArray hashtags = twitter_text_extractor_extract_hashtags_with_indices(extractor, cText);
    [allEntities addObjectsFromArray:[self entitiesFromEntityArray:hashtags inText:text]];

    // Cashtags
    TwitterTextEntityArray cashtags = twitter_text_extractor_extract_cashtags_with_indices(extractor, cText);
    [allEntities addObjectsFromArray:[self entitiesFromEntityArray:cashtags inText:text]];

    twitter_text_extractor_free(extractor);

    // Sort by start position
    [allEntities sortUsingComparator:^NSComparisonResult(TTTextEntity *a, TTTextEntity *b) {
        if (a.range.location < b.range.location) return NSOrderedAscending;
        if (a.range.location > b.range.location) return NSOrderedDescending;
        return NSOrderedSame;
    }];

    return allEntities;
}

+ (NSArray<TTTextEntity *> *)URLsInText:(NSString *)text {
    if (!text) return @[];

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return @[];

    const char *cText = [text UTF8String];
    TwitterTextEntityArray urls = twitter_text_extractor_extract_urls_with_indices(extractor, cText);

    NSArray *result = [self entitiesFromEntityArray:urls inText:text];
    twitter_text_extractor_free(extractor);

    return result;
}

+ (NSArray<TTTextEntity *> *)hashtagsInText:(NSString *)text checkingURLOverlap:(BOOL)checkingURLOverlap {
    if (!text) return @[];

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return @[];

    const char *cText = [text UTF8String];
    TwitterTextEntityArray hashtags = twitter_text_extractor_extract_hashtags_with_indices(extractor, cText);

    NSMutableArray *result = [[self entitiesFromEntityArray:hashtags inText:text] mutableCopy];

    // If checking URL overlap, remove hashtags that are inside URLs
    if (checkingURLOverlap) {
        TwitterTextEntityArray urls = twitter_text_extractor_extract_urls_with_indices(extractor, cText);
        NSArray *urlEntities = [self entitiesFromEntityArray:urls inText:text];

        NSMutableArray *filtered = [NSMutableArray array];
        for (TTTextEntity *hashtag in result) {
            BOOL overlaps = NO;
            for (TTTextEntity *url in urlEntities) {
                if (NSIntersectionRange(hashtag.range, url.range).length > 0) {
                    overlaps = YES;
                    break;
                }
            }
            if (!overlaps) {
                [filtered addObject:hashtag];
            }
        }
        result = filtered;
    }

    twitter_text_extractor_free(extractor);

    // Mark as hashtag type
    for (TTTextEntity *entity in result) {
        entity.type = TTTextEntityHashtag;
    }

    return result;
}

+ (NSArray<TTTextEntity *> *)symbolsInText:(NSString *)text checkingURLOverlap:(BOOL)checkingURLOverlap {
    if (!text) return @[];

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return @[];

    const char *cText = [text UTF8String];
    TwitterTextEntityArray cashtags = twitter_text_extractor_extract_cashtags_with_indices(extractor, cText);

    NSMutableArray *result = [[self entitiesFromEntityArray:cashtags inText:text] mutableCopy];

    // If checking URL overlap, remove symbols that are inside URLs
    if (checkingURLOverlap) {
        TwitterTextEntityArray urls = twitter_text_extractor_extract_urls_with_indices(extractor, cText);
        NSArray *urlEntities = [self entitiesFromEntityArray:urls inText:text];

        NSMutableArray *filtered = [NSMutableArray array];
        for (TTTextEntity *symbol in result) {
            BOOL overlaps = NO;
            for (TTTextEntity *url in urlEntities) {
                if (NSIntersectionRange(symbol.range, url.range).length > 0) {
                    overlaps = YES;
                    break;
                }
            }
            if (!overlaps) {
                [filtered addObject:symbol];
            }
        }
        result = filtered;
    }

    twitter_text_extractor_free(extractor);

    // Mark as symbol type
    for (TTTextEntity *entity in result) {
        entity.type = TTTextEntitySymbol;
    }

    return result;
}

+ (NSArray<TTTextEntity *> *)mentionedScreenNamesInText:(NSString *)text {
    if (!text) return @[];

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return @[];

    const char *cText = [text UTF8String];
    TwitterTextEntityArray mentions = twitter_text_extractor_extract_mentioned_screennames_with_indices(extractor, cText);

    NSArray *result = [self entitiesFromEntityArray:mentions inText:text];
    twitter_text_extractor_free(extractor);

    return result;
}

+ (NSArray<TTTextEntity *> *)mentionsOrListsInText:(NSString *)text {
    if (!text) return @[];

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return @[];

    const char *cText = [text UTF8String];
    TwitterTextEntityArray mentions = twitter_text_extractor_extract_mentions_or_lists_with_indices(extractor, cText);

    NSArray *result = [self entitiesFromEntityArray:mentions inText:text];
    twitter_text_extractor_free(extractor);

    return result;
}

+ (nullable TTTextEntity *)repliedScreenNameInText:(NSString *)text {
    if (!text) return nil;

    TwitterTextExtractor *extractor = twitter_text_extractor_new();
    if (!extractor) return nil;

    const char *cText = [text UTF8String];
    TwitterTextEntity *ffiEntity = twitter_text_extractor_extract_reply_username(extractor, cText);

    if (!ffiEntity) {
        twitter_text_extractor_free(extractor);
        return nil;
    }

    NSRange range = NSMakeRange(ffiEntity->start, ffiEntity->end - ffiEntity->start);
    TTTextEntity *entity = [TTTextEntity entityWithType:TTTextEntityScreenName range:range];

    twitter_text_entity_free(ffiEntity);
    twitter_text_extractor_free(extractor);

    return entity;
}

+ (NSInteger)tweetLength:(NSString *)text {
    return [self tweetLength:text transformedURLLength:23];
}

+ (NSInteger)tweetLength:(NSString *)text transformedURLLength:(NSInteger)transformedURLLength {
    if (!text) return 0;

    // Create a configuration with the specified URL length
    TwitterTextConfiguration *config = twitter_text_config_default();
    if (!config) return 0;

    twitter_text_config_set_transformed_url_length(config, (int32_t)transformedURLLength);

    const char *cText = [text UTF8String];
    TwitterTextParseResults result = twitter_text_parse(cText, config, true);

    twitter_text_config_free(config);

    return result.weighted_length;
}

+ (NSInteger)remainingCharacterCount:(NSString *)text {
    return [self remainingCharacterCount:text transformedURLLength:23];
}

+ (NSInteger)remainingCharacterCount:(NSString *)text transformedURLLength:(NSInteger)transformedURLLength {
    NSInteger length = [self tweetLength:text transformedURLLength:transformedURLLength];
    return 140 - length; // Classic tweet length
}

@end

#pragma mark - TTTextWeightedRange

@implementation TTTextWeightedRange

- (instancetype)initWithRange:(NSRange)range weight:(NSInteger)weight {
    self = [super init];
    if (self) {
        _range = range;
        _weight = weight;
    }
    return self;
}

@end

#pragma mark - TTTextConfiguration

@interface TTTextConfiguration ()
@property (nonatomic, assign) TwitterTextConfiguration *internalHandle;
@property (nonatomic, strong) NSArray<TTTextWeightedRange *> *cachedRanges;
@end

@implementation TTTextConfiguration

+ (instancetype)configurationFromJSONResource:(NSString *)jsonResource {
    TTTextConfiguration *config = [[TTTextConfiguration alloc] init];

    if ([jsonResource isEqualToString:kTwitterTextParserConfigurationV3]) {
        config.internalHandle = twitter_text_config_v3();
    } else if ([jsonResource isEqualToString:kTwitterTextParserConfigurationV2]) {
        config.internalHandle = twitter_text_config_v2();
    } else {
        // V1 (classic) uses v1 config
        config.internalHandle = twitter_text_config_v1();
    }

    return config;
}

+ (instancetype)configurationFromJSONString:(NSString *)jsonString {
    TTTextConfiguration *config = [[TTTextConfiguration alloc] init];
    const char *cJson = [jsonString UTF8String];
    config.internalHandle = twitter_text_config_from_json(cJson);
    return config;
}

- (void)dealloc {
    if (_internalHandle) {
        twitter_text_config_free(_internalHandle);
    }
}

- (void *)handle {
    return _internalHandle;
}

- (NSInteger)version {
    if (!_internalHandle) return 0;
    return twitter_text_config_get_version(_internalHandle);
}

- (NSInteger)maxWeightedTweetLength {
    if (!_internalHandle) return 0;
    return twitter_text_config_get_max_weighted_tweet_length(_internalHandle);
}

- (NSInteger)scale {
    if (!_internalHandle) return 0;
    return twitter_text_config_get_scale(_internalHandle);
}

- (NSInteger)defaultWeight {
    if (!_internalHandle) return 0;
    return twitter_text_config_get_default_weight(_internalHandle);
}

- (NSInteger)transformedURLLength {
    if (!_internalHandle) return 0;
    return twitter_text_config_get_transformed_url_length(_internalHandle);
}

- (BOOL)isEmojiParsingEnabled {
    if (!_internalHandle) return NO;
    return twitter_text_config_get_emoji_parsing_enabled(_internalHandle);
}

- (NSArray<TTTextWeightedRange *> *)ranges {
    if (_cachedRanges) return _cachedRanges;
    if (!_internalHandle) return @[];

    TwitterTextWeightedRangeArray rangeArray = twitter_text_config_get_ranges(_internalHandle);
    NSMutableArray *result = [NSMutableArray arrayWithCapacity:rangeArray.length];

    for (size_t i = 0; i < rangeArray.length; i++) {
        TwitterTextWeightedRange ffiRange = rangeArray.ranges[i];
        NSRange nsRange = NSMakeRange(ffiRange.range.start, ffiRange.range.end - ffiRange.range.start + 1);
        TTTextWeightedRange *weightedRange = [[TTTextWeightedRange alloc] initWithRange:nsRange weight:ffiRange.weight];
        [result addObject:weightedRange];
    }

    twitter_text_weighted_range_array_free(rangeArray);
    _cachedRanges = result;
    return result;
}

@end

#pragma mark - TTTextParseResults

@implementation TTTextParseResults

- (instancetype)initWithWeightedLength:(NSInteger)length
                            permillage:(NSInteger)permillage
                                 valid:(BOOL)valid
                          displayRange:(NSRange)displayRange
                            validRange:(NSRange)validRange {
    self = [super init];
    if (self) {
        _weightedLength = length;
        _permillage = permillage;
        _isValid = valid;
        _displayTextRange = displayRange;
        _validDisplayTextRange = validRange;
    }
    return self;
}

@end

#pragma mark - TTTextParser

static TTTextParser *_defaultParser = nil;

@interface TTTextParser ()
@property (nonatomic, strong) TTTextConfiguration *configuration;
@end

@implementation TTTextParser

+ (instancetype)defaultParser {
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        _defaultParser = [[TTTextParser alloc] initWithConfiguration:
            [TTTextConfiguration configurationFromJSONResource:kTwitterTextParserConfigurationV3]];
    });
    return _defaultParser;
}

+ (void)setDefaultParserWithConfiguration:(TTTextConfiguration *)configuration {
    _defaultParser = [[TTTextParser alloc] initWithConfiguration:configuration];
}

- (instancetype)initWithConfiguration:(TTTextConfiguration *)configuration {
    self = [super init];
    if (self) {
        _configuration = configuration;
    }
    return self;
}

- (TTTextParseResults *)parseTweet:(NSString *)text {
    if (!text) {
        return [[TTTextParseResults alloc] initWithWeightedLength:0
                                                       permillage:0
                                                            valid:YES
                                                     displayRange:NSMakeRange(0, 0)
                                                       validRange:NSMakeRange(0, 0)];
    }

    const char *cText = [text UTF8String];
    TwitterTextParseResults result = twitter_text_parse(cText, (TwitterTextConfiguration *)_configuration.handle, true);

    NSRange displayRange = NSMakeRange(result.display_text_range.start,
                                        result.display_text_range.end - result.display_text_range.start + 1);
    NSRange validRange = NSMakeRange(result.valid_text_range.start,
                                      result.valid_text_range.end - result.valid_text_range.start + 1);

    return [[TTTextParseResults alloc] initWithWeightedLength:result.weighted_length
                                                   permillage:result.permillage
                                                        valid:result.is_valid
                                                 displayRange:displayRange
                                                   validRange:validRange];
}

- (NSInteger)maxWeightedTweetLength {
    return _configuration.maxWeightedTweetLength;
}

@end

#pragma mark - TTTextAutolinker

@interface TTTextAutolinker ()
@property (nonatomic, assign) TwitterTextAutolinker *internalHandle;
@end

@implementation TTTextAutolinker

- (instancetype)init {
    return [self initWithNoFollow:YES];
}

- (instancetype)initWithNoFollow:(BOOL)noFollow {
    self = [super init];
    if (self) {
        _internalHandle = twitter_text_autolinker_new(noFollow);
    }
    return self;
}

- (void)dealloc {
    if (_internalHandle) {
        twitter_text_autolinker_free(_internalHandle);
    }
}

- (BOOL)noFollow {
    // We don't have a getter in the C API, so we'd need to track this ourselves
    // For now, return YES as default
    return YES;
}

- (void)setNoFollow:(BOOL)noFollow {
    if (_internalHandle) {
        twitter_text_autolinker_set_no_follow(_internalHandle, noFollow);
    }
}

- (NSString *)autolink:(NSString *)text {
    if (!text || !_internalHandle) return text;

    const char *cText = [text UTF8String];
    char *result = twitter_text_autolinker_autolink(_internalHandle, cText);

    if (!result) return text;

    NSString *nsResult = [NSString stringWithUTF8String:result];
    twitter_text_string_free(result);
    return nsResult;
}

- (NSString *)autolinkURLs:(NSString *)text {
    if (!text || !_internalHandle) return text;

    const char *cText = [text UTF8String];
    char *result = twitter_text_autolinker_autolink_urls(_internalHandle, cText);

    if (!result) return text;

    NSString *nsResult = [NSString stringWithUTF8String:result];
    twitter_text_string_free(result);
    return nsResult;
}

- (NSString *)autolinkHashtags:(NSString *)text {
    if (!text || !_internalHandle) return text;

    const char *cText = [text UTF8String];
    char *result = twitter_text_autolinker_autolink_hashtags(_internalHandle, cText);

    if (!result) return text;

    NSString *nsResult = [NSString stringWithUTF8String:result];
    twitter_text_string_free(result);
    return nsResult;
}

- (NSString *)autolinkMentionsAndLists:(NSString *)text {
    if (!text || !_internalHandle) return text;

    const char *cText = [text UTF8String];
    char *result = twitter_text_autolinker_autolink_usernames_and_lists(_internalHandle, cText);

    if (!result) return text;

    NSString *nsResult = [NSString stringWithUTF8String:result];
    twitter_text_string_free(result);
    return nsResult;
}

- (NSString *)autolinkCashtags:(NSString *)text {
    if (!text || !_internalHandle) return text;

    const char *cText = [text UTF8String];
    char *result = twitter_text_autolinker_autolink_cashtags(_internalHandle, cText);

    if (!result) return text;

    NSString *nsResult = [NSString stringWithUTF8String:result];
    twitter_text_string_free(result);
    return nsResult;
}

- (void)setURLClass:(NSString *)urlClass {
    if (_internalHandle && urlClass) {
        twitter_text_autolinker_set_url_class(_internalHandle, [urlClass UTF8String]);
    }
}

- (void)setHashtagClass:(NSString *)hashtagClass {
    if (_internalHandle && hashtagClass) {
        twitter_text_autolinker_set_hashtag_class(_internalHandle, [hashtagClass UTF8String]);
    }
}

- (void)setMentionClass:(NSString *)mentionClass {
    if (_internalHandle && mentionClass) {
        twitter_text_autolinker_set_username_class(_internalHandle, [mentionClass UTF8String]);
    }
}

- (void)setCashtagClass:(NSString *)cashtagClass {
    if (_internalHandle && cashtagClass) {
        twitter_text_autolinker_set_cashtag_class(_internalHandle, [cashtagClass UTF8String]);
    }
}

@end
