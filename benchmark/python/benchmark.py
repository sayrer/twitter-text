#!/usr/bin/env python3
# Copyright 2025 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0

"""
Twitter Text Benchmark: Python (via Rust FFI)

Usage:
    benchmark.py [<autolink.yml> <extract.yml> <validate.yml> <parse.yml>]

If no arguments are provided, uses Bazel runfiles to find data files.
"""

import os
import sys
import time

import twitter_text
import yaml

ITERATIONS = 1000
WARMUP_ITERATIONS = 100


def get_runfiles_path(path: str) -> str:
    """Get the path to a file in Bazel runfiles."""
    # Try the runfiles directory relative to the script
    runfiles_dir = os.environ.get("RUNFILES_DIR")
    if runfiles_dir:
        full_path = os.path.join(runfiles_dir, "_main", path)
        if os.path.exists(full_path):
            return full_path

    # Try relative to script location
    script_dir = os.path.dirname(os.path.abspath(__file__))
    # Look for runfiles manifest
    for parent in [script_dir, os.path.dirname(script_dir)]:
        candidate = os.path.join(parent, "_main", path)
        if os.path.exists(candidate):
            return candidate
        # Also try without _main prefix
        candidate = os.path.join(parent, path)
        if os.path.exists(candidate):
            return candidate

    # Fallback: return the path as-is
    return path


def load_yaml(path: str) -> dict:
    with open(path, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def benchmark_autolink(data: dict) -> float:
    autolinker = twitter_text.Autolinker()
    autolinker.set_no_follow(False)
    tests = data.get("tests", [])

    # Warmup
    for _ in range(WARMUP_ITERATIONS):
        for test in tests:
            autolinker.autolink(test["text"])

    # Benchmark
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        for test in tests:
            autolinker.autolink(test["text"])
    elapsed = time.perf_counter() - start

    return ITERATIONS / elapsed


def benchmark_extract(data: dict) -> float:
    extractor = twitter_text.Extractor()
    tests = data.get("tests", {})

    # Collect all texts (like JS/Rust benchmark)
    all_texts = []
    for key in ["mentions", "urls", "hashtags", "cashtags"]:
        if key in tests and tests[key]:
            all_texts.extend(test["text"] for test in tests[key])

    # Warmup - call all 4 extract functions for each text
    for _ in range(WARMUP_ITERATIONS):
        for text in all_texts:
            extractor.extract_mentioned_screennames(text)
            extractor.extract_urls(text)
            extractor.extract_hashtags(text)
            extractor.extract_cashtags(text)

    # Benchmark
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        for text in all_texts:
            extractor.extract_mentioned_screennames(text)
            extractor.extract_urls(text)
            extractor.extract_hashtags(text)
            extractor.extract_cashtags(text)
    elapsed = time.perf_counter() - start

    return ITERATIONS / elapsed


def benchmark_validate_tweet(data: dict) -> float:
    validator = twitter_text.Validator()
    tests = data.get("tests", {})
    tweets = tests.get("tweets", [])

    # Warmup
    for _ in range(WARMUP_ITERATIONS):
        for test in tweets:
            validator.is_valid_tweet(test["text"])

    # Benchmark
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        for test in tweets:
            validator.is_valid_tweet(test["text"])
    elapsed = time.perf_counter() - start

    return ITERATIONS / elapsed


def benchmark_validate_all(data: dict) -> float:
    validator = twitter_text.Validator()
    tests = data.get("tests", {})
    tweets = tests.get("tweets", [])
    usernames = tests.get("usernames", [])
    hashtags = tests.get("hashtags", [])
    urls = tests.get("urls", [])

    # Warmup - call all 4 validate functions
    for _ in range(WARMUP_ITERATIONS):
        for test in tweets:
            validator.is_valid_tweet(test["text"])
        for test in usernames:
            validator.is_valid_username(test["text"])
        for test in hashtags:
            validator.is_valid_hashtag(test["text"])
        for test in urls:
            validator.is_valid_url(test["text"])

    # Benchmark
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        for test in tweets:
            validator.is_valid_tweet(test["text"])
        for test in usernames:
            validator.is_valid_username(test["text"])
        for test in hashtags:
            validator.is_valid_hashtag(test["text"])
        for test in urls:
            validator.is_valid_url(test["text"])
    elapsed = time.perf_counter() - start

    return ITERATIONS / elapsed


def benchmark_parse(data: dict) -> float:
    config = twitter_text.TwitterTextConfiguration()
    tests = data.get("tests", [])

    # Warmup
    for _ in range(WARMUP_ITERATIONS):
        for test in tests:
            twitter_text.TwitterTextParser.parse(test["text"], config, True)

    # Benchmark
    start = time.perf_counter()
    for _ in range(ITERATIONS):
        for test in tests:
            twitter_text.TwitterTextParser.parse(test["text"], config, True)
    elapsed = time.perf_counter() - start

    return ITERATIONS / elapsed


def main():
    if len(sys.argv) >= 5:
        # Use command-line arguments
        autolink_path = sys.argv[1]
        extract_path = sys.argv[2]
        validate_path = sys.argv[3]
        parse_path = sys.argv[4]
    else:
        # Use Bazel runfiles
        autolink_path = get_runfiles_path("benchmark/data/autolink.yml")
        extract_path = get_runfiles_path("benchmark/data/extract.yml")
        validate_path = get_runfiles_path("benchmark/data/validate.yml")
        parse_path = get_runfiles_path("benchmark/data/parse.yml")

    print("Twitter Text Benchmark: Python (via Rust FFI)")
    print("==============================================")

    # Load YAML files
    autolink_data = load_yaml(autolink_path)
    extract_data = load_yaml(extract_path)
    validate_data = load_yaml(validate_path)
    parse_data = load_yaml(parse_path)

    # Run benchmarks
    autolink_ops = benchmark_autolink(autolink_data)
    extract_ops = benchmark_extract(extract_data)
    validate_tweet_ops = benchmark_validate_tweet(validate_data)
    validate_all_ops = benchmark_validate_all(validate_data)
    parse_ops = benchmark_parse(parse_data)

    # Print results
    print()
    print(f"Autolink ({ITERATIONS} iterations):")
    print(f"  Python: {int(autolink_ops)} ops/sec")

    print()
    print(f"Extract ({ITERATIONS} iterations):")
    print(f"  Python: {int(extract_ops)} ops/sec")

    print()
    print(f"Validate Tweet ({ITERATIONS} iterations):")
    print(f"  Python: {int(validate_tweet_ops)} ops/sec")

    print()
    print(f"Validate All ({ITERATIONS} iterations):")
    print(f"  Python: {int(validate_all_ops)} ops/sec")

    print()
    print(f"Parse Tweet ({ITERATIONS} iterations):")
    print(f"  Python: {int(parse_ops)} ops/sec")

    print()
    print("Done.")


if __name__ == "__main__":
    main()
