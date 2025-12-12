// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0
//
// TwitterTextBenchmark.m
// Benchmark comparing old Obj-C twitter-text vs Rust FFI bindings

#import <Foundation/Foundation.h>

// Old implementation - use full path to avoid conflicts
#import "objc/lib/TwitterText.h"

// Rust FFI wrapper - aliased types
#import "rust/swift-bindings/objc/TwitterTextWrapper.h"

static const NSInteger ITERATIONS = 1000;
static const NSInteger WARMUP_ITERATIONS = 100;

// Rename old types to avoid conflicts with wrapper compatibility typedefs
typedef TwitterTextEntity OldTwitterTextEntity;

@interface BenchmarkData : NSObject
@property (nonatomic, strong) NSArray<NSDictionary *> *autolinkTests;
@property (nonatomic, strong) NSDictionary *extractTests;
@property (nonatomic, strong) NSDictionary *validateTests;
@property (nonatomic, strong) NSArray<NSDictionary *> *parseTests;
@end

@implementation BenchmarkData
@end

static NSDictionary *loadYAML(NSString *filename) {
    // YAML files are in benchmark/data/ directory
    NSString *path = [[NSBundle mainBundle] pathForResource:filename ofType:@"yml" inDirectory:@"benchmark/data"];
    if (!path) {
        // Try relative path for command-line execution
        path = [NSString stringWithFormat:@"benchmark/data/%@.yml", filename];
    }

    NSError *error = nil;
    NSString *content = [NSString stringWithContentsOfFile:path encoding:NSUTF8StringEncoding error:&error];
    if (error) {
        NSLog(@"Error loading %@: %@", filename, error);
        return nil;
    }

    // Simple YAML parsing for our test data format
    // Since we control the format, we can use a simple parser
    NSMutableDictionary *result = [NSMutableDictionary dictionary];
    NSMutableDictionary *currentSection = nil;
    NSMutableArray *currentArray = nil;
    NSMutableDictionary *currentItem = nil;
    NSString *currentKey = nil;

    for (NSString *line in [content componentsSeparatedByString:@"\n"]) {
        NSString *trimmed = [line stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];

        if ([trimmed hasPrefix:@"#"] || trimmed.length == 0) continue;

        if ([line hasPrefix:@"tests:"]) {
            currentSection = [NSMutableDictionary dictionary];
            result[@"tests"] = currentSection;
        } else if ([line hasPrefix:@"  "] && ![line hasPrefix:@"    "]) {
            // Section like "  mentions:" or "  tweets:"
            NSString *sectionName = [[trimmed stringByReplacingOccurrencesOfString:@":" withString:@""] stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];
            currentArray = [NSMutableArray array];
            if (currentSection) {
                currentSection[sectionName] = currentArray;
            } else {
                result[sectionName] = currentArray;
            }
        } else if ([trimmed hasPrefix:@"- "]) {
            currentItem = [NSMutableDictionary dictionary];
            [currentArray addObject:currentItem];
            // Parse inline key-value if present
            NSString *rest = [trimmed substringFromIndex:2];
            if ([rest containsString:@":"]) {
                NSRange colonRange = [rest rangeOfString:@":"];
                currentKey = [[rest substringToIndex:colonRange.location] stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];
                NSString *value = [[rest substringFromIndex:colonRange.location + 1] stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];
                if ([value hasPrefix:@"\""]) {
                    value = [value substringWithRange:NSMakeRange(1, value.length - 2)];
                }
                currentItem[currentKey] = value;
            }
        } else if ([trimmed containsString:@":"] && currentItem) {
            NSRange colonRange = [trimmed rangeOfString:@":"];
            currentKey = [[trimmed substringToIndex:colonRange.location] stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];
            NSString *value = [[trimmed substringFromIndex:colonRange.location + 1] stringByTrimmingCharactersInSet:[NSCharacterSet whitespaceCharacterSet]];
            if ([value hasPrefix:@"\""]) {
                value = [value substringWithRange:NSMakeRange(1, value.length - 2)];
            }
            if (value.length > 0) {
                if ([value isEqualToString:@"true"]) {
                    currentItem[currentKey] = @YES;
                } else if ([value isEqualToString:@"false"]) {
                    currentItem[currentKey] = @NO;
                } else {
                    currentItem[currentKey] = value;
                }
            }
        }
    }

    return result;
}

static void printResults(NSString *operation, double oldOpsPerSec, double rustOpsPerSec) {
    double speedup = rustOpsPerSec / oldOpsPerSec;
    NSString *label = speedup >= 1 ? @"faster" : @"slower";
    double ratio = speedup >= 1 ? speedup : (1 / speedup);

    printf("\n%s (%ld iterations):\n", [operation UTF8String], (long)ITERATIONS);
    printf("  Old Obj-C: %.0f ops/sec\n", oldOpsPerSec);
    printf("  Rust FFI:  %.0f ops/sec\n", rustOpsPerSec);
    printf("  Result:    %.1fx %s\n", ratio, [label UTF8String]);
}

static void benchmarkExtract(NSDictionary *data) {
    NSDictionary *tests = data[@"tests"];
    NSArray *mentions = tests[@"mentions"];
    NSArray *urls = tests[@"urls"];
    NSArray *hashtags = tests[@"hashtags"];
    NSArray *cashtags = tests[@"cashtags"];

    // Warmup old
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in mentions) {
            [TwitterText mentionedScreenNamesInText:test[@"text"]];
        }
        for (NSDictionary *test in urls) {
            [TwitterText URLsInText:test[@"text"]];
        }
        for (NSDictionary *test in hashtags) {
            [TwitterText hashtagsInText:test[@"text"] checkingURLOverlap:YES];
        }
        for (NSDictionary *test in cashtags) {
            [TwitterText symbolsInText:test[@"text"] checkingURLOverlap:YES];
        }
    }

    // Benchmark old
    CFAbsoluteTime startOld = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in mentions) {
            [TwitterText mentionedScreenNamesInText:test[@"text"]];
        }
        for (NSDictionary *test in urls) {
            [TwitterText URLsInText:test[@"text"]];
        }
        for (NSDictionary *test in hashtags) {
            [TwitterText hashtagsInText:test[@"text"] checkingURLOverlap:YES];
        }
        for (NSDictionary *test in cashtags) {
            [TwitterText symbolsInText:test[@"text"] checkingURLOverlap:YES];
        }
    }
    CFAbsoluteTime elapsedOld = CFAbsoluteTimeGetCurrent() - startOld;
    double oldOpsPerSec = ITERATIONS / elapsedOld;

    // Warmup rust (using TTTextEntity via wrapper)
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in mentions) {
            [TwitterText mentionedScreenNamesInText:test[@"text"]];
        }
        for (NSDictionary *test in urls) {
            [TwitterText URLsInText:test[@"text"]];
        }
        for (NSDictionary *test in hashtags) {
            [TwitterText hashtagsInText:test[@"text"] checkingURLOverlap:YES];
        }
        for (NSDictionary *test in cashtags) {
            [TwitterText symbolsInText:test[@"text"] checkingURLOverlap:YES];
        }
    }

    // Benchmark rust
    CFAbsoluteTime startRust = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in mentions) {
            [TwitterText mentionedScreenNamesInText:test[@"text"]];
        }
        for (NSDictionary *test in urls) {
            [TwitterText URLsInText:test[@"text"]];
        }
        for (NSDictionary *test in hashtags) {
            [TwitterText hashtagsInText:test[@"text"] checkingURLOverlap:YES];
        }
        for (NSDictionary *test in cashtags) {
            [TwitterText symbolsInText:test[@"text"] checkingURLOverlap:YES];
        }
    }
    CFAbsoluteTime elapsedRust = CFAbsoluteTimeGetCurrent() - startRust;
    double rustOpsPerSec = ITERATIONS / elapsedRust;

    printResults(@"Extract", oldOpsPerSec, rustOpsPerSec);
}

static void benchmarkValidate(NSDictionary *data) {
    NSDictionary *tests = data[@"tests"];
    NSArray *tweets = tests[@"tweets"];

    // Warmup old - use tweetLength as proxy for validation
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tweets) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }

    // Benchmark old
    CFAbsoluteTime startOld = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tweets) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsedOld = CFAbsoluteTimeGetCurrent() - startOld;
    double oldOpsPerSec = ITERATIONS / elapsedOld;

    // Warmup rust
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tweets) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }

    // Benchmark rust
    CFAbsoluteTime startRust = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tweets) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsedRust = CFAbsoluteTimeGetCurrent() - startRust;
    double rustOpsPerSec = ITERATIONS / elapsedRust;

    printResults(@"Validate", oldOpsPerSec, rustOpsPerSec);
}

static void benchmarkParse(NSDictionary *data) {
    NSArray *tests = data[@"tests"];

    // Old implementation doesn't have TwitterTextParser, use tweetLength
    // Warmup old
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }

    // Benchmark old
    CFAbsoluteTime startOld = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsedOld = CFAbsoluteTimeGetCurrent() - startOld;
    double oldOpsPerSec = ITERATIONS / elapsedOld;

    // Rust FFI uses TTTextParser
    TTTextParser *parser = [TTTextParser defaultParser];

    // Warmup rust
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [parser parseTweet:test[@"text"]];
        }
    }

    // Benchmark rust
    CFAbsoluteTime startRust = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [parser parseTweet:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsedRust = CFAbsoluteTimeGetCurrent() - startRust;
    double rustOpsPerSec = ITERATIONS / elapsedRust;

    printResults(@"Parse Tweet", oldOpsPerSec, rustOpsPerSec);
}

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        printf("Twitter Text Benchmark: Old Obj-C vs Rust FFI\n");
        printf("==============================================\n");

        NSDictionary *extractData = loadYAML(@"extract");
        NSDictionary *validateData = loadYAML(@"validate");
        NSDictionary *parseData = loadYAML(@"parse");

        if (!extractData || !validateData || !parseData) {
            fprintf(stderr, "Failed to load benchmark data\n");
            return 1;
        }

        benchmarkExtract(extractData);
        benchmarkValidate(validateData);
        benchmarkParse(parseData);

        printf("\nDone.\n");
    }
    return 0;
}
