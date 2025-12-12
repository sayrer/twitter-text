// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Long-text benchmark for Rust:Nom
//! Generates same corpus as JS benchmark for fair comparison
//!
//! Usage: longtext_benchmark [--backend=nom|pest|external]

use std::env;
use std::time::Instant;
use twitter_text::extractor::{Extract, Extractor};
use twitter_text::ExternalValidator;

const CORPUS_SIZE: usize = 150_000;

// Filler text segments (no entities) - must match JS exactly
const FILLERS: &[&str] = &[
    "This is some regular text without any special entities just plain words. ",
    "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod. ",
    "The quick brown fox jumps over the lazy dog multiple times in a row. ",
    "Here we have more filler content to pad out the tweet to proper length. ",
    "Random thoughts and musings that don't contain any special characters. ",
    "Just normal everyday text that needs to be parsed but has no entities. ",
    "More padding text here to reach the target length of about 2000 chars. ",
    "Writing sample text is harder than it seems when you need lots of it. ",
];

// Entity templates - must match JS exactly
const MENTIONS: &[&str] = &["@user", "@elonmusk", "@TwitterAPI", "@github", "@rustlang"];
const HASHTAGS: &[&str] = &["#tech", "#coding", "#AI", "#startup", "#news", "#trending"];
const URLS: &[&str] = &[
    "https://example.com",
    "https://t.co/abc123",
    "https://github.com/test",
];
const CASHTAGS: &[&str] = &["$AAPL", "$TSLA", "$GOOGL", "$BTC"];

fn generate_long_text(mut seed: u32) -> String {
    let mut text = String::with_capacity(2100);
    let target_length = 2000;
    let entities_per_segment = 3;

    while text.len() < target_length {
        // Add filler (about 150 chars)
        text.push_str(FILLERS[(seed as usize) % FILLERS.len()]);
        seed = seed.wrapping_mul(31).wrapping_add(17);

        // Add 3 entities
        for e in 0..entities_per_segment {
            if text.len() >= target_length {
                break;
            }
            let entity_type = ((seed as usize) + e) % 4;
            match entity_type {
                0 => {
                    text.push_str(MENTIONS[((seed as usize) + e) % MENTIONS.len()]);
                    text.push(' ');
                }
                1 => {
                    text.push_str(HASHTAGS[((seed as usize) + e) % HASHTAGS.len()]);
                    text.push(' ');
                }
                2 => {
                    text.push_str(URLS[((seed as usize) + e) % URLS.len()]);
                    text.push(' ');
                }
                3 => {
                    text.push_str(CASHTAGS[((seed as usize) + e) % CASHTAGS.len()]);
                    text.push(' ');
                }
                _ => unreachable!(),
            }
        }
        seed = seed.wrapping_mul(31).wrapping_add(17);
    }

    // Add unique suffix to prevent memoization
    let mut result = text[..target_length - 10].to_string();
    result.push_str(&format!(" #id{}", seed));
    result
}

fn parse_backend(args: &[String]) -> ExternalValidator {
    for arg in args {
        if arg.starts_with("--backend=") {
            return match arg.strip_prefix("--backend=").unwrap() {
                "pest" => ExternalValidator::Pest,
                "external" => ExternalValidator::External,
                "nom" => ExternalValidator::Nom,
                other => {
                    eprintln!("Unknown backend: {}. Using 'nom'.", other);
                    ExternalValidator::Nom
                }
            };
        }
    }
    ExternalValidator::Nom
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let backend = parse_backend(&args);

    let backend_name = match backend {
        ExternalValidator::Nom => "Nom",
        ExternalValidator::Pest => "Pest",
        ExternalValidator::External => "External",
    };

    println!("Long-Text Benchmark: Rust:{}", backend_name);
    println!("======================================================================");

    // Generate corpus
    print!("Generating {} unique 2000-char texts... ", CORPUS_SIZE);
    let start = Instant::now();
    let texts: Vec<String> = (0..CORPUS_SIZE)
        .map(|i| generate_long_text(i as u32))
        .collect();
    let gen_elapsed = start.elapsed();
    println!("done ({:.2}s)", gen_elapsed.as_secs_f64());

    let avg_len: usize = texts.iter().map(|t| t.len()).sum::<usize>() / texts.len();
    println!("Corpus: {} texts, avg {} chars each", CORPUS_SIZE, avg_len);
    println!();

    // Create extractor
    let extractor = Extractor::with_external_validator(backend);

    // Benchmark
    println!("Benchmarking extract operations...");
    let start = Instant::now();
    for text in &texts {
        let _ = extractor.extract_mentioned_screennames(text);
        let _ = extractor.extract_urls(text);
        let _ = extractor.extract_hashtags(text);
        let _ = extractor.extract_cashtags(text);
    }
    let elapsed = start.elapsed().as_secs_f64();

    let tps = (CORPUS_SIZE as f64) / elapsed;
    println!();
    println!("======================================================================");
    println!("RESULTS");
    println!("======================================================================");
    println!("Rust:{} Extract: {:.0} tweets/sec", backend_name, tps);
    println!("Time: {:.2}s", elapsed);
    println!();
    println!("Done.");
}
