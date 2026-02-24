#!/bin/bash
# Copyright 2025 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0
#
# Regenerate TLD files from tld_lib.yml.
# Requires: Python 3, PyYAML
#
# By default, only regenerates tlds.rs (the PHF hash set used at runtime).
# The Pest grammar uses character-class-based TLD matching and does not
# need regeneration when TLDs change.
#
# To also regenerate the Pest trie grammar (legacy, requires patricia-trie):
#   ./update_tlds.sh --pest
#
# Usage: ./update_tlds.sh [--pest]

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"

TLD_YAML="$REPO_ROOT/rust/conformance/tests/tld_lib.yml"
TLD_GEN_PHF="$REPO_ROOT/rust/conformance/tests/tld_gen_phf.py"
TLDS_RS="$REPO_ROOT/rust/twitter-text/src/tlds.rs"

UPDATE_PEST=false
for arg in "$@"; do
    case "$arg" in
        --pest) UPDATE_PEST=true ;;
        *) echo "Unknown option: $arg" >&2; exit 1 ;;
    esac
done

if [ "$UPDATE_PEST" = true ]; then
    TLD_GEN="$REPO_ROOT/rust/conformance/tests/tld_gen.py"
    INSERT_SCRIPT="$REPO_ROOT/rust/conformance/tests/insert_tld_grammar.sh"
    PEST_FILE="$REPO_ROOT/rust/parser/src/twitter_text.pest"

    TMPDIR="$(mktemp -d)"
    trap 'rm -rf "$TMPDIR"' EXIT

    echo "==> Generating Pest TLD grammar..."
    python3 "$TLD_GEN" \
        --input "$TLD_YAML" \
        --output "$TMPDIR/generated_tld.pest"

    echo "==> Inserting TLD grammar into twitter_text.pest..."
    bash "$INSERT_SCRIPT" \
        "$PEST_FILE" \
        "$TMPDIR/generated_tld.pest" \
        "$PEST_FILE"
fi

echo "==> Generating tlds.rs PHF hash set..."
python3 "$TLD_GEN_PHF" \
    --input "$TLD_YAML" \
    --output "$TLDS_RS"

echo ""
echo "Done. Updated:"
if [ "$UPDATE_PEST" = true ]; then
    echo "  - $PEST_FILE"
fi
echo "  - $TLDS_RS"
