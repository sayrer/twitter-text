// Conformance tests for the old twitter-text API wrapper
// Run with: bazel test //rust/wasm-bindings/old-js/tests:conformance_test

import assert from "node:assert";
import { test, describe } from "node:test";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Find runfiles directory for Bazel
function findRunfilesDir() {
  if (process.env.RUNFILES_DIR) {
    return process.env.RUNFILES_DIR;
  }
  if (process.env.JS_BINARY__RUNFILES) {
    return process.env.JS_BINARY__RUNFILES;
  }
  return null;
}

const runfilesDir = findRunfilesDir();

// Import the wrapper module
let wrapperPath;
if (runfilesDir) {
  wrapperPath = path.join(runfilesDir, "_main/rust/wasm-bindings/old-js/twitter-text.mjs");
} else {
  wrapperPath = path.join(__dirname, "../twitter-text.mjs");
}

const { twttr } = await import(wrapperPath);
const txt = twttr.txt;

// Load YAML conformance files
function loadYAML(filename) {
  const possiblePaths = [];

  if (runfilesDir) {
    possiblePaths.push(path.join(runfilesDir, "_main/conformance", filename));
  }

  possiblePaths.push(
    path.join(__dirname, "../../../../conformance", filename),
    filename
  );

  for (const p of possiblePaths) {
    try {
      const contents = fs.readFileSync(p, "utf8");
      return parse(contents);
    } catch (e) {
      // Try next path
    }
  }
  throw new Error(`Could not load ${filename}`);
}

// ============================================================================
// Extract Tests
// ============================================================================

describe("Extract Conformance Tests (old API)", () => {
  const yaml = loadYAML("extract.yml");

  describe("mentions", () => {
    const tests = yaml.tests.mentions || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.extractMentions(tc.text);
        assert.deepStrictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("urls", () => {
    const tests = yaml.tests.urls || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.extractUrls(tc.text);
        assert.deepStrictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("hashtags", () => {
    const tests = yaml.tests.hashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.extractHashtags(tc.text);
        assert.deepStrictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("cashtags", () => {
    const tests = yaml.tests.cashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.extractCashtags(tc.text);
        assert.deepStrictEqual(result, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// Validate Tests
// ============================================================================

describe("Validate Conformance Tests (old API)", () => {
  const yaml = loadYAML("validate.yml");

  describe("tweets", () => {
    const tests = yaml.tests.tweets || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.isValidTweetText(tc.text, txt.configs.version1);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("usernames", () => {
    const tests = yaml.tests.usernames || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.isValidUsername(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("lists", () => {
    const tests = yaml.tests.lists || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.isValidList(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("hashtags", () => {
    const tests = yaml.tests.hashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.isValidHashtag(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("urls", () => {
    const tests = yaml.tests.urls || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.isValidUrl(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("urls_without_protocol", () => {
    const tests = yaml.tests.urls_without_protocol || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.isValidUrl(tc.text, true, false);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// Autolink Tests
// ============================================================================

describe("Autolink Conformance Tests (old API)", () => {
  const yaml = loadYAML("autolink.yml");
  const defaultOptions = { suppressNoFollow: true, suppressDataScreenName: true };

  describe("usernames", () => {
    const tests = yaml.tests.usernames || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.autoLinkUsernamesOrLists(tc.text, defaultOptions);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("lists", () => {
    const tests = yaml.tests.lists || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.autoLinkUsernamesOrLists(tc.text, defaultOptions);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("hashtags", () => {
    const tests = yaml.tests.hashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.autoLinkHashtags(tc.text, defaultOptions);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("urls", () => {
    const tests = yaml.tests.urls || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.autoLinkUrlsCustom(tc.text, defaultOptions);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("cashtags", () => {
    const tests = yaml.tests.cashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.autoLinkCashtags(tc.text, defaultOptions);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("all", () => {
    const tests = yaml.tests.all || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.autoLink(tc.text, defaultOptions);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// Hit Highlighting Tests
// ============================================================================

describe("Hit Highlighting Conformance Tests (old API)", () => {
  const yaml = loadYAML("hit_highlighting.yml");

  describe("plain_text", () => {
    const tests = yaml.tests.plain_text || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.hitHighlight(tc.text, tc.hits);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("with_links", () => {
    const tests = yaml.tests.with_links || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.hitHighlight(tc.text, tc.hits);
        assert.strictEqual(result, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// TLD Tests
// ============================================================================

describe("TLD Conformance Tests (old API)", () => {
  const yaml = loadYAML("tlds.yml");

  describe("country", () => {
    const tests = yaml.tests.country || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.extractUrls(tc.text);
        assert.deepStrictEqual(result, tc.expected, tc.description);
      });
    }
  });

  describe("generic", () => {
    const tests = yaml.tests.generic || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const result = txt.extractUrls(tc.text);
        assert.deepStrictEqual(result, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// Old API-specific Tests
// ============================================================================

describe("Old API Specific Tests", () => {
  test("htmlEscape escapes special characters", () => {
    assert.strictEqual(txt.htmlEscape("&"), "&amp;");
    assert.strictEqual(txt.htmlEscape("<"), "&lt;");
    assert.strictEqual(txt.htmlEscape(">"), "&gt;");
    assert.strictEqual(txt.htmlEscape('"'), "&quot;");
    assert.strictEqual(txt.htmlEscape("'"), "&#39;");
    assert.strictEqual(txt.htmlEscape("&<>\""), "&amp;&lt;&gt;&quot;");
    assert.strictEqual(txt.htmlEscape(undefined), undefined);
  });

  test("splitTags splits HTML tags", () => {
    assert.deepStrictEqual(txt.splitTags("foo"), ["foo"]);
    assert.deepStrictEqual(txt.splitTags("foo<a>foo"), ["foo", "a", "foo"]);
    assert.deepStrictEqual(txt.splitTags("<><>"), ["", "", "", "", ""]);
  });

  test("configs are available", () => {
    assert.ok(txt.configs.version1);
    assert.ok(txt.configs.version2);
    assert.ok(txt.configs.version3);
  });

  test("parseTweet returns expected fields", () => {
    const result = txt.parseTweet("Hello, world!");
    assert.ok("weightedLength" in result);
    assert.ok("permillage" in result);
    assert.ok("isValid" in result);
    assert.ok("displayRangeStart" in result);
    assert.ok("displayRangeEnd" in result);
    assert.ok("validRangeStart" in result);
    assert.ok("validRangeEnd" in result);
  });

  test("modifyIndicesFromUTF16ToUnicode converts indices", () => {
    const text = "\uD801\uDC00 #hashtag";
    const entities = [{ indices: [3, 11] }];
    txt.modifyIndicesFromUTF16ToUnicode(text, entities);
    assert.deepStrictEqual(entities[0].indices, [2, 10]);
  });

  test("modifyIndicesFromUnicodeToUTF16 converts indices", () => {
    const text = "\uD801\uDC00 #hashtag";
    const entities = [{ indices: [2, 10] }];
    txt.modifyIndicesFromUnicodeToUTF16(text, entities);
    assert.deepStrictEqual(entities[0].indices, [3, 11]);
  });
});

console.log("\nTwitter Text Old API Conformance Tests");
console.log("======================================\n");
