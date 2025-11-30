# Ruby Bindings Setup

The Ruby bindings use Magnus (Rust-Ruby FFI) and require Ruby 3.4.1 to be built from source by Bazel.

## Prerequisites

Ruby 3.4.1 requires `libyaml` to be available during compilation. Install it using your system package manager:

### macOS (Homebrew)
```bash
brew install libyaml
```

Then uncomment the CPPFLAGS/LDFLAGS lines in `.bazelrc` to point to Homebrew's installation.

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

## How It Works

1. `rules_ruby` downloads and compiles Ruby 3.4.1 from source using `ruby-build`
2. `ruby-build` respects environment variables like `RUBY_CONFIGURE_OPTS`, `CPPFLAGS`, and `LDFLAGS`
3. These are set in `.bazelrc` to help ruby-build find system dependencies
4. `rb-sys` (part of Magnus) generates Ruby FFI bindings at compile time using the built Ruby
5. The Magnus-based bindings are compiled into a Ruby extension (.bundle on macOS, .so on Linux)

## Known Limitations

`rules_ruby` doesn't currently expose a way to provide hermetic build dependencies like libyaml through Bazel. This means libyaml must be installed system-wide. This is tracked as a limitation of the current rules_ruby toolchain integration.
