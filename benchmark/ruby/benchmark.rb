#!/usr/bin/env ruby
# Copyright 2024 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0

# Twitter Text Benchmark: Old Ruby vs Rust FFI

require 'yaml'
require 'benchmark'

# Load Rust bindings from Bazel runfiles
runfiles_dir = ENV['RUNFILES_DIR'] || File.expand_path('../..', __dir__)
rust_twittertext_so = File.join(runfiles_dir, 'rust', 'ruby-bindings', 'twittertext.so')

if File.exist?(rust_twittertext_so)
  require rust_twittertext_so
else
  # Fallback for macOS bundle
  rust_twittertext_bundle = rust_twittertext_so.sub('.so', '.bundle')
  if File.exist?(rust_twittertext_bundle)
    require rust_twittertext_bundle
  else
    raise "Cannot find Rust twittertext library at #{rust_twittertext_so} or #{rust_twittertext_bundle}"
  end
end

# Load old Ruby implementation
# Add the rb/lib directory to the load path
old_lib_path = File.join(runfiles_dir, 'rb', 'lib')
$LOAD_PATH.unshift(old_lib_path) unless $LOAD_PATH.include?(old_lib_path)
require 'twitter-text'

ITERATIONS = 1000
WARMUP_ITERATIONS = 100

def load_yaml(filename)
  data_path = File.join(ENV['RUNFILES_DIR'] || '.', 'benchmark', 'data', filename)
  unless File.exist?(data_path)
    # Try relative path
    data_path = File.join(File.dirname(__FILE__), '..', 'data', filename)
  end
  YAML.load_file(data_path)
end

def benchmark_autolink(data)
  tests = data['tests']

  # Old Ruby implementation uses include to mix in the module
  old_autolinker = Object.new.extend(Twitter::TwitterText::Autolink)

  # Rust implementation
  rust_autolinker = Twittertext::Autolinker.new

  # Warmup old
  WARMUP_ITERATIONS.times do
    tests.each { |test| old_autolinker.auto_link(test['text']) }
  end

  # Benchmark old
  old_time = Benchmark.realtime do
    ITERATIONS.times do
      tests.each { |test| old_autolinker.auto_link(test['text']) }
    end
  end
  old_ops_per_sec = ITERATIONS / old_time

  # Warmup rust
  WARMUP_ITERATIONS.times do
    tests.each { |test| rust_autolinker.autolink(test['text']) }
  end

  # Benchmark rust
  rust_time = Benchmark.realtime do
    ITERATIONS.times do
      tests.each { |test| rust_autolinker.autolink(test['text']) }
    end
  end
  rust_ops_per_sec = ITERATIONS / rust_time

  print_results("Autolink", old_ops_per_sec, rust_ops_per_sec)
end

def benchmark_extract(data)
  tests = data['tests']
  mentions = tests['mentions']
  urls = tests['urls']
  hashtags = tests['hashtags']
  cashtags = tests['cashtags']

  # Old Ruby implementation
  old_extractor = Object.new.extend(Twitter::TwitterText::Extractor)

  # Rust implementation
  rust_extractor = Twittertext::Extractor.new

  # Warmup old
  WARMUP_ITERATIONS.times do
    mentions.each { |test| old_extractor.extract_mentioned_screen_names(test['text']) }
    urls.each { |test| old_extractor.extract_urls(test['text']) }
    hashtags.each { |test| old_extractor.extract_hashtags(test['text']) }
    cashtags.each { |test| old_extractor.extract_cashtags(test['text']) }
  end

  # Benchmark old
  old_time = Benchmark.realtime do
    ITERATIONS.times do
      mentions.each { |test| old_extractor.extract_mentioned_screen_names(test['text']) }
      urls.each { |test| old_extractor.extract_urls(test['text']) }
      hashtags.each { |test| old_extractor.extract_hashtags(test['text']) }
      cashtags.each { |test| old_extractor.extract_cashtags(test['text']) }
    end
  end
  old_ops_per_sec = ITERATIONS / old_time

  # Warmup rust
  WARMUP_ITERATIONS.times do
    mentions.each { |test| rust_extractor.extract_mentioned_screennames(test['text']) }
    urls.each { |test| rust_extractor.extract_urls(test['text']) }
    hashtags.each { |test| rust_extractor.extract_hashtags(test['text']) }
    cashtags.each { |test| rust_extractor.extract_cashtags(test['text']) }
  end

  # Benchmark rust
  rust_time = Benchmark.realtime do
    ITERATIONS.times do
      mentions.each { |test| rust_extractor.extract_mentioned_screennames(test['text']) }
      urls.each { |test| rust_extractor.extract_urls(test['text']) }
      hashtags.each { |test| rust_extractor.extract_hashtags(test['text']) }
      cashtags.each { |test| rust_extractor.extract_cashtags(test['text']) }
    end
  end
  rust_ops_per_sec = ITERATIONS / rust_time

  print_results("Extract", old_ops_per_sec, rust_ops_per_sec)
end

def benchmark_validate(data)
  tests = data['tests']
  tweets = tests['tweets']

  # Old Ruby implementation
  old_validator = Object.new.extend(Twitter::TwitterText::Validation)

  # Rust implementation
  rust_validator = Twittertext::Validator.new

  # Warmup old
  WARMUP_ITERATIONS.times do
    tweets.each { |test| old_validator.parse_tweet(test['text'])[:valid] }
  end

  # Benchmark old
  old_time = Benchmark.realtime do
    ITERATIONS.times do
      tweets.each { |test| old_validator.parse_tweet(test['text'])[:valid] }
    end
  end
  old_ops_per_sec = ITERATIONS / old_time

  # Warmup rust
  WARMUP_ITERATIONS.times do
    tweets.each { |test| rust_validator.is_valid_tweet(test['text']) }
  end

  # Benchmark rust
  rust_time = Benchmark.realtime do
    ITERATIONS.times do
      tweets.each { |test| rust_validator.is_valid_tweet(test['text']) }
    end
  end
  rust_ops_per_sec = ITERATIONS / rust_time

  print_results("Validate", old_ops_per_sec, rust_ops_per_sec)
end

def benchmark_parse(data)
  tests = data['tests']

  # Old Ruby implementation
  old_validator = Object.new.extend(Twitter::TwitterText::Validation)

  # Rust implementation uses TwitterTextParser
  config = Twittertext::TwitterTextConfiguration.new

  # Warmup old
  WARMUP_ITERATIONS.times do
    tests.each { |test| old_validator.parse_tweet(test['text']) }
  end

  # Benchmark old
  old_time = Benchmark.realtime do
    ITERATIONS.times do
      tests.each { |test| old_validator.parse_tweet(test['text']) }
    end
  end
  old_ops_per_sec = ITERATIONS / old_time

  # Warmup rust
  WARMUP_ITERATIONS.times do
    tests.each { |test| Twittertext::TwitterTextParser.parse(test['text'], config, true) }
  end

  # Benchmark rust
  rust_time = Benchmark.realtime do
    ITERATIONS.times do
      tests.each { |test| Twittertext::TwitterTextParser.parse(test['text'], config, true) }
    end
  end
  rust_ops_per_sec = ITERATIONS / rust_time

  print_results("Parse Tweet", old_ops_per_sec, rust_ops_per_sec)
end

def print_results(operation, old_ops_per_sec, rust_ops_per_sec)
  speedup = rust_ops_per_sec / old_ops_per_sec
  if speedup >= 1
    label = "faster"
    ratio = speedup
  else
    label = "slower"
    ratio = 1 / speedup
  end

  puts
  puts "#{operation} (#{ITERATIONS} iterations):"
  puts "  Old Ruby:  #{old_ops_per_sec.round(0).to_s.reverse.gsub(/(\d{3})(?=\d)/, '\\1,').reverse} ops/sec"
  puts "  Rust FFI:  #{rust_ops_per_sec.round(0).to_s.reverse.gsub(/(\d{3})(?=\d)/, '\\1,').reverse} ops/sec"
  puts "  Result:    #{'%.1f' % ratio}x #{label}"
end

# Main
puts "Twitter Text Benchmark: Old Ruby vs Rust FFI"
puts "============================================="

autolink_data = load_yaml('autolink.yml')
extract_data = load_yaml('extract.yml')
validate_data = load_yaml('validate.yml')
parse_data = load_yaml('parse.yml')

benchmark_autolink(autolink_data)
benchmark_extract(extract_data)
benchmark_validate(validate_data)
benchmark_parse(parse_data)

puts
puts "Done."
