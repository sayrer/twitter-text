// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Gungraun benchmarks for twitter-text
//!
//! Uses Valgrind-based instrumentation for precise, reproducible measurements.
//! Run with: bazel run //benchmark/rust:gungraun_benchmark
//!
//! Requires valgrind and gungraun-runner to be installed.

use gungraun::{library_benchmark, library_benchmark_group, main};
use std::hint::black_box;
use twitter_text::autolinker::Autolinker;
use twitter_text::extractor::Extractor;
use twitter_text::validator::Validator;
use twitter_text::ParserBackend;
use twitter_text_config::Configuration;

// Sample texts for benchmarking - representative of real tweet content
const SIMPLE_TEXT: &str = "Hello world!";
const TEXT_WITH_MENTION: &str = "Hello @jack how are you?";
const TEXT_WITH_HASHTAG: &str = "Check out #rustlang for great programming content";
const TEXT_WITH_URL: &str = "Check out https://github.com/rust-lang/rust for more info";
const TEXT_WITH_CASHTAG: &str = "Stock $AAPL is looking good today";
const COMPLEX_TEXT: &str = "Hey @elonmusk check out #AI and #ML trends at https://example.com/ai - $TSLA looking strong! @OpenAI is doing great work. Visit https://t.co/abc123 for more #tech news.";

// --- Extractor benchmarks ---

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_extract_urls_nom(text: &str) {
    let extractor = Extractor::with_parser_backend(ParserBackend::Nom);
    black_box(extractor.extract_urls(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_extract_urls_pest(text: &str) {
    let extractor = Extractor::with_parser_backend(ParserBackend::Pest);
    black_box(extractor.extract_urls(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_extract_mentions_nom(text: &str) {
    let extractor = Extractor::with_parser_backend(ParserBackend::Nom);
    black_box(extractor.extract_mentioned_screennames(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_extract_mentions_pest(text: &str) {
    let extractor = Extractor::with_parser_backend(ParserBackend::Pest);
    black_box(extractor.extract_mentioned_screennames(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_extract_hashtags_nom(text: &str) {
    let extractor = Extractor::with_parser_backend(ParserBackend::Nom);
    black_box(extractor.extract_hashtags(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_extract_hashtags_pest(text: &str) {
    let extractor = Extractor::with_parser_backend(ParserBackend::Pest);
    black_box(extractor.extract_hashtags(text));
}

// --- Validator benchmarks ---

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_validate_tweet_nom(text: &str) {
    let validator = Validator::with_parser_backend(ParserBackend::Nom);
    black_box(validator.is_valid_tweet(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_validate_tweet_pest(text: &str) {
    let validator = Validator::with_parser_backend(ParserBackend::Pest);
    black_box(validator.is_valid_tweet(text));
}

// --- Autolinker benchmarks ---

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_autolink_nom(text: &str) {
    let autolinker = Autolinker::with_parser_backend(false, ParserBackend::Nom);
    black_box(autolinker.autolink(text));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_autolink_pest(text: &str) {
    let autolinker = Autolinker::with_parser_backend(false, ParserBackend::Pest);
    black_box(autolinker.autolink(text));
}

// --- Parse benchmarks ---

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_parse_nom(text: &str) {
    let config = Configuration::default();
    black_box(twitter_text::parse_with_parser_backend(
        text,
        &config,
        true,
        ParserBackend::Nom,
    ));
}

#[library_benchmark]
#[bench::simple(SIMPLE_TEXT)]
#[bench::mention(TEXT_WITH_MENTION)]
#[bench::hashtag(TEXT_WITH_HASHTAG)]
#[bench::url(TEXT_WITH_URL)]
#[bench::cashtag(TEXT_WITH_CASHTAG)]
#[bench::complex(COMPLEX_TEXT)]
fn bench_parse_pest(text: &str) {
    let config = Configuration::default();
    black_box(twitter_text::parse_with_parser_backend(
        text,
        &config,
        true,
        ParserBackend::Pest,
    ));
}

// --- Group definitions ---

library_benchmark_group!(
    name = extract_nom;
    benchmarks = bench_extract_urls_nom, bench_extract_mentions_nom, bench_extract_hashtags_nom
);

library_benchmark_group!(
    name = extract_pest;
    benchmarks = bench_extract_urls_pest, bench_extract_mentions_pest, bench_extract_hashtags_pest
);

library_benchmark_group!(
    name = validate;
    benchmarks = bench_validate_tweet_nom, bench_validate_tweet_pest
);

library_benchmark_group!(
    name = autolink;
    benchmarks = bench_autolink_nom, bench_autolink_pest
);

library_benchmark_group!(
    name = parse;
    benchmarks = bench_parse_nom, bench_parse_pest
);

main!(
    library_benchmark_groups = extract_nom,
    extract_pest,
    validate,
    autolink,
    parse
);
