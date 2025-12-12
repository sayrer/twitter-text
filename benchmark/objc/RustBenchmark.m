// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0
//
// RustBenchmark.m
// Benchmark for Rust FFI twitter-text implementation via Obj-C wrapper

#import <Foundation/Foundation.h>
#import "rust/swift-bindings/objc/TwitterTextWrapper.h"
#import "benchmark/objc/YAMLHelper-Swift.h"

static const NSInteger ITERATIONS = 1000;
static const NSInteger WARMUP_ITERATIONS = 100;

static double benchmarkAutolink(NSDictionary *data) {
    NSArray *tests = data[@"tests"];
    TTTextAutolinker *autolinker = [[TTTextAutolinker alloc] initWithNoFollow:NO];

    // Warmup
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [autolinker autolink:test[@"text"]];
        }
    }

    // Benchmark
    CFAbsoluteTime start = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [autolinker autolink:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsed = CFAbsoluteTimeGetCurrent() - start;
    return ITERATIONS / elapsed;
}

static double benchmarkExtract(NSDictionary *data) {
    NSDictionary *tests = data[@"tests"];
    NSArray *mentions = tests[@"mentions"];
    NSArray *urls = tests[@"urls"];
    NSArray *hashtags = tests[@"hashtags"];
    NSArray *cashtags = tests[@"cashtags"];

    // Collect all texts (like JS/Rust benchmark)
    NSMutableArray *allTexts = [NSMutableArray array];
    for (NSDictionary *test in mentions) {
        [allTexts addObject:test[@"text"]];
    }
    for (NSDictionary *test in urls) {
        [allTexts addObject:test[@"text"]];
    }
    for (NSDictionary *test in hashtags) {
        [allTexts addObject:test[@"text"]];
    }
    for (NSDictionary *test in cashtags) {
        [allTexts addObject:test[@"text"]];
    }

    // Warmup - call all 4 extract functions for each text
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSString *text in allTexts) {
            [TwitterText mentionedScreenNamesInText:text];
            [TwitterText URLsInText:text];
            [TwitterText hashtagsInText:text checkingURLOverlap:YES];
            [TwitterText symbolsInText:text checkingURLOverlap:YES];
        }
    }

    // Benchmark
    CFAbsoluteTime start = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSString *text in allTexts) {
            [TwitterText mentionedScreenNamesInText:text];
            [TwitterText URLsInText:text];
            [TwitterText hashtagsInText:text checkingURLOverlap:YES];
            [TwitterText symbolsInText:text checkingURLOverlap:YES];
        }
    }
    CFAbsoluteTime elapsed = CFAbsoluteTimeGetCurrent() - start;
    return ITERATIONS / elapsed;
}

static double benchmarkValidateTweet(NSDictionary *data) {
    NSDictionary *tests = data[@"tests"];
    NSArray *tweets = tests[@"tweets"];

    // Warmup
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tweets) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }

    // Benchmark
    CFAbsoluteTime start = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tweets) {
            [TwitterText tweetLength:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsed = CFAbsoluteTimeGetCurrent() - start;
    return ITERATIONS / elapsed;
}

// Note: Obj-C wrapper doesn't have isValidUsername/Hashtag/Url methods,
// so Validate All benchmark is not available for this implementation.

static double benchmarkParse(NSDictionary *data) {
    NSArray *tests = data[@"tests"];
    TTTextParser *parser = [TTTextParser defaultParser];

    // Warmup
    for (NSInteger i = 0; i < WARMUP_ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [parser parseTweet:test[@"text"]];
        }
    }

    // Benchmark
    CFAbsoluteTime start = CFAbsoluteTimeGetCurrent();
    for (NSInteger i = 0; i < ITERATIONS; i++) {
        for (NSDictionary *test in tests) {
            [parser parseTweet:test[@"text"]];
        }
    }
    CFAbsoluteTime elapsed = CFAbsoluteTimeGetCurrent() - start;
    return ITERATIONS / elapsed;
}

int main(int argc, const char * argv[]) {
    @autoreleasepool {
        if (argc < 5) {
            fprintf(stderr, "Usage: %s <autolink.yml> <extract.yml> <validate.yml> <parse.yml>\n", argv[0]);
            return 1;
        }

        NSDictionary *autolinkData = [YAMLHelper loadYAMLFromPath:[NSString stringWithUTF8String:argv[1]]];
        NSDictionary *extractData = [YAMLHelper loadYAMLFromPath:[NSString stringWithUTF8String:argv[2]]];
        NSDictionary *validateData = [YAMLHelper loadYAMLFromPath:[NSString stringWithUTF8String:argv[3]]];
        NSDictionary *parseData = [YAMLHelper loadYAMLFromPath:[NSString stringWithUTF8String:argv[4]]];

        if (!autolinkData || !extractData || !validateData || !parseData) {
            fprintf(stderr, "Failed to load benchmark data\n");
            return 1;
        }

        double autolinkOps = benchmarkAutolink(autolinkData);
        double extractOps = benchmarkExtract(extractData);
        double validateTweetOps = benchmarkValidateTweet(validateData);
        double parseOps = benchmarkParse(parseData);

        // Output JSON for easy parsing
        printf("{\"autolink\": %.2f, \"extract\": %.2f, \"validate_tweet\": %.2f, \"parse\": %.2f}\n",
               autolinkOps, extractOps, validateTweetOps, parseOps);
    }
    return 0;
}
