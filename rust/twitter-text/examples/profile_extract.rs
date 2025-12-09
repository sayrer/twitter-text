// Run with: cargo build --release --example profile_extract
// Then profile with: samply record ./target/release/examples/profile_extract
// Or: cargo flamegraph --example profile_extract

use std::time::Instant;
use twitter_text::extractor::{Extract, Extractor};

fn main() {
    // Sample texts similar to benchmark data
    let texts = vec![
        "Hello @world, check out https://example.com/path?query=1#fragment for more info!",
        "Testing #hashtag and @mention extraction with $CASHTAG",
        "Multiple URLs: https://foo.com https://bar.org/path https://baz.net",
        "Unicode test: こんにちは @user #日本語 https://例え.jp/パス",
        "Emoji test: 🎉🚀✨ @twitter #excited https://t.co/abc123",
        "Long text with many entities @user1 @user2 @user3 #tag1 #tag2 #tag3 $SYM1 $SYM2 https://a.com https://b.com https://c.com",
        "Edge cases: email@notamention.com #123invalid $lowercaseinvalid",
        "RT @someone: This is a retweet with #hashtags and https://links.com",
    ];

    let iterations = 10_000;

    // Test WITH url_without_protocol (default)
    {
        let extractor = Extractor::new();
        println!(
            "Running {} iterations WITH url_without_protocol...",
            iterations
        );
        let start = Instant::now();

        for _ in 0..iterations {
            for text in &texts {
                let _ = extractor.extract_urls_with_indices(text);
                let _ = extractor.extract_mentioned_screennames_with_indices(text);
                let _ = extractor.extract_hashtags_with_indices(text);
                let _ = extractor.extract_cashtags_with_indices(text);
            }
        }

        let elapsed = start.elapsed();
        println!("  Time: {:?}", elapsed);
    }

    // Test WITHOUT url_without_protocol
    {
        let mut extractor = Extractor::new();
        extractor.set_extract_url_without_protocol(false);
        println!(
            "Running {} iterations WITHOUT url_without_protocol...",
            iterations
        );
        let start = Instant::now();

        for _ in 0..iterations {
            for text in &texts {
                let _ = extractor.extract_urls_with_indices(text);
                let _ = extractor.extract_mentioned_screennames_with_indices(text);
                let _ = extractor.extract_hashtags_with_indices(text);
                let _ = extractor.extract_cashtags_with_indices(text);
            }
        }

        let elapsed = start.elapsed();
        println!("  Time: {:?}", elapsed);
    }

    println!("Done.");
}
