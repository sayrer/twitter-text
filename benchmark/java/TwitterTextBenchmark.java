// Copyright 2025 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

package benchmark;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import java.io.InputStream;
import java.util.List;
import java.util.Map;

/**
 * Benchmark comparing old Java twitter-text implementation vs Rust FFI bindings.
 * Uses result accumulation to thwart JIT escape analysis optimization.
 *
 * Note: This benchmark reflects realistic usage where Java code works with
 * Java Strings (UTF-16). The Rust FFI pays UTF-16 -> UTF-8 -> UTF-16 conversion
 * cost, which is inherent to the current binding design. A future optimization
 * could add byte[] APIs to avoid this overhead when input is already UTF-8.
 */
public class TwitterTextBenchmark {

    private static final int ITERATIONS = 1000;
    private static final int WARMUP_ITERATIONS = 100;

    // Global sink to prevent escape analysis from optimizing away results
    private static long sink = 0;

    // Old Java implementation
    private static final com.twitter.twittertext.Autolink OLD_AUTOLINK =
        new com.twitter.twittertext.Autolink(false);
    private static final com.twitter.twittertext.Extractor OLD_EXTRACTOR =
        new com.twitter.twittertext.Extractor();
    private static final com.twitter.twittertext.Validator OLD_VALIDATOR =
        new com.twitter.twittertext.Validator();

    public static void main(String[] args) throws Exception {
        System.out.println("Twitter Text Benchmark: Old Java vs Rust FFI");
        System.out.println("=============================================");

        // Load benchmark data
        ObjectMapper mapper = new ObjectMapper(new YAMLFactory());

        Map<String, Object> autolinkData = loadYaml(mapper, "autolink.yml");
        Map<String, Object> extractData = loadYaml(mapper, "extract.yml");
        Map<String, Object> validateData = loadYaml(mapper, "validate.yml");
        Map<String, Object> parseData = loadYaml(mapper, "parse.yml");

        benchmarkAutolink(autolinkData);
        benchmarkExtract(extractData);
        benchmarkValidateTweet(validateData);
        benchmarkValidateAll(validateData);
        benchmarkParse(parseData);

        // Print sink to ensure it's not optimized away
        System.out.println("\nDone. (checksum: " + sink + ")");
    }

    @SuppressWarnings("unchecked")
    private static Map<String, Object> loadYaml(
        ObjectMapper mapper,
        String filename
    ) throws Exception {
        try (
            InputStream is = TwitterTextBenchmark.class.getResourceAsStream(
                "/" + filename
            )
        ) {
            if (is == null) {
                throw new RuntimeException(
                    "Could not find resource: " + filename
                );
            }
            return mapper.readValue(is, Map.class);
        }
    }

    @SuppressWarnings("unchecked")
    private static void benchmarkAutolink(Map<String, Object> data) {
        List<Map<String, String>> tests = (List<Map<String, String>>) data.get(
            "tests"
        );

        // Warmup old - accumulate to prevent optimization
        for (int i = 0; i < WARMUP_ITERATIONS; i++) {
            for (Map<String, String> test : tests) {
                sink += OLD_AUTOLINK.autoLink(test.get("text")).length();
            }
        }

        // Benchmark old - accumulate lengths
        long startOld = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            for (Map<String, String> test : tests) {
                sink += OLD_AUTOLINK.autoLink(test.get("text")).length();
            }
        }
        long elapsedOld = System.nanoTime() - startOld;
        double opsPerSecOld = (ITERATIONS * 1_000_000_000.0) / elapsedOld;

        // Rust FFI version
        try (
            com.sayrer.twitter_text.Autolink rustAutolink =
                new com.sayrer.twitter_text.Autolink()
        ) {
            rustAutolink.setNoFollow(false);

            // Warmup rust - accumulate to prevent optimization
            for (int i = 0; i < WARMUP_ITERATIONS; i++) {
                for (Map<String, String> test : tests) {
                    sink += rustAutolink.autolink(test.get("text")).length();
                }
            }

            // Benchmark rust - accumulate lengths
            long startRust = System.nanoTime();
            for (int i = 0; i < ITERATIONS; i++) {
                for (Map<String, String> test : tests) {
                    sink += rustAutolink.autolink(test.get("text")).length();
                }
            }
            long elapsedRust = System.nanoTime() - startRust;
            double opsPerSecRust = (ITERATIONS * 1_000_000_000.0) / elapsedRust;

            printResults("Autolink", opsPerSecOld, opsPerSecRust);
        }
    }

    @SuppressWarnings("unchecked")
    private static void benchmarkExtract(Map<String, Object> data) {
        Map<String, Object> tests = (Map<String, Object>) data.get("tests");
        List<Map<String, String>> mentions = (List<
            Map<String, String>
        >) tests.get("mentions");
        List<Map<String, String>> urls = (List<Map<String, String>>) tests.get(
            "urls"
        );
        List<Map<String, String>> hashtags = (List<
            Map<String, String>
        >) tests.get("hashtags");
        List<Map<String, String>> cashtags = (List<
            Map<String, String>
        >) tests.get("cashtags");

        // Collect all texts (like JS/Rust benchmark)
        java.util.ArrayList<String> allTexts = new java.util.ArrayList<>();
        for (Map<String, String> test : mentions) {
            allTexts.add(test.get("text"));
        }
        for (Map<String, String> test : urls) {
            allTexts.add(test.get("text"));
        }
        for (Map<String, String> test : hashtags) {
            allTexts.add(test.get("text"));
        }
        for (Map<String, String> test : cashtags) {
            allTexts.add(test.get("text"));
        }

        // Warmup old - call all 4 extract functions for each text
        for (int i = 0; i < WARMUP_ITERATIONS; i++) {
            for (String text : allTexts) {
                sink += OLD_EXTRACTOR.extractMentionedScreennames(text).size();
                sink += OLD_EXTRACTOR.extractURLs(text).size();
                sink += OLD_EXTRACTOR.extractHashtags(text).size();
                sink += OLD_EXTRACTOR.extractCashtags(text).size();
            }
        }

        // Benchmark old
        long startOld = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            for (String text : allTexts) {
                sink += OLD_EXTRACTOR.extractMentionedScreennames(text).size();
                sink += OLD_EXTRACTOR.extractURLs(text).size();
                sink += OLD_EXTRACTOR.extractHashtags(text).size();
                sink += OLD_EXTRACTOR.extractCashtags(text).size();
            }
        }
        long elapsedOld = System.nanoTime() - startOld;
        double opsPerSecOld = (ITERATIONS * 1_000_000_000.0) / elapsedOld;

        // Rust FFI version
        try (
            com.sayrer.twitter_text.Extractor rustExtractor =
                new com.sayrer.twitter_text.Extractor()
        ) {
            // Warmup rust - call all 4 extract functions for each text
            for (int i = 0; i < WARMUP_ITERATIONS; i++) {
                for (String text : allTexts) {
                    sink += rustExtractor.extractMentionedScreennames(
                        text
                    ).length;
                    sink += rustExtractor.extractUrls(text).length;
                    sink += rustExtractor.extractHashtags(text).length;
                    sink += rustExtractor.extractCashtags(text).length;
                }
            }

            // Benchmark rust
            long startRust = System.nanoTime();
            for (int i = 0; i < ITERATIONS; i++) {
                for (String text : allTexts) {
                    sink += rustExtractor.extractMentionedScreennames(
                        text
                    ).length;
                    sink += rustExtractor.extractUrls(text).length;
                    sink += rustExtractor.extractHashtags(text).length;
                    sink += rustExtractor.extractCashtags(text).length;
                }
            }
            long elapsedRust = System.nanoTime() - startRust;
            double opsPerSecRust = (ITERATIONS * 1_000_000_000.0) / elapsedRust;

            printResults("Extract", opsPerSecOld, opsPerSecRust);
        }
    }

    @SuppressWarnings("unchecked")
    private static void benchmarkValidateTweet(Map<String, Object> data) {
        Map<String, Object> tests = (Map<String, Object>) data.get("tests");
        List<Map<String, Object>> tweets = (List<
            Map<String, Object>
        >) tests.get("tweets");

        // Warmup old - accumulate valid count
        for (int i = 0; i < WARMUP_ITERATIONS; i++) {
            for (Map<String, Object> test : tweets) {
                sink += OLD_VALIDATOR.isValidTweet((String) test.get("text"))
                    ? 1
                    : 0;
            }
        }

        // Benchmark old - accumulate valid count
        long startOld = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            for (Map<String, Object> test : tweets) {
                sink += OLD_VALIDATOR.isValidTweet((String) test.get("text"))
                    ? 1
                    : 0;
            }
        }
        long elapsedOld = System.nanoTime() - startOld;
        double opsPerSecOld = (ITERATIONS * 1_000_000_000.0) / elapsedOld;

        // Rust FFI version
        try (
            com.sayrer.twitter_text.Validator rustValidator =
                com.sayrer.twitter_text.Validator.create()
        ) {
            // Warmup rust - accumulate valid count
            for (int i = 0; i < WARMUP_ITERATIONS; i++) {
                for (Map<String, Object> test : tweets) {
                    sink += rustValidator.isValidTweet(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
            }

            // Benchmark rust - accumulate valid count
            long startRust = System.nanoTime();
            for (int i = 0; i < ITERATIONS; i++) {
                for (Map<String, Object> test : tweets) {
                    sink += rustValidator.isValidTweet(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
            }
            long elapsedRust = System.nanoTime() - startRust;
            double opsPerSecRust = (ITERATIONS * 1_000_000_000.0) / elapsedRust;

            printResults("Validate Tweet", opsPerSecOld, opsPerSecRust);
        }
    }

    @SuppressWarnings("unchecked")
    private static void benchmarkValidateAll(Map<String, Object> data) {
        Map<String, Object> tests = (Map<String, Object>) data.get("tests");
        List<Map<String, Object>> tweets = (List<
            Map<String, Object>
        >) tests.get("tweets");
        List<Map<String, Object>> usernames = (List<
            Map<String, Object>
        >) tests.get("usernames");
        List<Map<String, Object>> hashtags = (List<
            Map<String, Object>
        >) tests.get("hashtags");
        List<Map<String, Object>> urls = (List<Map<String, Object>>) tests.get(
            "urls"
        );

        // Rust FFI version only - old Java doesn't have isValidUsername/Hashtag/Url
        try (
            com.sayrer.twitter_text.Validator rustValidator =
                com.sayrer.twitter_text.Validator.create()
        ) {
            // Warmup rust - all 4 validation types
            for (int i = 0; i < WARMUP_ITERATIONS; i++) {
                for (Map<String, Object> test : tweets) {
                    sink += rustValidator.isValidTweet(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
                for (Map<String, Object> test : usernames) {
                    sink += rustValidator.isValidUsername(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
                for (Map<String, Object> test : hashtags) {
                    sink += rustValidator.isValidHashtag(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
                for (Map<String, Object> test : urls) {
                    sink += rustValidator.isValidUrl((String) test.get("text"))
                        ? 1
                        : 0;
                }
            }

            // Benchmark rust - all 4 validation types
            long startRust = System.nanoTime();
            for (int i = 0; i < ITERATIONS; i++) {
                for (Map<String, Object> test : tweets) {
                    sink += rustValidator.isValidTweet(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
                for (Map<String, Object> test : usernames) {
                    sink += rustValidator.isValidUsername(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
                for (Map<String, Object> test : hashtags) {
                    sink += rustValidator.isValidHashtag(
                            (String) test.get("text")
                        )
                        ? 1
                        : 0;
                }
                for (Map<String, Object> test : urls) {
                    sink += rustValidator.isValidUrl((String) test.get("text"))
                        ? 1
                        : 0;
                }
            }
            long elapsedRust = System.nanoTime() - startRust;
            double opsPerSecRust = (ITERATIONS * 1_000_000_000.0) / elapsedRust;

            System.out.printf("%nValidate All (%d iterations):%n", ITERATIONS);
            System.out.printf("  Rust FFI:  %,.0f ops/sec%n", opsPerSecRust);
        }
    }

    @SuppressWarnings("unchecked")
    private static void benchmarkParse(Map<String, Object> data) {
        List<Map<String, Object>> tests = (List<Map<String, Object>>) data.get(
            "tests"
        );

        // Warmup old - accumulate weighted lengths
        for (int i = 0; i < WARMUP_ITERATIONS; i++) {
            for (Map<String, Object> test : tests) {
                sink += com.twitter.twittertext.TwitterTextParser.parseTweet(
                    (String) test.get("text")
                ).weightedLength;
            }
        }

        // Benchmark old - accumulate weighted lengths
        long startOld = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            for (Map<String, Object> test : tests) {
                sink += com.twitter.twittertext.TwitterTextParser.parseTweet(
                    (String) test.get("text")
                ).weightedLength;
            }
        }
        long elapsedOld = System.nanoTime() - startOld;
        double opsPerSecOld = (ITERATIONS * 1_000_000_000.0) / elapsedOld;

        // Rust FFI version
        // Warmup rust - accumulate weighted lengths
        for (int i = 0; i < WARMUP_ITERATIONS; i++) {
            for (Map<String, Object> test : tests) {
                sink += com.sayrer.twitter_text.TwitterTextParser.parseTweet(
                    (String) test.get("text")
                ).weightedLength;
            }
        }

        // Benchmark rust - accumulate weighted lengths
        long startRust = System.nanoTime();
        for (int i = 0; i < ITERATIONS; i++) {
            for (Map<String, Object> test : tests) {
                sink += com.sayrer.twitter_text.TwitterTextParser.parseTweet(
                    (String) test.get("text")
                ).weightedLength;
            }
        }
        long elapsedRust = System.nanoTime() - startRust;
        double opsPerSecRust = (ITERATIONS * 1_000_000_000.0) / elapsedRust;

        printResults("Parse Tweet", opsPerSecOld, opsPerSecRust);
    }

    private static void printResults(
        String operation,
        double oldOpsPerSec,
        double rustOpsPerSec
    ) {
        double speedup = rustOpsPerSec / oldOpsPerSec;
        String label = speedup >= 1 ? "faster" : "slower";
        double ratio = speedup >= 1 ? speedup : (1 / speedup);

        System.out.printf("%n%s (%d iterations):%n", operation, ITERATIONS);
        System.out.printf("  Old Java:  %,.0f ops/sec%n", oldOpsPerSec);
        System.out.printf("  Rust FFI:  %,.0f ops/sec%n", rustOpsPerSec);
        System.out.printf("  Result:    %.1fx %s%n", ratio, label);
    }
}
