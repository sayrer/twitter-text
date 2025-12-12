// Compare extract performance on short vs long tweets
import { readFileSync } from "fs";
import { parse } from "yaml";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

function findRunfilesDir() {
  if (process.env.RUNFILES_DIR) return process.env.RUNFILES_DIR;
  if (process.env.JS_BINARY__RUNFILES) return process.env.JS_BINARY__RUNFILES;
  return null;
}

const runfilesDir = findRunfilesDir();

// Load implementations
let oldTwitterText, rustTwitterText;
if (runfilesDir) {
  oldTwitterText = require(
    join(runfilesDir, "_main/js/pkg/twitter-text-wrapper.cjs"),
  );
  rustTwitterText = (
    await import(
      join(runfilesDir, "_main/rust/wasm-bindings/old-js/twitter-text.mjs")
    )
  ).default;
} else {
  oldTwitterText = require("../../js/pkg/twitter-text-wrapper.cjs");
  rustTwitterText = (
    await import("../../rust/wasm-bindings/old-js/twitter-text.mjs")
  ).default;
}

function loadYaml(filename) {
  let path;
  if (runfilesDir) {
    path = join(runfilesDir, "_main/benchmark/data", filename);
  } else {
    path = join(__dirname, "..", "data", filename);
  }
  return parse(readFileSync(path, "utf8"));
}

const ITERATIONS = 1000;
const WARMUP = 100;

function benchmark(name, texts, oldFn, rustFn) {
  // Warmup
  for (let i = 0; i < WARMUP; i++) {
    for (const text of texts) {
      oldFn(text);
      rustFn(text);
    }
  }

  // Benchmark old
  let start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    for (const text of texts) {
      oldFn(text);
    }
  }
  const oldTime = performance.now() - start;
  const oldOps = ((ITERATIONS / oldTime) * 1000).toFixed(0);

  // Benchmark rust
  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    for (const text of texts) {
      rustFn(text);
    }
  }
  const rustTime = performance.now() - start;
  const rustOps = ((ITERATIONS / rustTime) * 1000).toFixed(0);

  const ratio =
    oldTime < rustTime
      ? `Old ${(rustTime / oldTime).toFixed(1)}x faster`
      : `Rust ${(oldTime / rustTime).toFixed(1)}x faster`;

  console.log(
    `${name}: Old ${oldOps} ops/sec, Rust ${rustOps} ops/sec (${ratio})`,
  );
}

function runExtractBenchmark(name, data) {
  const allTexts = [
    ...data.tests.mentions.map((t) => t.text),
    ...data.tests.urls.map((t) => t.text),
    ...data.tests.hashtags.map((t) => t.text),
    ...data.tests.cashtags.map((t) => t.text),
  ];

  const avgLen =
    allTexts.reduce((sum, t) => sum + t.length, 0) / allTexts.length;
  console.log(
    `\n${name} (${allTexts.length} texts, avg ${avgLen.toFixed(0)} chars):`,
  );
  console.log("-".repeat(60));

  benchmark(
    "extractMentions",
    allTexts,
    (t) => oldTwitterText.extractMentions(t),
    (t) => rustTwitterText.extractMentions(t),
  );

  benchmark(
    "extractUrls",
    allTexts,
    (t) => oldTwitterText.extractUrls(t),
    (t) => rustTwitterText.extractUrls(t),
  );

  benchmark(
    "extractHashtags",
    allTexts,
    (t) => oldTwitterText.extractHashtags(t),
    (t) => rustTwitterText.extractHashtags(t),
  );

  benchmark(
    "extractCashtags",
    allTexts,
    (t) => oldTwitterText.extractCashtags(t),
    (t) => rustTwitterText.extractCashtags(t),
  );
}

console.log("Extract Performance: Short vs Long Tweets");
console.log("==========================================");

const shortData = loadYaml("extract.yml");
const longData = loadYaml("extract_long.yml");

runExtractBenchmark("SHORT TWEETS", shortData);
runExtractBenchmark("LONG TWEETS", longData);

console.log("\nDone.");
