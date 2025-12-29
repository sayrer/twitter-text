#!/bin/bash
# Extract twitter-text v1.14.7 for benchmarking with Ruby 1.8.7

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
TARGET_DIR="$SCRIPT_DIR/old_twitter_text"

echo "Extracting twitter-text v1.14.7 to $TARGET_DIR"

# Clean up existing
rm -rf "$TARGET_DIR"
mkdir -p "$TARGET_DIR"

cd "$REPO_ROOT"

# Extract the twitter-text lib files
git archive v1.14.7 rb/lib/twitter-text.rb rb/lib/twitter-text/ | tar -x --strip-components=2 -C "$TARGET_DIR/"

# Extract TLD assets
mkdir -p "$TARGET_DIR/assets"
git show v1.14.7:rb/lib/assets/tld_lib.yml > "$TARGET_DIR/assets/tld_lib.yml"

echo "Done. Files extracted:"
find "$TARGET_DIR" -type f | head -20
