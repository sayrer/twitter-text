# Sanitizer Tests

This directory contains AddressSanitizer (ASan) and LeakSanitizer (LSan) enabled test targets for detecting memory issues.

## The Challenge

The Bazel LLVM toolchain (version 16.0.0) used by this project **does not include sanitizer runtime libraries**. This means the sanitizer tests are built with `-fsanitize=address` but cannot find the required `libclang_rt.asan_*.dylib` (macOS) or `libclang_rt.asan-*.so` (Linux) at runtime.

## Solutions

### Option 1: Use Regular Tests (Recommended for CI)

The regular tests in `//rust/cpp-bindings:*` work fine. For most purposes, these are sufficient. The sanitizer tests are primarily useful for local development when hunting memory issues.

```bash
bazel test //rust/cpp-bindings:all
```

### Option 2: Run Tests Directly with System Sanitizer Runtime

Build the test, then run it directly with the library path set:

**macOS:**
```bash
# Build the test
bazel build //rust/cpp-sanitizer-bindings:config_test_asan

# Run directly with library path
DYLD_LIBRARY_PATH=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/lib/clang/17/lib/darwin \
  ./bazel-bin/rust/cpp-sanitizer-bindings/config_test_asan
```

**Linux:**
```bash
# Build the test  
bazel build //rust/cpp-sanitizer-bindings:config_test_asan

# Run directly with library path
LD_LIBRARY_PATH=/usr/lib/llvm-16/lib/clang/16/lib/linux \
  ./bazel-bin/rust/cpp-sanitizer-bindings/config_test_asan
```

### Option 3: Use System Compiler Instead of Bazel LLVM Toolchain

If you need to run sanitizers regularly, consider using your system compiler which includes sanitizer runtimes. This requires modifying MODULE.bazel to not register the LLVM toolchain, or using `--repo_env=BAZEL_DO_NOT_DETECT_CPP_TOOLCHAIN=1` and configuring a local toolchain.

## Available Tests

All tests mirror those in `//rust/cpp-bindings` but with ASan/LSan enabled:

- `config_test_asan` - Configuration handling tests
- `autolink_test_asan` - Autolinking functionality tests
- `extractor_test_asan` - Entity extraction tests  
- `hithighlighter_test_asan` - Hit highlighting tests
- `validator_test_asan` - Validation tests

## What These Tests Detect

When working properly, these tests can detect:

- Memory leaks
- Use-after-free errors
- Buffer overflows
- Stack and heap buffer overruns
- Use of uninitialized memory
- Double frees

## Future Improvements

To make these tests work seamlessly in Bazel, we would need to either:

1. Use a Bazel LLVM toolchain distribution that includes sanitizer runtimes
2. Package the sanitizer runtimes as separate Bazel targets and add them as data dependencies with proper rpath configuration
3. Switch to using the system compiler toolchain which includes sanitizers

For now, Option 2 (running tests directly) is the most practical approach for local development.
