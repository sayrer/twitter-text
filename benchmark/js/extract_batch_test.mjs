// Test boundary crossing overhead by batching multiple tweets per WASM call
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

// Load raw WASM module directly
let wasmModule;
if (runfilesDir) {
  wasmModule = await import(join(runfilesDir, "_main/rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js"));
} else {
  wasmModule = await import("../../rust/wasm-bindings/twitter_text_wasm_nodejs/twitter_text_wasm_nodejs.js");
}

// Load old JS for comparison
let oldTwitterText;
if (runfilesDir) {
  oldTwitterText = require(join(runfilesDir, "_main/js/pkg/twitter-text-wrapper.cjs"));
} else {
  oldTwitterText = require("../../js/pkg/twitter-text-wrapper.cjs");
}

// Sample tweets with URLs
const sampleTexts = [
  "Check out https://github.com/anthropics/claude for AI",
  "News at https://twitter.com about #tech",
  "Visit https://example.com/path?query=1 for details",
  "Link: https://t.co/abc123 and https://t.co/xyz789",
  "Multiple URLs: https://a.com https://b.com https://c.com",
  "No URL in this tweet about #coding",
  "Another link https://docs.rust-lang.org/book/",
  "Blog post: https://blog.example.org/2024/post",
  "API docs https://api.example.com/v1/docs",
  "Join us at https://meetup.com/rust-users",
];

const extractor = new wasmModule.Extractor();

const TOTAL_TWEETS = 1000;
const WARMUP = 100;

console.log("Boundary Crossing Overhead Test");
console.log("================================");
console.log(`Total tweets processed per test: ${TOTAL_TWEETS}`);
console.log("");

// Warmup
for (let i = 0; i < WARMUP; i++) {
  extractor.extractUrls(sampleTexts[i % sampleTexts.length]);
  oldTwitterText.extractUrls(sampleTexts[i % sampleTexts.length]);
}

// Test 1: Old JS - 1 tweet per call (baseline)
let start = performance.now();
for (let i = 0; i < TOTAL_TWEETS; i++) {
  oldTwitterText.extractUrls(sampleTexts[i % sampleTexts.length]);
}
let oldTime = performance.now() - start;
console.log(`Old JS (1 tweet/call × ${TOTAL_TWEETS}): ${(TOTAL_TWEETS/oldTime*1000).toFixed(0)} tweets/sec`);

// Test 2: Rust WASM - 1 tweet per call (current approach)
start = performance.now();
for (let i = 0; i < TOTAL_TWEETS; i++) {
  extractor.extractUrls(sampleTexts[i % sampleTexts.length]);
}
let rustSingleTime = performance.now() - start;
console.log(`Rust WASM (1 tweet/call × ${TOTAL_TWEETS}): ${(TOTAL_TWEETS/rustSingleTime*1000).toFixed(0)} tweets/sec`);

// Test 3: Rust WASM - 10 tweets per call
const batch10 = [];
for (let i = 0; i < 10; i++) {
  batch10.push(sampleTexts[i % sampleTexts.length]);
}
start = performance.now();
for (let i = 0; i < TOTAL_TWEETS / 10; i++) {
  extractor.extractUrlsBatch(batch10);
}
let rustBatch10Time = performance.now() - start;
console.log(`Rust WASM (10 tweets/call × ${TOTAL_TWEETS/10}): ${(TOTAL_TWEETS/rustBatch10Time*1000).toFixed(0)} tweets/sec`);

// Test 4: Rust WASM - 100 tweets per call
const batch100 = [];
for (let i = 0; i < 100; i++) {
  batch100.push(sampleTexts[i % sampleTexts.length]);
}
start = performance.now();
for (let i = 0; i < TOTAL_TWEETS / 100; i++) {
  extractor.extractUrlsBatch(batch100);
}
let rustBatch100Time = performance.now() - start;
console.log(`Rust WASM (100 tweets/call × ${TOTAL_TWEETS/100}): ${(TOTAL_TWEETS/rustBatch100Time*1000).toFixed(0)} tweets/sec`);

console.log("");
console.log("Analysis:");
console.log(`  Rust 1/call vs Old JS: ${(rustSingleTime/oldTime).toFixed(2)}x ${rustSingleTime > oldTime ? "slower" : "faster"}`);
console.log(`  Rust 10/call vs 1/call: ${(rustSingleTime/rustBatch10Time).toFixed(2)}x ${rustBatch10Time < rustSingleTime ? "faster" : "slower"}`);
console.log(`  Rust 100/call vs 1/call: ${(rustSingleTime/rustBatch100Time).toFixed(2)}x ${rustBatch100Time < rustSingleTime ? "faster" : "slower"}`);
console.log(`  Rust 100/call vs Old JS: ${(rustBatch100Time/oldTime).toFixed(2)}x ${rustBatch100Time > oldTime ? "slower" : "faster"}`);

console.log("\nDone.");
