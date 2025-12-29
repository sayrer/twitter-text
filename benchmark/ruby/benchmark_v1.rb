#!/usr/bin/env ruby
# Copyright 2024 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0

# Ruby benchmark for twitter-text v1.14.7
# Compatible with Ruby 1.8.7 (REE) through 3.x

# Set KCODE for Ruby 1.8
$KCODE = 'UTF8' if RUBY_VERSION < '1.9'

require 'yaml'
require 'benchmark'

# Load v1.14.7 Ruby implementation
lib_path = File.expand_path('old_twitter_text', File.dirname(__FILE__))
$LOAD_PATH.unshift(lib_path) unless $LOAD_PATH.include?(lib_path)
require 'twitter-text'

ITERATIONS = 1000
WARMUP_ITERATIONS = 100

def load_yaml(filename)
  data_path = File.join(File.dirname(__FILE__), '..', 'data', filename)
  YAML.load_file(data_path)
end

def benchmark_autolink(data)
  tests = data['tests']
  autolinker = Object.new.extend(Twitter::Autolink)

  # Warmup
  WARMUP_ITERATIONS.times do
    tests.each { |test| autolinker.auto_link(test['text']) }
  end

  # Benchmark
  time = Benchmark.realtime do
    ITERATIONS.times do
      tests.each { |test| autolinker.auto_link(test['text']) }
    end
  end
  ops_per_sec = ITERATIONS / time

  print_results("Autolink", ops_per_sec)
end

def benchmark_extract(data)
  tests = data['tests']
  mentions = tests['mentions'] || []
  urls = tests['urls'] || []
  hashtags = tests['hashtags'] || []
  cashtags = tests['cashtags'] || []

  all_texts = []
  mentions.each { |test| all_texts << test['text'] }
  urls.each { |test| all_texts << test['text'] }
  hashtags.each { |test| all_texts << test['text'] }
  cashtags.each { |test| all_texts << test['text'] }

  extractor = Object.new.extend(Twitter::Extractor)

  # Warmup
  WARMUP_ITERATIONS.times do
    all_texts.each do |text|
      extractor.extract_mentioned_screen_names(text)
      extractor.extract_urls(text)
      extractor.extract_hashtags(text)
      extractor.extract_cashtags(text)
    end
  end

  # Benchmark
  time = Benchmark.realtime do
    ITERATIONS.times do
      all_texts.each do |text|
        extractor.extract_mentioned_screen_names(text)
        extractor.extract_urls(text)
        extractor.extract_hashtags(text)
        extractor.extract_cashtags(text)
      end
    end
  end
  ops_per_sec = ITERATIONS / time

  print_results("Extract", ops_per_sec)
end

def benchmark_validate_tweet(data)
  tests = data['tests']
  tweets = tests['tweets'] || []

  validator = Object.new.extend(Twitter::Validation)

  # Warmup
  WARMUP_ITERATIONS.times do
    tweets.each { |test| validator.tweet_invalid?(test['text']).nil? }
  end

  # Benchmark
  time = Benchmark.realtime do
    ITERATIONS.times do
      tweets.each { |test| validator.tweet_invalid?(test['text']).nil? }
    end
  end
  ops_per_sec = ITERATIONS / time

  print_results("Validate Tweet", ops_per_sec)
end

def benchmark_validate_all(data)
  tests = data['tests']
  tweets = tests['tweets'] || []
  usernames = tests['usernames'] || []
  hashtags = tests['hashtags'] || []
  urls = tests['urls'] || []

  validator = Object.new.extend(Twitter::Validation)

  # Warmup
  WARMUP_ITERATIONS.times do
    tweets.each { |test| validator.tweet_invalid?(test['text']).nil? }
    usernames.each { |test| validator.valid_username?(test['text']) }
    hashtags.each { |test| validator.valid_hashtag?(test['text']) }
    urls.each { |test| validator.valid_url?(test['text']) }
  end

  # Benchmark
  time = Benchmark.realtime do
    ITERATIONS.times do
      tweets.each { |test| validator.tweet_invalid?(test['text']).nil? }
      usernames.each { |test| validator.valid_username?(test['text']) }
      hashtags.each { |test| validator.valid_hashtag?(test['text']) }
      urls.each { |test| validator.valid_url?(test['text']) }
    end
  end
  ops_per_sec = ITERATIONS / time

  print_results("Validate All", ops_per_sec)
end

def print_results(operation, ops_per_sec)
  puts
  puts "#{operation} (#{ITERATIONS} iterations):"
  puts "  Ruby #{RUBY_VERSION}: #{format_number(ops_per_sec.round(0))} ops/sec"
end

def format_number(n)
  n.to_s.reverse.gsub(/(\d{3})(?=\d)/, '\\1,').reverse
end

# Main
puts "Twitter Text v1.14.7 Ruby Benchmark"
puts "===================================="
puts "Ruby version: #{RUBY_VERSION}"
puts "Ruby platform: #{RUBY_PLATFORM}"
puts

autolink_data = load_yaml('autolink.yml')
extract_data = load_yaml('extract.yml')
validate_data = load_yaml('validate.yml')

benchmark_autolink(autolink_data)
benchmark_extract(extract_data)
benchmark_validate_tweet(validate_data)
benchmark_validate_all(validate_data)

puts
puts "Done."
