// Benchmark: Old JavaScript implementation vs Rust WASM bindings
import { readFileSync } from "fs";
import { parse } from "yaml";
import { dirname, join } from "path";
import { fileURLToPath } from "url";
import { createRequire } from "module";

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

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

// Load the old JavaScript implementation (via CommonJS wrapper)
let oldTwitterText;
if (runfilesDir) {
  const oldPath = join(runfilesDir, "_main/js/pkg/twitter-text-wrapper.cjs");
  oldTwitterText = require(oldPath);
} else {
  oldTwitterText = require("../../js/pkg/twitter-text-wrapper.cjs");
}

// Load the Rust WASM implementation (via old-js wrapper for API compatibility)
let rustTwitterText;
if (runfilesDir) {
  const wrapperPath = join(
    runfilesDir,
    "_main/rust/wasm-bindings/old-js/twitter-text.mjs",
  );
  rustTwitterText = (await import(wrapperPath)).default;
} else {
  rustTwitterText = (
    await import("../../rust/wasm-bindings/old-js/twitter-text.mjs")
  ).default;
}

const ITERATIONS = 1000;
const WARMUP_ITERATIONS = 100;

// Global sink to prevent escape analysis from optimizing away results
// We accumulate values and print at the end to ensure work isn't eliminated
let sink = 0;

function loadYaml(filename) {
  let path;
  if (runfilesDir) {
    path = join(runfilesDir, "_main/benchmark/data", filename);
  } else {
    path = join(__dirname, "..", "data", filename);
  }
  const content = readFileSync(path, "utf8");
  return parse(content);
}

function arraysEqual(a, b) {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

function runBenchmark(name, fn, iterations) {
  // Warmup - also accumulate to prevent optimization
  for (let i = 0; i < WARMUP_ITERATIONS; i++) {
    sink += fn();
  }

  const start = performance.now();
  for (let i = 0; i < iterations; i++) {
    sink += fn();
  }
  const elapsed = performance.now() - start;
  const opsPerSec = (iterations / elapsed) * 1000;
  return { elapsed, opsPerSec };
}

function formatNumber(n) {
  return n.toLocaleString("en-US", { maximumFractionDigits: 0 });
}

function printResults(operation, oldResult, rustResult) {
  const speedup = rustResult.opsPerSec / oldResult.opsPerSec;
  const label = speedup >= 1 ? "faster" : "slower";
  const ratio = speedup >= 1 ? speedup : 1 / speedup;
  console.log(`\n${operation} (${ITERATIONS} iterations):`);
  console.log(`  Old JS:    ${formatNumber(oldResult.opsPerSec)} ops/sec`);
  console.log(`  Rust WASM: ${formatNumber(rustResult.opsPerSec)} ops/sec`);
  console.log(`  Result:    ${ratio.toFixed(1)}x ${label}`);
}

// Autolink benchmark
function benchmarkAutolink() {
  const data = loadYaml("autolink.yml");
  const tests = data.tests;

  // Correctness check
  let errors = 0;
  for (const test of tests) {
    const oldResult = oldTwitterText.autoLink(test.text);
    const rustResult = rustTwitterText.autoLink(test.text);
    if (oldResult !== rustResult) {
      errors++;
      if (errors <= 3) {
        console.log(`  [MISMATCH] Autolink "${test.text.substring(0, 40)}..."`);
        console.log(`    Old:  ${oldResult.substring(0, 80)}...`);
        console.log(`    Rust: ${rustResult.substring(0, 80)}...`);
      }
    }
  }
  if (errors > 0) {
    console.log(`  [WARNING] ${errors}/${tests.length} autolink mismatches`);
  }

  // Return accumulated length to thwart escape analysis
  const oldFn = () => {
    let acc = 0;
    for (const test of tests) {
      acc += oldTwitterText.autoLink(test.text).length;
    }
    return acc;
  };

  const rustFn = () => {
    let acc = 0;
    for (const test of tests) {
      acc += rustTwitterText.autoLink(test.text).length;
    }
    return acc;
  };

  const oldResult = runBenchmark("Autolink (old)", oldFn, ITERATIONS);
  const rustResult = runBenchmark("Autolink (rust)", rustFn, ITERATIONS);
  printResults("Autolink", oldResult, rustResult);
}

// Extract benchmark
function benchmarkExtract() {
  const data = loadYaml("extract.yml");

  const mentionTexts = data.tests.mentions.map((t) => t.text);
  const urlTexts = data.tests.urls.map((t) => t.text);
  const hashtagTexts = data.tests.hashtags.map((t) => t.text);
  const cashtagTexts = data.tests.cashtags.map((t) => t.text);

  const allTexts = [
    ...mentionTexts,
    ...urlTexts,
    ...hashtagTexts,
    ...cashtagTexts,
  ];

  // Correctness check
  let errors = 0;
  for (const text of allTexts) {
    const oldMentions = oldTwitterText.extractMentions(text);
    const rustMentions = rustTwitterText.extractMentions(text);
    if (!arraysEqual(oldMentions, rustMentions)) {
      errors++;
      if (errors <= 3) {
        console.log(
          `  [MISMATCH] extractMentions "${text.substring(0, 40)}..."`,
        );
        console.log(`    Old:  ${JSON.stringify(oldMentions)}`);
        console.log(`    Rust: ${JSON.stringify(rustMentions)}`);
      }
    }

    const oldUrls = oldTwitterText.extractUrls(text);
    const rustUrls = rustTwitterText.extractUrls(text);
    if (!arraysEqual(oldUrls, rustUrls)) {
      errors++;
      if (errors <= 3) {
        console.log(`  [MISMATCH] extractUrls "${text.substring(0, 40)}..."`);
        console.log(`    Old:  ${JSON.stringify(oldUrls)}`);
        console.log(`    Rust: ${JSON.stringify(rustUrls)}`);
      }
    }

    const oldHashtags = oldTwitterText.extractHashtags(text);
    const rustHashtags = rustTwitterText.extractHashtags(text);
    if (!arraysEqual(oldHashtags, rustHashtags)) {
      errors++;
      if (errors <= 3) {
        console.log(
          `  [MISMATCH] extractHashtags "${text.substring(0, 40)}..."`,
        );
        console.log(`    Old:  ${JSON.stringify(oldHashtags)}`);
        console.log(`    Rust: ${JSON.stringify(rustHashtags)}`);
      }
    }

    const oldCashtags = oldTwitterText.extractCashtags(text);
    const rustCashtags = rustTwitterText.extractCashtags(text);
    if (!arraysEqual(oldCashtags, rustCashtags)) {
      errors++;
      if (errors <= 3) {
        console.log(
          `  [MISMATCH] extractCashtags "${text.substring(0, 40)}..."`,
        );
        console.log(`    Old:  ${JSON.stringify(oldCashtags)}`);
        console.log(`    Rust: ${JSON.stringify(rustCashtags)}`);
      }
    }
  }
  if (errors > 0) {
    console.log(
      `  [WARNING] ${errors} extract mismatches across ${allTexts.length} texts`,
    );
  }

  // Return accumulated array lengths to thwart escape analysis
  const oldFn = () => {
    let acc = 0;
    for (const text of allTexts) {
      acc += oldTwitterText.extractMentions(text).length;
      acc += oldTwitterText.extractUrls(text).length;
      acc += oldTwitterText.extractHashtags(text).length;
      acc += oldTwitterText.extractCashtags(text).length;
    }
    return acc;
  };

  const rustFn = () => {
    let acc = 0;
    for (const text of allTexts) {
      acc += rustTwitterText.extractMentions(text).length;
      acc += rustTwitterText.extractUrls(text).length;
      acc += rustTwitterText.extractHashtags(text).length;
      acc += rustTwitterText.extractCashtags(text).length;
    }
    return acc;
  };

  const oldResult = runBenchmark("Extract (old)", oldFn, ITERATIONS);
  const rustResult = runBenchmark("Extract (rust)", rustFn, ITERATIONS);
  printResults("Extract", oldResult, rustResult);
}

// Validate Tweet benchmark (tweets only - comparable across all implementations)
function benchmarkValidateTweet() {
  const data = loadYaml("validate.yml");

  const tweetTexts = data.tests.tweets.map((t) => t.text);

  // Correctness check
  let errors = 0;
  for (const text of tweetTexts) {
    const oldResult = oldTwitterText.isValidTweetText(text);
    const rustResult = rustTwitterText.isValidTweetText(text);
    if (oldResult !== rustResult) {
      errors++;
      if (errors <= 3) {
        console.log(
          `  [MISMATCH] isValidTweetText "${text.substring(0, 40)}..."`,
        );
        console.log(`    Old:  ${oldResult}`);
        console.log(`    Rust: ${rustResult}`);
      }
    }
  }
  if (errors > 0) {
    console.log(`  [WARNING] ${errors} validate tweet mismatches`);
  }

  // Return count of valid results to thwart escape analysis
  const oldFn = () => {
    let acc = 0;
    for (const text of tweetTexts) {
      acc += oldTwitterText.isValidTweetText(text) ? 1 : 0;
    }
    return acc;
  };

  const rustFn = () => {
    let acc = 0;
    for (const text of tweetTexts) {
      acc += rustTwitterText.isValidTweetText(text) ? 1 : 0;
    }
    return acc;
  };

  const oldResult = runBenchmark("Validate Tweet (old)", oldFn, ITERATIONS);
  const rustResult = runBenchmark("Validate Tweet (rust)", rustFn, ITERATIONS);
  printResults("Validate Tweet", oldResult, rustResult);
}

// Validate All benchmark (tweets, usernames, hashtags, urls)
function benchmarkValidateAll() {
  const data = loadYaml("validate.yml");

  const tweetTexts = data.tests.tweets.map((t) => t.text);
  const usernameTexts = data.tests.usernames.map((t) => t.text);
  const hashtagTexts = data.tests.hashtags.map((t) => t.text);
  const urlTexts = data.tests.urls.map((t) => t.text);

  // Correctness check
  let errors = 0;
  for (const text of usernameTexts) {
    const oldResult = oldTwitterText.isValidUsername(text);
    const rustResult = rustTwitterText.isValidUsername(text);
    if (oldResult !== rustResult) {
      errors++;
      if (errors <= 3) {
        console.log(`  [MISMATCH] isValidUsername "${text}"`);
        console.log(`    Old:  ${oldResult}`);
        console.log(`    Rust: ${rustResult}`);
      }
    }
  }
  if (errors > 0) {
    console.log(`  [WARNING] ${errors} validate all mismatches`);
  }

  // Return count of valid results to thwart escape analysis
  const oldFn = () => {
    let acc = 0;
    for (const text of tweetTexts) {
      acc += oldTwitterText.isValidTweetText(text) ? 1 : 0;
    }
    for (const text of usernameTexts) {
      acc += oldTwitterText.isValidUsername(text) ? 1 : 0;
    }
    for (const text of hashtagTexts) {
      acc += oldTwitterText.isValidHashtag(text) ? 1 : 0;
    }
    for (const text of urlTexts) {
      acc += oldTwitterText.isValidUrl(text) ? 1 : 0;
    }
    return acc;
  };

  const rustFn = () => {
    let acc = 0;
    for (const text of tweetTexts) {
      acc += rustTwitterText.isValidTweetText(text) ? 1 : 0;
    }
    for (const text of usernameTexts) {
      acc += rustTwitterText.isValidUsername(text) ? 1 : 0;
    }
    for (const text of hashtagTexts) {
      acc += rustTwitterText.isValidHashtag(text) ? 1 : 0;
    }
    for (const text of urlTexts) {
      acc += rustTwitterText.isValidUrl(text) ? 1 : 0;
    }
    return acc;
  };

  const oldResult = runBenchmark("Validate All (old)", oldFn, ITERATIONS);
  const rustResult = runBenchmark("Validate All (rust)", rustFn, ITERATIONS);
  printResults("Validate All", oldResult, rustResult);
}

// Parse tweet benchmark
function benchmarkParse() {
  const data = loadYaml("parse.yml");
  const tests = data.tests;

  // Correctness check
  // Note: Old JS returns "valid", Rust wrapper returns "isValid"
  let errors = 0;
  for (const test of tests) {
    const oldResult = oldTwitterText.parseTweet(test.text);
    const rustResult = rustTwitterText.parseTweet(test.text);
    const oldValid = oldResult.valid; // Old JS uses "valid"
    const rustValid = rustResult.isValid; // Rust wrapper uses "isValid"
    if (
      oldResult.weightedLength !== rustResult.weightedLength ||
      oldValid !== rustValid
    ) {
      errors++;
      if (errors <= 3) {
        console.log(
          `  [MISMATCH] parseTweet "${test.text.substring(0, 40)}..."`,
        );
        console.log(
          `    Old:  weightedLength=${oldResult.weightedLength}, valid=${oldValid}`,
        );
        console.log(
          `    Rust: weightedLength=${rustResult.weightedLength}, isValid=${rustValid}`,
        );
      }
    }
  }
  if (errors > 0) {
    console.log(`  [WARNING] ${errors}/${tests.length} parse mismatches`);
  }

  // Return accumulated weightedLength to thwart escape analysis
  const oldFn = () => {
    let acc = 0;
    for (const test of tests) {
      acc += oldTwitterText.parseTweet(test.text).weightedLength;
    }
    return acc;
  };

  const rustFn = () => {
    let acc = 0;
    for (const test of tests) {
      acc += rustTwitterText.parseTweet(test.text).weightedLength;
    }
    return acc;
  };

  const oldResult = runBenchmark("Parse (old)", oldFn, ITERATIONS);
  const rustResult = runBenchmark("Parse (rust)", rustFn, ITERATIONS);
  printResults("Parse Tweet", oldResult, rustResult);
}

// Main
console.log("Twitter Text Benchmark: Old JavaScript vs Rust WASM");
console.log("====================================================");

benchmarkAutolink();
benchmarkExtract();
benchmarkValidateTweet();
benchmarkValidateAll();
benchmarkParse();

// Print sink value to ensure it's not optimized away
console.log(`\nDone. (checksum: ${sink})`);
