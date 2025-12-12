// Quick test for extract performance breakdown
import { readFileSync } from "fs";
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

let oldTwitterText, rustTwitterText;
if (runfilesDir) {
  oldTwitterText = require(join(runfilesDir, "_main/js/pkg/twitter-text-wrapper.cjs"));
  rustTwitterText = (await import(join(runfilesDir, "_main/rust/wasm-bindings/old-js/twitter-text.mjs"))).default;
} else {
  oldTwitterText = require("../../js/pkg/twitter-text-wrapper.cjs");
  rustTwitterText = (await import("../../rust/wasm-bindings/old-js/twitter-text.mjs")).default;
}

const text = "Thanks @elonmusk for the update about #AI and $TSLA at https://tesla.com";

// Warmup
for (let i = 0; i < 100; i++) {
  oldTwitterText.extractMentions(text);
  rustTwitterText.extractMentions(text);
}

const ITERATIONS = 1000;

function benchmark(name, oldFn, rustFn) {
  let start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) oldFn();
  let oldTime = performance.now() - start;

  start = performance.now();
  for (let i = 0; i < ITERATIONS; i++) rustFn();
  let rustTime = performance.now() - start;

  const oldOps = (ITERATIONS/oldTime*1000).toFixed(0);
  const rustOps = (ITERATIONS/rustTime*1000).toFixed(0);
  const ratio = (oldTime > rustTime) ? (rustTime/oldTime).toFixed(1) : (oldTime/rustTime).toFixed(1);
  const label = (oldTime > rustTime) ? "faster" : "slower";
  console.log(`${name}: Old ${oldOps} ops/sec, Rust ${rustOps} ops/sec, Rust is ${ratio}x ${label}`);
}

benchmark("extractMentions",
  () => oldTwitterText.extractMentions(text),
  () => rustTwitterText.extractMentions(text));

benchmark("extractUrls",
  () => oldTwitterText.extractUrls(text),
  () => rustTwitterText.extractUrls(text));

benchmark("extractHashtags",
  () => oldTwitterText.extractHashtags(text),
  () => rustTwitterText.extractHashtags(text));

benchmark("extractCashtags",
  () => oldTwitterText.extractCashtags(text),
  () => rustTwitterText.extractCashtags(text));

console.log("\nDone.");
