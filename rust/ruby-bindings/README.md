# Ruby Bindings

The Ruby bindings use [Magnus](https://github.com/matsadler/magnus), a Rust-Ruby FFI library that allows calling Rust code from Ruby.

## Prerequisites

Ruby 3.4.1 requires `libyaml` to be available during compilation. Install it using your system package manager:

### macOS (Homebrew)
```bash
brew install libyaml
```

The `.bazelrc` file already configures the necessary `CPPFLAGS` and `LDFLAGS` for Homebrew installations on macOS.

### Linux (Debian/Ubuntu)
```bash
sudo apt-get install libyaml-dev
```

### Linux (Fedora/RHEL)
```bash
sudo dnf install libyaml-devel
```

## Building

Once libyaml is installed:

```bash
bazel build //rust/ruby-bindings:twittertext
```

This produces:
- `twittertext.bundle` on macOS
- `twittertext.so` on Linux

## Running Tests

```bash
bazel test //rust/ruby-bindings/spec:all
```

## How It Works

1. **Ruby Toolchain**: `rules_ruby` (configured in `MODULE.bazel`) downloads and builds Ruby 3.4.1 from source
2. **Build Configuration**: The `.bazelrc` sets `RUBY_CONFIGURE_OPTS` and platform-specific compiler flags to help find system dependencies like libyaml
3. **Rust Compilation**: The `rust_shared_library` target compiles the Rust code with Magnus dependencies
4. **FFI Bindings**: Magnus generates the Ruby C API bindings at compile time, linking against the Ruby toolchain
5. **Native Extension**: The output is a native Ruby extension that can be loaded with `require`

## Known Limitations

`rules_ruby` doesn't currently provide hermetic build dependencies for native extensions. This means:
- libyaml must be installed system-wide
- Other system libraries required by Ruby gems must be available on the host system

This is a limitation of the current `rules_ruby` toolchain integration with Bazel.
