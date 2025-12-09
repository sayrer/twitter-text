// Test where the overhead is: WASM call vs Array.from vs JS wrapper
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

const text = "Thanks @elonmusk for the update about #AI and $TSLA at https://tesla.com";

// Create extractor directly
const extractor = new wasmModule.Extractor();

// Warmup
for (let i = 0; i < 100; i++) {
  extractor.extractMentionedScreennames(text);
}

const ITERATIONS = 1000;

// Test 1: Raw WASM call (returns js_sys::Array)
let start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  extractor.extractMentionedScreennames(text);
}
let time1 = performance.now() - start;
console.log(`Raw WASM extractMentions: ${(ITERATIONS/time1*1000).toFixed(0)} ops/sec`);

// Test 2: WASM call + Array.from
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  Array.from(extractor.extractMentionedScreennames(text));
}
let time2 = performance.now() - start;
console.log(`WASM + Array.from: ${(ITERATIONS/time2*1000).toFixed(0)} ops/sec`);

// Test 3: Just array iteration
const arr = extractor.extractMentionedScreennames(text);
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  Array.from(arr);
}
let time3 = performance.now() - start;
console.log(`Just Array.from on cached result: ${(ITERATIONS/time3*1000).toFixed(0)} ops/sec`);

// Test 4: extractUrls (more complex)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  extractor.extractUrls(text);
}
let time4 = performance.now() - start;
console.log(`Raw WASM extractUrls: ${(ITERATIONS/time4*1000).toFixed(0)} ops/sec`);

// Test 5: extractEntitiesWithIndices (returns Entity objects)
start = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  extractor.extractEntitiesWithIndices(text);
}
let time5 = performance.now() - start;
console.log(`Raw WASM extractEntitiesWithIndices: ${(ITERATIONS/time5*1000).toFixed(0)} ops/sec`);

console.log("\nDone.");
