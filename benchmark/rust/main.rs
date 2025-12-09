// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Pure Rust twitter-text benchmark

use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::time::Instant;
use twitter_text::autolinker::Autolinker;
use twitter_text::extractor::{Extract, Extractor, ValidatingExtractor};
use twitter_text_config::Configuration;

const ITERATIONS: u32 = 1000;
const WARMUP_ITERATIONS: u32 = 100;

#[derive(Debug, Deserialize)]
struct TestCase {
    #[allow(dead_code)]
    description: Option<String>,
    text: String,
    #[allow(dead_code)]
    expected: Option<serde_yaml_ng::Value>,
}

#[derive(Debug, Deserialize)]
struct ExtractTests {
    mentions: Option<Vec<TestCase>>,
    urls: Option<Vec<TestCase>>,
    hashtags: Option<Vec<TestCase>>,
    cashtags: Option<Vec<TestCase>>,
}

#[derive(Debug, Deserialize)]
struct ExtractData {
    tests: ExtractTests,
}

#[derive(Debug, Deserialize)]
struct ValidateTests {
    tweets: Option<Vec<TestCase>>,
}

#[derive(Debug, Deserialize)]
struct ValidateData {
    tests: ValidateTests,
}

#[derive(Debug, Deserialize)]
struct ParseData {
    tests: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
struct AutolinkData {
    tests: Vec<TestCase>,
}

fn benchmark_autolink(data: &AutolinkData) -> f64 {
    let autolinker = Autolinker::new(false);
    let tests = &data.tests;

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        for test in tests {
            let _ = autolinker.autolink(&test.text);
        }
    }

    // Benchmark
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for test in tests {
            let _ = autolinker.autolink(&test.text);
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    ITERATIONS as f64 / elapsed
}

fn benchmark_extract(data: &ExtractData) -> f64 {
    let extractor = Extractor::new();

    // Collect all texts like the JS benchmark does
    let mut all_texts: Vec<&str> = Vec::new();
    if let Some(mentions) = &data.tests.mentions {
        for test in mentions {
            all_texts.push(&test.text);
        }
    }
    if let Some(urls) = &data.tests.urls {
        for test in urls {
            all_texts.push(&test.text);
        }
    }
    if let Some(hashtags) = &data.tests.hashtags {
        for test in hashtags {
            all_texts.push(&test.text);
        }
    }
    if let Some(cashtags) = &data.tests.cashtags {
        for test in cashtags {
            all_texts.push(&test.text);
        }
    }

    // Warmup - call all 4 extract functions for each text (like JS benchmark)
    for _ in 0..WARMUP_ITERATIONS {
        for text in &all_texts {
            let _ = extractor.extract_mentioned_screennames(text);
            let _ = extractor.extract_urls(text);
            let _ = extractor.extract_hashtags(text);
            let _ = extractor.extract_cashtags(text);
        }
    }

    // Benchmark
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for text in &all_texts {
            let _ = extractor.extract_mentioned_screennames(text);
            let _ = extractor.extract_urls(text);
            let _ = extractor.extract_hashtags(text);
            let _ = extractor.extract_cashtags(text);
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    ITERATIONS as f64 / elapsed
}

fn benchmark_validate(data: &ValidateData) -> f64 {
    let config = Configuration::default();
    let extractor = ValidatingExtractor::new(&config);
    let tweets = data
        .tests
        .tweets
        .as_ref()
        .map(|v| v.as_slice())
        .unwrap_or(&[]);

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        for test in tweets {
            let _ = extractor.extract_urls_with_indices(&test.text);
        }
    }

    // Benchmark
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for test in tweets {
            let _ = extractor.extract_urls_with_indices(&test.text);
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    ITERATIONS as f64 / elapsed
}

fn benchmark_parse(data: &ParseData) -> f64 {
    let config = Configuration::default();

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        for test in &data.tests {
            let _ = twitter_text::parse(&test.text, &config, true);
        }
    }

    // Benchmark
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for test in &data.tests {
            let _ = twitter_text::parse(&test.text, &config, true);
        }
    }
    let elapsed = start.elapsed().as_secs_f64();
    ITERATIONS as f64 / elapsed
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "Usage: {} <autolink.yml> <extract.yml> <validate.yml> <parse.yml>",
            args[0]
        );
        std::process::exit(1);
    }

    println!("Twitter Text Benchmark: Pure Rust");
    println!("==================================");

    // Load data
    let autolink_content = fs::read_to_string(&args[1]).expect("Failed to read autolink.yml");
    let extract_content = fs::read_to_string(&args[2]).expect("Failed to read extract.yml");
    let validate_content = fs::read_to_string(&args[3]).expect("Failed to read validate.yml");
    let parse_content = fs::read_to_string(&args[4]).expect("Failed to read parse.yml");

    let autolink_data: AutolinkData =
        serde_yaml_ng::from_str(&autolink_content).expect("Failed to parse autolink.yml");
    let extract_data: ExtractData =
        serde_yaml_ng::from_str(&extract_content).expect("Failed to parse extract.yml");
    let validate_data: ValidateData =
        serde_yaml_ng::from_str(&validate_content).expect("Failed to parse validate.yml");
    let parse_data: ParseData =
        serde_yaml_ng::from_str(&parse_content).expect("Failed to parse parse.yml");

    let autolink_ops = benchmark_autolink(&autolink_data);
    let extract_ops = benchmark_extract(&extract_data);
    let validate_ops = benchmark_validate(&validate_data);
    let parse_ops = benchmark_parse(&parse_data);

    println!("\nAutolink ({} iterations):", ITERATIONS);
    println!("  Rust: {:.0} ops/sec", autolink_ops);

    println!("\nExtract ({} iterations):", ITERATIONS);
    println!("  Rust: {:.0} ops/sec", extract_ops);

    println!("\nValidate ({} iterations):", ITERATIONS);
    println!("  Rust: {:.0} ops/sec", validate_ops);

    println!("\nParse Tweet ({} iterations):", ITERATIONS);
    println!("  Rust: {:.0} ops/sec", parse_ops);

    println!("\nDone.");
}
