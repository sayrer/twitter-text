#!/usr/bin/env python3
# Copyright 2026 Robert Sayre
# Licensed under the Apache License, Version 2.0
# http://www.apache.org/licenses/LICENSE-2.0
"""Generate config/v4.json and its per-block tests from the Unicode Character Database.

Configuration v3 weighs U+0000-U+10FF (and three slices of General
Punctuation) at one unit and everything else at two. The double weight was
meant for East Asian logographic and syllabic scripts, but the cutoff also
catches every alphabet, abjad, abugida and syllabary that Unicode encodes
above U+10FF: Ethiopic, Cherokee, Khmer, the Latin, Greek, Cyrillic and
Arabic extension blocks, and roughly 170 more. v4 corrects that without
touching v3, which stays byte-identical to upstream twitter-text.

The rule, applied per Unicode block above U+10FF:

* the block has at least one letter (General_Category L*) or combining mark
  (Mn, Mc, Me), and
* none of its letters are East_Asian_Width Wide or Fullwidth, and
* it is not one of the blocks listed in EXCLUDED below: Korean, logographic
  and hieroglyphic scripts, and letter-shaped symbols such as the
  Mathematical Alphanumeric Symbols.

Such a block weighs one unit per code point under v4. Adjacent blocks are
merged into one range. Everything v3 already weighs at one unit is kept
verbatim, so v4 is a superset of v3.

Usage:
    config/generate_v4.py --ucd DIR

DIR must hold Blocks.txt, UnicodeData.txt and EastAsianWidth.txt from the
Unicode version named in UNICODE_VERSION; pass --download to fetch them into
DIR first. Writes config/v4.json and rust/twitter-text/tests/v4_script_weights.rs
relative to the repository root. Needs only the Python standard library.
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import unicodedata
import urllib.request

UNICODE_VERSION = "18.0.0"
UCD_FILES = ["Blocks.txt", "UnicodeData.txt", "EastAsianWidth.txt"]
UCD_URL = "https://www.unicode.org/Public/{}/ucd/{}"

SCALE_WEIGHT = 100

# The v3 ranges, which v4 keeps verbatim. Everything at or below U+10FF plus
# three slices of General Punctuation: spaces and format controls, dashes and
# quotation marks, and primes.
V3_RANGES = [(0x0000, 0x10FF), (0x2000, 0x200D), (0x2010, 0x201F), (0x2032, 0x2037)]

# Blocks above U+10FF that pass the letter and width tests but stay at the
# default weight, with the reason.
EXCLUDED = {
    # Korean. Hangul Syllables and the leading consonants in Hangul Jamo are
    # Wide, which already keeps those blocks out; the trailing jamo in this
    # block only ever combine into those syllables.
    "Hangul Jamo Extended-B": "Korean",
    # Logographic and hieroglyphic scripts: one code point is a word or a
    # syllable, like the Han characters the default weight was designed for.
    "Linear A": "logographic",
    "Linear B Ideograms": "logographic",
    "Cypro-Minoan": "logographic",
    "Cuneiform": "logographic",
    "Early Dynastic Cuneiform": "logographic",
    "Egyptian Hieroglyphs": "logographic",
    "Egyptian Hieroglyph Format Controls": "logographic",
    "Egyptian Hieroglyphs Extended-A": "logographic",
    "Anatolian Hieroglyphs": "logographic",
    # Undeciphered pictographs.
    "Phaistos Disc": "logographic",
    # Letter-shaped symbols and notation, not writing systems.
    "Coptic Epact Numbers": "symbols",
    "Superscripts and Subscripts": "symbols",
    "Letterlike Symbols": "symbols",
    "Number Forms": "symbols",
    "Combining Diacritical Marks for Symbols": "symbols",
    "Supplemental Punctuation": "symbols",
    "Mathematical Alphanumeric Symbols": "symbols",
    "Arabic Mathematical Alphabetic Symbols": "symbols",
    "Musical Symbols": "symbols",
    "Musical Symbols Supplement": "symbols",
    "Ancient Greek Musical Notation": "symbols",
    "Znamenny Musical Notation": "symbols",
    "Sutton SignWriting": "symbols",
    # Format characters whose weight upstream twitter-text defines.
    "Variation Selectors": "format",
    "Variation Selectors Supplement": "format",
}


def parse_blocks(path):
    blocks = []
    with open(path, encoding="utf-8") as f:
        for line in f:
            m = re.match(r"([0-9A-F]+)\.\.([0-9A-F]+); (.*)", line)
            if m:
                blocks.append((int(m[1], 16), int(m[2], 16), m[3].strip()))
    return blocks


def parse_unicode_data(path):
    """Return {cp: (general_category, canonical_decomposition_or_empty)}."""
    data = {}
    first = None
    with open(path, encoding="utf-8") as f:
        for line in f:
            fields = line.rstrip("\n").split(";")
            cp, name, gc, decomp = int(fields[0], 16), fields[1], fields[2], fields[5]
            if name.endswith("First>"):
                first = cp
                continue
            if name.endswith("Last>"):
                for c in range(first, cp + 1):
                    data[c] = (gc, "")
                first = None
                continue
            # Tagged decompositions (<compat>, <font>, ...) are not canonical
            # and NFC leaves them alone.
            data[cp] = (gc, "" if decomp.startswith("<") else decomp)
    return data


def parse_east_asian_width(path):
    width = {}
    with open(path, encoding="utf-8") as f:
        for line in f:
            line = line.split("#")[0].strip()
            if not line:
                continue
            rng, w = [x.strip() for x in line.split(";")[:2]]
            if ".." in rng:
                a, b = [int(x, 16) for x in rng.split("..")]
            else:
                a = b = int(rng, 16)
            for c in range(a, b + 1):
                width[c] = w
    return width


def in_v3(cp):
    return any(a <= cp <= b for a, b in V3_RANGES)


def nfc_stable(cp, decomp):
    """True when NFC maps the code point to itself.

    Python's own tables may predate UNICODE_VERSION, so they are consulted only
    for code points they know; anything newer is accepted when it has no
    canonical decomposition, which holds for every newly encoded script.
    """
    if unicodedata.category(chr(cp)) != "Cn":
        return unicodedata.normalize("NFC", chr(cp)) == chr(cp)
    return decomp == ""


def classify(blocks, data, width):
    """Return (included, excluded) lists of (start, end, name, sample_cp)."""
    included, excluded = [], []
    for start, end, name in blocks:
        if end <= 0x10FF:
            continue
        letters, marks, wide = [], [], 0
        for cp in range(start, end + 1):
            if in_v3(cp) or cp not in data:
                continue
            gc = data[cp][0]
            if gc.startswith("L"):
                if width.get(cp, "N") in ("W", "F"):
                    wide += 1
                else:
                    letters.append(cp)
            elif gc in ("Mn", "Mc", "Me"):
                marks.append(cp)
        if not letters and not marks:
            continue
        if wide:
            continue
        candidates = [cp for cp in letters + marks if nfc_stable(cp, data[cp][1])]
        if not candidates:
            sys.exit("{}: no NFC-stable letter or mark to use as a test sample".format(name))
        entry = (start, end, name, candidates[0])
        if name in EXCLUDED:
            excluded.append(entry)
        else:
            included.append(entry)
    return included, excluded


def merge_adjacent(included):
    ranges = []
    for start, end, _name, _sample in included:
        if ranges and ranges[-1][1] + 1 == start:
            ranges[-1] = (ranges[-1][0], end)
        else:
            ranges.append((start, end))
    return ranges


def write_json(path, ranges):
    config = {
        "version": 4,
        "maxWeightedTweetLength": 280,
        "scale": 100,
        "defaultWeight": 200,
        "emojiParsingEnabled": True,
        "transformedURLLength": 23,
        "ranges": [{"start": a, "end": b, "weight": SCALE_WEIGHT} for a, b in ranges],
    }
    with open(path, "w", encoding="utf-8") as f:
        json.dump(config, f, indent=2)
        f.write("\n")


def rust_ident(name):
    ident = re.sub(r"[^a-z0-9]+", "_", name.lower()).strip("_")
    return ident


def write_rust_tests(path, included, excluded):
    lines = [
        "// Copyright 2026 Robert Sayre",
        "// Licensed under the Apache License, Version 2.0",
        "// http://www.apache.org/licenses/LICENSE-2.0",
        "//",
        "// Generated by config/generate_v4.py from Unicode {}. Do not edit;".format(UNICODE_VERSION),
        "// rerun the generator instead.",
        "",
        "//! One test per Unicode block that configuration v4 weighs at one unit",
        "//! per code point and v3 leaves at the default weight. Each test checks",
        "//! that a single v4 range covers the whole block, that no v3 range",
        "//! touches it, and that a sample letter from the block weighs one unit",
        "//! under v4 and two under v3 and under the default configuration.",
        "",
        "use twitter_text::parse;",
        "use twitter_text_config::{config_v3, config_v4, default, Configuration};",
        "",
        "fn weighted_length(config: &Configuration, text: &str) -> i32 {",
        "    parse(text, config, false).weighted_length",
        "}",
        "",
        "fn check_block(name: &str, start: i32, end: i32, sample: &str) {",
        "    let v4 = config_v4();",
        "    let v3 = config_v3();",
        "    assert!(",
        "        v4.ranges",
        "            .iter()",
        "            .any(|r| r.contains(start) && r.contains(end) && r.weight == 100),",
        "        \"{} U+{:04X}..U+{:04X} should lie within one v4 range of weight 100\",",
        "        name, start, end",
        "    );",
        "    assert!(",
        "        !v3.ranges.iter().any(|r| r.contains(start) || r.contains(end)),",
        "        \"{} U+{:04X}..U+{:04X} must stay at the default weight in v3\",",
        "        name, start, end",
        "    );",
        "    assert_eq!(sample.chars().count(), 1);",
        "    assert_eq!(weighted_length(v4, sample), 1, \"{}: {:?} should weigh one unit under v4\", name, sample);",
        "    assert_eq!(weighted_length(v3, sample), 2, \"{}: {:?} should weigh two units under v3\", name, sample);",
        "    assert_eq!(weighted_length(default(), sample), 2, \"{}: {:?} should weigh two units by default\", name, sample);",
        "}",
        "",
        "macro_rules! block_tests {",
        "    ($($id:ident: ($name:expr, $start:expr, $end:expr, $sample:expr);)*) => {",
        "        $(",
        "            #[test]",
        "            fn $id() {",
        "                check_block($name, $start, $end, $sample);",
        "            }",
        "        )*",
        "    };",
        "}",
        "",
        "block_tests! {",
    ]
    for start, end, name, sample in included:
        lines.append(
            "    {}: ({}, 0x{:04X}, 0x{:04X}, \"\\u{{{:X}}}\");".format(
                rust_ident(name), json.dumps(name), start, end, sample
            )
        )
    lines += [
        "}",
        "",
        "/// Blocks above U+10FF that hold narrow letters but stay at the default",
        "/// weight under v4 as well: Korean, logographic scripts, and letter-shaped",
        "/// symbols.",
        "#[test]",
        "fn excluded_blocks_keep_the_default_weight() {",
        "    let v4 = config_v4();",
        "    let excluded: &[(&str, i32, i32, &str)] = &[",
    ]
    for start, end, name, sample in excluded:
        lines.append(
            "        ({}, 0x{:04X}, 0x{:04X}, \"\\u{{{:X}}}\"),".format(json.dumps(name), start, end, sample)
        )
    lines += [
        "    ];",
        "    for (name, start, end, sample) in excluded {",
        "        assert!(",
        "            !v4.ranges.iter().any(|r| r.contains(*start) || r.contains(*end)),",
        "            \"{} U+{:04X}..U+{:04X} should not be in any v4 range\",",
        "            name, start, end",
        "        );",
        "        assert_eq!(weighted_length(v4, sample), 2, \"{}: {:?} should weigh two units under v4\", name, sample);",
        "    }",
        "}",
        "",
        "/// v4 is v3 plus the alphabetic blocks: every v3 range appears in v4 verbatim.",
        "#[test]",
        "fn v4_is_a_superset_of_v3() {",
        "    let v4 = config_v4();",
        "    let v3 = config_v3();",
        "    assert_eq!(v4.version, 4);",
        "    assert_eq!(v3.version, 3);",
        "    for range in &v3.ranges {",
        "        assert!(v4.ranges.contains(range), \"v3 range {:?} is missing from v4\", range);",
        "    }",
        "    assert_eq!(v4.max_weighted_tweet_length, v3.max_weighted_tweet_length);",
        "    assert_eq!(v4.scale, v3.scale);",
        "    assert_eq!(v4.default_weight, v3.default_weight);",
        "    assert_eq!(v4.transformed_url_length, v3.transformed_url_length);",
        "    assert_eq!(v4.emoji_parsing_enabled, v3.emoji_parsing_enabled);",
        "}",
        "",
        "/// The ranges are sorted and disjoint, so the first range that contains a",
        "/// code point is the only one that does.",
        "#[test]",
        "fn v4_ranges_are_sorted_and_disjoint() {",
        "    let ranges = &config_v4().ranges;",
        "    assert_eq!(ranges.len(), {});".format(len(merge_adjacent(included)) + len(V3_RANGES)),
        "    for pair in ranges.windows(2) {",
        "        assert!(pair[0].range.end() < pair[1].range.start(), \"{:?} overlaps or follows {:?}\", pair[0], pair[1]);",
        "    }",
        "}",
        "",
    ]
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    rustfmt = shutil.which("rustfmt")
    if rustfmt:
        subprocess.run([rustfmt, "--edition", "2021", path], check=True)
    else:
        print("rustfmt not found; run it on", path, file=sys.stderr)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--ucd", required=True, help="directory holding the UCD files")
    ap.add_argument("--download", action="store_true", help="fetch the UCD files into --ucd first")
    ap.add_argument("--list", action="store_true", help="print the included and excluded blocks")
    args = ap.parse_args()

    os.makedirs(args.ucd, exist_ok=True)
    for name in UCD_FILES:
        path = os.path.join(args.ucd, name)
        if args.download or not os.path.exists(path):
            url = UCD_URL.format(UNICODE_VERSION, name)
            print("fetching", url, file=sys.stderr)
            urllib.request.urlretrieve(url, path)

    blocks = parse_blocks(os.path.join(args.ucd, "Blocks.txt"))
    data = parse_unicode_data(os.path.join(args.ucd, "UnicodeData.txt"))
    width = parse_east_asian_width(os.path.join(args.ucd, "EastAsianWidth.txt"))
    included, excluded = classify(blocks, data, width)

    unknown = set(EXCLUDED) - {name for _, _, name, _ in excluded}
    if unknown:
        sys.exit("EXCLUDED names that match no qualifying block: " + ", ".join(sorted(unknown)))

    if args.list:
        for start, end, name, sample in included:
            print("+ {:04X}..{:04X} {} ({})".format(start, end, name, chr(sample)))
        for start, end, name, sample in excluded:
            print("- {:04X}..{:04X} {} [{}]".format(start, end, name, EXCLUDED[name]))

    ranges = sorted(V3_RANGES + merge_adjacent(included))
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    write_json(os.path.join(root, "config", "v4.json"), ranges)
    write_rust_tests(
        os.path.join(root, "rust", "twitter-text", "tests", "v4_script_weights.rs"), included, excluded
    )
    print(
        "{} blocks included in {} new ranges, {} excluded; {} ranges in total".format(
            len(included), len(merge_adjacent(included)), len(excluded), len(ranges)
        ),
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
