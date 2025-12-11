#!/bin/bash
# Run the Obj-C benchmark comparing old implementation vs Rust FFI

set -e

# Bazel runfiles handling - find the runfiles directory
if [[ -n "${RUNFILES_DIR:-}" ]]; then
    RUNFILES="$RUNFILES_DIR"
elif [[ -n "${RUNFILES_MANIFEST_FILE:-}" ]]; then
    RUNFILES="$(dirname "$RUNFILES_MANIFEST_FILE")"
elif [[ -f "${BASH_SOURCE[0]}.runfiles/_main/benchmark/objc/run_benchmark.sh" ]]; then
    RUNFILES="${BASH_SOURCE[0]}.runfiles/_main"
elif [[ -f "${BASH_SOURCE[0]}.runfiles/benchmark/objc/run_benchmark.sh" ]]; then
    RUNFILES="${BASH_SOURCE[0]}.runfiles"
else
    # Fallback to script directory approach
    RUNFILES="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/../.."
fi

# Data files
AUTOLINK_YML="$RUNFILES/benchmark/data/autolink.yml"
EXTRACT_YML="$RUNFILES/benchmark/data/extract.yml"
VALIDATE_YML="$RUNFILES/benchmark/data/validate.yml"
PARSE_YML="$RUNFILES/benchmark/data/parse.yml"

# Binary paths
OLD_BENCHMARK="$RUNFILES/benchmark/objc/old_benchmark"
RUST_BENCHMARK="$RUNFILES/benchmark/objc/rust_benchmark"

echo "Twitter Text Benchmark: Old Obj-C vs Rust FFI"
echo "=============================================="

# Run old benchmark (no autolink support)
OLD_RESULT=$("$OLD_BENCHMARK" "$EXTRACT_YML" "$VALIDATE_YML" "$PARSE_YML")
OLD_EXTRACT=$(echo "$OLD_RESULT" | sed 's/.*"extract": \([0-9.]*\).*/\1/')
OLD_VALIDATE=$(echo "$OLD_RESULT" | sed 's/.*"validate_tweet": \([0-9.]*\).*/\1/')
OLD_PARSE=$(echo "$OLD_RESULT" | sed 's/.*"parse": \([0-9.]*\).*/\1/')

# Run rust benchmark (includes autolink)
RUST_RESULT=$("$RUST_BENCHMARK" "$AUTOLINK_YML" "$EXTRACT_YML" "$VALIDATE_YML" "$PARSE_YML")
RUST_AUTOLINK=$(echo "$RUST_RESULT" | sed 's/.*"autolink": \([0-9.]*\).*/\1/')
RUST_EXTRACT=$(echo "$RUST_RESULT" | sed 's/.*"extract": \([0-9.]*\).*/\1/')
RUST_VALIDATE=$(echo "$RUST_RESULT" | sed 's/.*"validate_tweet": \([0-9.]*\).*/\1/')
RUST_PARSE=$(echo "$RUST_RESULT" | sed 's/.*"parse": \([0-9.]*\).*/\1/')

print_results() {
    local operation=$1
    local old_ops=$2
    local rust_ops=$3

    # Calculate speedup using awk
    local speedup=$(awk "BEGIN {printf \"%.1f\", $rust_ops / $old_ops}")
    local label="faster"

    if (( $(awk "BEGIN {print ($speedup < 1)}") )); then
        speedup=$(awk "BEGIN {printf \"%.1f\", $old_ops / $rust_ops}")
        label="slower"
    fi

    echo ""
    echo "$operation (1000 iterations):"
    printf "  Old Obj-C: %.0f ops/sec\n" "$old_ops"
    printf "  Rust FFI:  %.0f ops/sec\n" "$rust_ops"
    echo "  Result:    ${speedup}x $label"
}

print_rust_only() {
    local operation=$1
    local rust_ops=$2

    echo ""
    echo "$operation (1000 iterations):"
    echo "  Old Obj-C: (not supported)"
    printf "  Rust FFI:  %.0f ops/sec\n" "$rust_ops"
}

print_rust_only "Autolink" "$RUST_AUTOLINK"
print_results "Extract" "$OLD_EXTRACT" "$RUST_EXTRACT"
print_results "Validate" "$OLD_VALIDATE" "$RUST_VALIDATE"
print_results "Parse Tweet" "$OLD_PARSE" "$RUST_PARSE"

echo ""
echo "Done."
