#!/bin/bash
# Copyright 2025 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0
#
# Regenerate TLD files from tld_lib.yml:
#   - rust/twitter-text/src/tlds.rs: the PHF hash set used by the External
#     and Nom parser backends.
#   - rust/parser/src/twitter_text_full_tld.pest: the TLD trie used by the
#     Pest parser backend.
#
# The default grammar (twitter_text.pest) uses character-class-based TLD
# matching and does not need regeneration when TLDs change.
#
# Requires: Python 3, PyYAML, Bazel (the trie generator's dependencies are
# provided hermetically by Bazel)
#
# Usage: ./update_tlds.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"

TLD_YAML="$REPO_ROOT/rust/conformance/tests/tld_lib.yml"
TLD_GEN_PHF="$REPO_ROOT/rust/conformance/tests/tld_gen_phf.py"
TLDS_RS="$REPO_ROOT/rust/twitter-text/src/tlds.rs"
FULL_PEST="$REPO_ROOT/rust/parser/src/twitter_text_full_tld.pest"

if [ "$#" -ne 0 ]; then
    echo "Usage: $0" >&2
    exit 1
fi

echo "==> Generating tlds.rs PHF hash set..."
python3 "$TLD_GEN_PHF" \
    --input "$TLD_YAML" \
    --output "$TLDS_RS"

echo "==> Generating Pest TLD trie for twitter_text_full_tld.pest..."
(cd "$REPO_ROOT" && bazel build //rust/conformance:update_parser_with_tlds)
GENERATED="$(cd "$REPO_ROOT" && bazel info bazel-bin)/rust/conformance/twitter_text_with_tlds.pest"
cp "$GENERATED" "$FULL_PEST"
chmod 644 "$FULL_PEST"

echo ""
echo "Done. Updated:"
echo "  - $TLDS_RS"
echo "  - $FULL_PEST"
