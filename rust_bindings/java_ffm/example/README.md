# Java FFM C Bindings Demo

This is a simple demonstration of calling C functions from Java using the Foreign Function & Memory (FFM) API.

## Files

- `math.h` - C header file declaring simple math functions
- `math.c` - C implementation of the math functions
- `MathBindings.java` - Java class that creates method handles for the C functions
- `Main.java` - Java main class that demonstrates calling the C functions
- `BUILD.bazel` - Bazel build rules for the entire demo

## Building and Running with Bazel

### Build the demo

```bash
bazel build //rust_bindings/java_ffm/example:main
```

### Run the demo

```bash
bazel run //rust_bindings/java_ffm/example:main
```

Expected output:
```
Java FFM C Bindings Demo
========================
add_numbers(10, 20) = 30
multiply_numbers(7, 6) = 42
```

## What's Happening

1. Bazel compiles `math.c` into a shared library (`libmath.so`)
2. The `java_binary` rule automatically sets up the library path to find the shared library
3. At runtime, Java's FFM API loads the C library and calls the functions

## Notes

- The FFM API requires Java 21+ with `--enable-preview` flag (or Java 22+ where it's finalized)
- Bazel handles all the library path setup automatically via the `data` attribute
- The shared library name will be `libmath.so` on Linux/macOS
