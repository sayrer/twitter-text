// Profile extract on long tweets - run with node --cpu-prof
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

// Load Rust WASM implementation
let rustTwitterText;
if (runfilesDir) {
  rustTwitterText = (await import(join(runfilesDir, "_main/rust/wasm-bindings/old-js/twitter-text.mjs"))).default;
} else {
  rustTwitterText = (await import("../../rust/wasm-bindings/old-js/twitter-text.mjs")).default;
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

const ITERATIONS = 10000;

const longData = loadYaml("extract_long.yml");
const allTexts = [
  ...longData.tests.mentions.map(t => t.text),
  ...longData.tests.urls.map(t => t.text),
  ...longData.tests.hashtags.map(t => t.text),
  ...longData.tests.cashtags.map(t => t.text),
];

console.log(`Profiling Rust WASM extract on ${allTexts.length} long tweets, ${ITERATIONS} iterations...`);

// Warmup
for (let i = 0; i < 100; i++) {
  for (const text of allTexts) {
    rustTwitterText.extractMentions(text);
    rustTwitterText.extractUrls(text);
    rustTwitterText.extractHashtags(text);
    rustTwitterText.extractCashtags(text);
  }
}

console.log("Warmup done, starting profiled run...");

const start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  for (const text of allTexts) {
    rustTwitterText.extractMentions(text);
    rustTwitterText.extractUrls(text);
    rustTwitterText.extractHashtags(text);
    rustTwitterText.extractCashtags(text);
  }
}
const elapsed = performance.now() - start;

console.log(`Done. ${ITERATIONS} iterations in ${(elapsed/1000).toFixed(2)}s`);
console.log(`${(ITERATIONS / elapsed * 1000).toFixed(0)} ops/sec`);
