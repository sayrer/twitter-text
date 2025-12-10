// twitter-text WASM bindings conformance tests
// Run with: bazel test //rust/wasm-bindings:wasm_conformance_test
// Or manually: node rust/wasm-bindings/tests/twitter_text_wasm_test.mjs

import assert from "node:assert";
import { test, describe } from "node:test";
import * as fs from "node:fs";
import * as path from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "yaml";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Find the runfiles directory (for Bazel) or use relative paths (for manual runs)
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

// Import the WASM module - use the Node.js target
let wasmPath;
if (runfilesDir) {
  wasmPath = path.join(
    runfilesDir,
    "_main/rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js",
  );
} else {
  wasmPath = path.join(
    __dirname,
    "../../../bazel-bin/rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js",
  );
}

const wasm = await import(wasmPath);

// Load YAML conformance files
function loadYAML(filename) {
  const possiblePaths = [];

  if (runfilesDir) {
    possiblePaths.push(path.join(runfilesDir, "_main/conformance", filename));
  }

  possiblePaths.push(
    path.join(__dirname, "../../../conformance", filename),
    path.join(__dirname, "../../..", filename),
    filename,
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

describe("Extract Conformance Tests", () => {
  const yaml = loadYAML("extract.yml");
  const extractor = new wasm.Extractor();

  describe("mentions", () => {
    const tests = yaml.tests.mentions || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const entities = extractor.extractMentionedScreennames(tc.text);
        const values = Array.from(entities);
        assert.deepStrictEqual(values, tc.expected, tc.description);
      });
    }
  });

  describe("urls", () => {
    const tests = yaml.tests.urls || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const entities = extractor.extractUrls(tc.text);
        const values = Array.from(entities);
        assert.deepStrictEqual(values, tc.expected, tc.description);
      });
    }
  });

  describe("hashtags", () => {
    const tests = yaml.tests.hashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const entities = extractor.extractHashtags(tc.text);
        const values = Array.from(entities);
        assert.deepStrictEqual(values, tc.expected, tc.description);
      });
    }
  });

  describe("cashtags", () => {
    const tests = yaml.tests.cashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const entities = extractor.extractCashtags(tc.text);
        const values = Array.from(entities);
        assert.deepStrictEqual(values, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// Autolink Tests
// ============================================================================

describe("Autolink Conformance Tests", () => {
  const yaml = loadYAML("autolink.yml");

  describe("usernames", () => {
    const tests = yaml.tests.usernames || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const autolinker = new wasm.Autolinker();
        const result = autolinker.autolinkUsernamesAndLists(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        autolinker.free();
      });
    }
  });

  describe("lists", () => {
    const tests = yaml.tests.lists || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const autolinker = new wasm.Autolinker();
        const result = autolinker.autolinkUsernamesAndLists(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        autolinker.free();
      });
    }
  });

  describe("hashtags", () => {
    const tests = yaml.tests.hashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const autolinker = new wasm.Autolinker();
        const result = autolinker.autolinkHashtags(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        autolinker.free();
      });
    }
  });

  describe("urls", () => {
    const tests = yaml.tests.urls || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const autolinker = new wasm.Autolinker();
        const result = autolinker.autolinkUrls(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        autolinker.free();
      });
    }
  });

  describe("cashtags", () => {
    const tests = yaml.tests.cashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const autolinker = new wasm.Autolinker();
        const result = autolinker.autolinkCashtags(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        autolinker.free();
      });
    }
  });

  describe("all", () => {
    const tests = yaml.tests.all || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const autolinker = new wasm.Autolinker();
        const result = autolinker.autolink(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        autolinker.free();
      });
    }
  });
});

// ============================================================================
// Validate Tests
// ============================================================================

describe("Validate Conformance Tests", () => {
  const yaml = loadYAML("validate.yml");

  describe("tweets", () => {
    const tests = yaml.tests.tweets || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const validator = new wasm.Validator();
        const result = validator.isValidTweet(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        validator.free();
      });
    }
  });

  describe("usernames", () => {
    const tests = yaml.tests.usernames || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const validator = new wasm.Validator();
        const result = validator.isValidUsername(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        validator.free();
      });
    }
  });

  describe("lists", () => {
    const tests = yaml.tests.lists || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const validator = new wasm.Validator();
        const result = validator.isValidList(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        validator.free();
      });
    }
  });

  describe("hashtags", () => {
    const tests = yaml.tests.hashtags || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const validator = new wasm.Validator();
        const result = validator.isValidHashtag(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        validator.free();
      });
    }
  });

  describe("urls", () => {
    const tests = yaml.tests.urls || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const validator = new wasm.Validator();
        const result = validator.isValidUrl(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        validator.free();
      });
    }
  });

  describe("urls_without_protocol", () => {
    const tests = yaml.tests.urls_without_protocol || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const validator = new wasm.Validator();
        const result = validator.isValidUrlWithoutProtocol(tc.text);
        assert.strictEqual(result, tc.expected, tc.description);
        validator.free();
      });
    }
  });
});

// ============================================================================
// Hit Highlighting Tests
// ============================================================================

describe("Hit Highlighting Conformance Tests", () => {
  const yaml = loadYAML("hit_highlighting.yml");

  describe("plain_text", () => {
    const tests = yaml.tests.plain_text || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const highlighter = new wasm.HitHighlighter();
        const result = highlighter.highlight(tc.text, tc.hits);
        assert.strictEqual(result, tc.expected, tc.description);
        highlighter.free();
      });
    }
  });

  describe("with_links", () => {
    const tests = yaml.tests.with_links || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const highlighter = new wasm.HitHighlighter();
        const result = highlighter.highlight(tc.text, tc.hits);
        assert.strictEqual(result, tc.expected, tc.description);
        highlighter.free();
      });
    }
  });
});

// ============================================================================
// TLD Tests
// ============================================================================

describe("TLD Conformance Tests", () => {
  const yaml = loadYAML("tlds.yml");
  const extractor = new wasm.Extractor();

  describe("country", () => {
    const tests = yaml.tests.country || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const urls = extractor.extractUrls(tc.text);
        const values = Array.from(urls);
        assert.deepStrictEqual(values, tc.expected, tc.description);
      });
    }
  });

  describe("generic", () => {
    const tests = yaml.tests.generic || [];
    for (const tc of tests) {
      test(tc.description, () => {
        const urls = extractor.extractUrls(tc.text);
        const values = Array.from(urls);
        assert.deepStrictEqual(values, tc.expected, tc.description);
      });
    }
  });
});

// ============================================================================
// Federated Mention Tests
// ============================================================================

describe("Federated Mention Tests", () => {
  const extractor = new wasm.Extractor();

  test("extracts simple federated mention", () => {
    const mentions = extractor.extractFederatedMentions(
      "Hello @user@mastodon.social!",
    );
    const values = Array.from(mentions);
    assert.strictEqual(values.length, 1);
    assert.strictEqual(values[0], "@user@mastodon.social");
  });

  test("extracts federated mentions with indices", () => {
    const entities = extractor.extractFederatedMentionsWithIndices(
      "Hello @user@mastodon.social!",
    );
    const arr = Array.from(entities);
    assert.strictEqual(arr.length, 1);
    assert.strictEqual(arr[0].value, "@user@mastodon.social");
    assert.strictEqual(arr[0].start, 6);
    assert.strictEqual(arr[0].end, 27);
    assert.strictEqual(arr[0].entityType, 4); // FEDERATEDMENTION
    assert.strictEqual(arr[0].isFederatedMention(), true);
  });

  test("extracts multiple federated mentions", () => {
    const mentions = extractor.extractFederatedMentions(
      "@alice@example.com and @bob@other.org",
    );
    const values = Array.from(mentions);
    assert.strictEqual(values.length, 2);
    assert.strictEqual(values[0], "@alice@example.com");
    assert.strictEqual(values[1], "@bob@other.org");
  });

  test("extracts mixed regular and federated mentions", () => {
    const mentions = extractor.extractFederatedMentions(
      "@regular and @federated@domain.com",
    );
    const values = Array.from(mentions);
    assert.strictEqual(values.length, 2);
    // Regular mentions don't include @ prefix
    assert.strictEqual(values[0], "regular");
    assert.strictEqual(values[1], "@federated@domain.com");
  });

  test("extractEntitiesWithIndicesFederated includes federated mentions", () => {
    const entities = extractor.extractEntitiesWithIndicesFederated(
      "Check @user@mastodon.social and https://example.com #hashtag $CASH",
    );
    const arr = Array.from(entities);
    assert.strictEqual(arr.length, 4);

    // Find federated mention
    const federated = arr.find((e) => e.entityType === 4);
    assert.ok(federated, "Should find federated mention");
    assert.strictEqual(federated.value, "@user@mastodon.social");
    assert.strictEqual(federated.isFederatedMention(), true);
  });

  test("extractEntitiesWithIndices excludes federated mentions", () => {
    const entities = extractor.extractEntitiesWithIndices(
      "Check @user@mastodon.social and https://example.com #hashtag $CASH",
    );
    const arr = Array.from(entities);

    // Should NOT find federated mention (entity_type 4)
    const federated = arr.find((e) => e.entityType === 4);
    assert.strictEqual(
      federated,
      undefined,
      "Should not find federated mention in regular entities",
    );
  });
});

// ============================================================================
// Configuration Tests
// ============================================================================

describe("Configuration Tests", () => {
  test("configV1 returns valid configuration", () => {
    const config = wasm.TwitterTextConfiguration.configV1();
    assert.strictEqual(config.version, 1);
    assert.strictEqual(config.maxWeightedTweetLength, 140);
    assert.strictEqual(config.emojiParsingEnabled, false);
    config.free();
  });

  test("configV2 returns valid configuration", () => {
    const config = wasm.TwitterTextConfiguration.configV2();
    assert.strictEqual(config.version, 2);
    assert.strictEqual(config.maxWeightedTweetLength, 280);
    assert.strictEqual(config.emojiParsingEnabled, false);
    config.free();
  });

  test("configV3 returns valid configuration", () => {
    const config = wasm.TwitterTextConfiguration.configV3();
    assert.strictEqual(config.version, 3);
    assert.strictEqual(config.maxWeightedTweetLength, 280);
    assert.strictEqual(config.emojiParsingEnabled, true);
    config.free();
  });
});

// ============================================================================
// Tweet Parser Tests
// ============================================================================

describe("Tweet Parser Tests", () => {
  test("parseTweet parses simple text", () => {
    const result = wasm.parseTweet("Hello, world!");
    assert.strictEqual(result.isValid, true);
    assert.strictEqual(result.weightedLength, 13);
    result.free();
  });

  test("emoji counting with v2 config", () => {
    const config = wasm.TwitterTextConfiguration.configV2();
    const parser = new wasm.TwitterTextParser(config);
    // Text: "H" + cat emoji + smiley + family emoji
    const text =
      "H\u{1F431}\u263A\u{1F468}\u200D\u{1F469}\u200D\u{1F467}\u200D\u{1F466}";
    const result = parser.parseTweet(text);
    assert.strictEqual(result.weightedLength, 16);
    assert.strictEqual(result.isValid, true);
    config.free();
    parser.free();
    result.free();
  });

  test("emoji counting with v3 config", () => {
    const config = wasm.TwitterTextConfiguration.configV3();
    const parser = new wasm.TwitterTextParser(config);
    // Text: "H" + cat emoji + smiley + family emoji
    const text =
      "H\u{1F431}\u263A\u{1F468}\u200D\u{1F469}\u200D\u{1F467}\u200D\u{1F466}";
    const result = parser.parseTweet(text);
    assert.strictEqual(result.weightedLength, 7);
    assert.strictEqual(result.isValid, true);
    config.free();
    parser.free();
    result.free();
  });
});

console.log("\nTwitter Text WASM Conformance Tests");
console.log("====================================\n");
