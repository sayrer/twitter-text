// Test string size impact on WASM performance
import { dirname, join } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));

function findRunfilesDir() {
  if (process.env.RUNFILES_DIR) return process.env.RUNFILES_DIR;
  if (process.env.JS_BINARY__RUNFILES) return process.env.JS_BINARY__RUNFILES;
  return null;
}

const runfilesDir = findRunfilesDir();

let wasmModule;
if (runfilesDir) {
  wasmModule = await import(join(runfilesDir, "_main/rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js"));
} else {
  wasmModule = await import("../../rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js");
}

const extractor = new wasmModule.Extractor();

const shortText = "@a";
const mediumText = "Thanks @elonmusk for the update about #AI";
const longText = "A".repeat(280) + " @user";

const ITERATIONS = 1000;

function benchmark(name, text) {
  // Warmup
  for (let i = 0; i < 100; i++) {
    extractor.extractMentionedScreennames(text);
  }

  let start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    extractor.extractMentionedScreennames(text);
  }
  let time = performance.now() - start;
  console.log(`${name} (${text.length} chars): ${(ITERATIONS/time*1000).toFixed(0)} ops/sec`);
}

benchmark("Short", shortText);
benchmark("Medium", mediumText);
benchmark("Long", longText);

// Also test parseTweet which is a different code path
console.log("\nparseTweet:");
const config = wasmModule.TwitterTextConfiguration.configV3();

function benchmarkParse(name, text) {
  // Warmup
  for (let i = 0; i < 100; i++) {
    wasmModule.parseTweetWithConfig(text, config);
  }

  let start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) {
    wasmModule.parseTweetWithConfig(text, config);
  }
  let time = performance.now() - start;
  console.log(`${name} (${text.length} chars): ${(ITERATIONS/time*1000).toFixed(0)} ops/sec`);
}

benchmarkParse("Short", "Hi");
benchmarkParse("Medium", mediumText);
benchmarkParse("Long", longText);

console.log("\nDone.");
