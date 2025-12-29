# Ruby Benchmark for twitter-text

This directory contains benchmarks for comparing Ruby implementations across different Ruby versions, including Ruby Enterprise Edition 1.8.7.

## Files

- `benchmark_v1.rb` - Benchmark script using twitter-text v1.14.7 (Ruby 1.8.7 compatible)
- `benchmark_ruby_only.rb` - Benchmark script using current twitter-text (Ruby 3.x only)
- `setup_v1.sh` - Script to extract v1.14.7 source from git
- `old_twitter_text/` - Extracted v1.14.7 Ruby implementation

## Setup on Linux (for REE 1.8.7 testing)

### 1. Build Ruby Enterprise Edition 1.8.7

```bash
# Clone the REE fork
git clone https://github.com/twitter-forks/rubyenterpriseedition187-248
cd rubyenterpriseedition187-248

# Install bison if needed
sudo apt install bison

# Build
autoconf
./configure --prefix=$HOME/ree
make -j$(nproc)
make install

# Verify
~/ree/bin/ruby --version
# Should show: ruby 1.8.7 ... Ruby Enterprise Edition 2010.02
```

### 2. Setup the v1.14.7 benchmark files

If `old_twitter_text/` directory already exists (copied from another machine), skip to step 3.

Otherwise, run the setup script and apply the fix:

```bash
cd ~/github/sayrer/twitter-text

# Run setup script
bash benchmark/ruby/setup_v1.sh

# Fix the TLD path in regex.rb (the extracted file has wrong path)
# Edit benchmark/ruby/old_twitter_text/twitter-text/regex.rb
# Change line ~29-34 from:
#     TLDS = YAML.load_file(
#       File.join(
#         File.expand_path('../../..', __FILE__), # project root
#         'lib', 'assets', 'tld_lib.yml'
#       )
#     )
# To:
#     TLDS = YAML.load_file(
#       File.join(
#         File.expand_path('../..', __FILE__), # old_twitter_text dir
#         'assets', 'tld_lib.yml'
#       )
#     )

# Also, the tld_lib.yml file extracted is a symlink reference.
# Get the actual content:
git show v1.14.7:conformance/tld_lib.yml > benchmark/ruby/old_twitter_text/assets/tld_lib.yml
```

### 3. Run the benchmarks

```bash
cd ~/github/sayrer/twitter-text

# With REE 1.8.7
~/ree/bin/ruby benchmark/ruby/benchmark_v1.rb

# With modern Ruby (for comparison)
ruby benchmark/ruby/benchmark_v1.rb
```

## Expected Output

```
Twitter Text v1.14.7 Ruby Benchmark
====================================
Ruby version: 1.8.7
Ruby platform: x86_64-linux

Autolink (1000 iterations):
  Ruby 1.8.7: XXX ops/sec

Extract (1000 iterations):
  Ruby 1.8.7: XXX ops/sec

Validate Tweet (1000 iterations):
  Ruby 1.8.7: XXX ops/sec

Validate All (1000 iterations):
  Ruby 1.8.7: XXX ops/sec

Done.
```

## Notes

- The v1.14.7 code uses `$KCODE = 'UTF8'` for Ruby 1.8.x compatibility
- The `regex_range` helper in v1.14.7 uses `pack('U')` for Ruby 1.8 instead of `\u{}` escapes
- Modern twitter-text (v3.x) uses Unicode escapes that Ruby 1.8.7 cannot parse
- The v1.14.7 API differs slightly from v3.x:
  - Uses `Twitter::Autolink` instead of `Twitter::TwitterText::Autolink`
  - Uses `tweet_invalid?` instead of `parse_tweet`
