// Standalone benchmark script - run directly with system node
// Usage: node benchmark/js/profile_standalone.mjs
// For profiling: node --prof benchmark/js/profile_standalone.mjs
import { readFileSync } from "fs";
import { parse } from "yaml";
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

// Load Rust WASM directly from bazel-bin
const wasmPath = join(
  __dirname,
  "../../bazel-bin/rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js",
);
const wasmModule = await import(wasmPath);

function loadYaml(filename) {
  const path = join(__dirname, "../..", filename);
  return parse(readFileSync(path, "utf8"));
}

// Use the full conformance test suite as base
const extractData = loadYaml("conformance/extract.yml");
const baseTexts = [
  ...extractData.tests.mentions.map((t) => t.text),
  ...extractData.tests.mentions_with_indices.map((t) => t.text),
  ...extractData.tests.mentions_or_lists_with_indices.map((t) => t.text),
  ...extractData.tests.replies.map((t) => t.text),
  ...extractData.tests.urls.map((t) => t.text),
  ...extractData.tests.urls_with_indices.map((t) => t.text),
  ...extractData.tests.urls_with_directional_markers.map((t) => t.text),
  ...extractData.tests.hashtags.map((t) => t.text),
  ...extractData.tests.hashtags_from_astral.map((t) => t.text),
  ...extractData.tests.hashtags_with_indices.map((t) => t.text),
  ...extractData.tests.cashtags.map((t) => t.text),
  ...extractData.tests.cashtags_with_indices.map((t) => t.text),
];

// Generate a large corpus of long texts (~2000 chars each)
// With ~3 entities per 200 characters (so ~30 entities per text)
const CORPUS_SIZE = 150000;
const allTexts = [];

// Filler text segments (no entities)
const fillers = [
  "This is some regular text without any special entities just plain words. ",
  "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod. ",
  "The quick brown fox jumps over the lazy dog multiple times in a row. ",
  "Here we have more filler content to pad out the tweet to proper length. ",
  "Random thoughts and musings that don't contain any special characters. ",
  "Just normal everyday text that needs to be parsed but has no entities. ",
  "More padding text here to reach the target length of about 2000 chars. ",
  "Writing sample text is harder than it seems when you need lots of it. ",
];

// Entity templates
const mentions = ["@user", "@elonmusk", "@TwitterAPI", "@github", "@rustlang"];
const hashtags = ["#tech", "#coding", "#AI", "#startup", "#news", "#trending"];
const urls = [
  "https://example.com",
  "https://t.co/abc123",
  "https://github.com/test",
];
const cashtags = ["$AAPL", "$TSLA", "$GOOGL", "$BTC"];

function generateLongText(seed) {
  let text = "";
  const targetLength = 2000;
  const entitiesPerSegment = 3; // 3 entities per ~200 chars

  while (text.length < targetLength) {
    // Add filler (about 150 chars)
    text += fillers[seed % fillers.length];
    seed = (seed * 31 + 17) >>> 0; // Simple PRNG

    // Add 3 entities
    for (let e = 0; e < entitiesPerSegment && text.length < targetLength; e++) {
      const entityType = (seed + e) % 4;
      switch (entityType) {
        case 0:
          text += mentions[(seed + e) % mentions.length] + " ";
          break;
        case 1:
          text += hashtags[(seed + e) % hashtags.length] + " ";
          break;
        case 2:
          text += urls[(seed + e) % urls.length] + " ";
          break;
        case 3:
          text += cashtags[(seed + e) % cashtags.length] + " ";
          break;
      }
    }
    seed = (seed * 31 + 17) >>> 0;
  }

  // Add unique suffix to prevent memoization
  return text.substring(0, targetLength - 10) + ` #id${seed}`;
}

for (let i = 0; i < CORPUS_SIZE; i++) {
  allTexts.push(generateLongText(i));
}

// Verify average length
const avgLen = allTexts.reduce((sum, t) => sum + t.length, 0) / allTexts.length;
console.log(`Average text length: ${avgLen.toFixed(0)} chars`);

console.log(
  `Profiling Rust WASM extract on ${allTexts.length.toLocaleString()} unique tweets`,
);
console.log("=".repeat(70));

const extractor = new wasmModule.Extractor();

// Rust-only profiling
let start = performance.now();
for (const text of allTexts) {
  extractor.extractMentionedScreennames(text);
  extractor.extractUrls(text);
  extractor.extractHashtags(text);
  extractor.extractCashtags(text);
}
const elapsed = performance.now() - start;

const tps = ((allTexts.length / elapsed) * 1000).toFixed(0);
console.log(`Rust WASM: ${Number(tps).toLocaleString()} tweets/sec`);
console.log(`Total time: ${(elapsed / 1000).toFixed(2)}s`);

console.log("\nDone.");
