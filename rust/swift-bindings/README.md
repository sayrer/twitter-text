# TwitterText Swift

Swift bindings for the twitter-text library, providing parsing and validation for tweet text.

## Features

- Extract URLs, mentions, hashtags, and cashtags from tweet text
- Automatic linking of entities to HTML
- Tweet validation and length calculation
- Support for different configuration versions (v1, v2, v3)
- Full Unicode support

## Installation

### Swift Package Manager

Add to your `Package.swift`:

```swift
dependencies: [
    .package(path: "path/to/twitter-text/rust/swift-bindings")
]
```

### Building the Native Library

The Swift package depends on the Rust FFI library. Build it first:

```bash
# From the repository root
bazel build //rust/ffi-bindings:twitter_text_ffi_cdylib
```

## Usage

### Extracting Entities

```swift
import TwitterText

let extractor = Extractor()
let text = "Check out https://example.com and follow @user #swift"

// Extract all entities
let entities = extractor.extractEntities(from: text)

// Extract specific types
let urls = extractor.extractURLs(from: text)
let mentions = extractor.extractMentions(from: text)
let hashtags = extractor.extractHashtags(from: text)

for entity in entities {
    print("\(entity.type): \(entity.value) at \(entity.range)")
}
```

### Autolinking

```swift
import TwitterText

let autolinker = Autolinker()
let text = "Check out https://example.com and follow @user #swift"

// Link all entities
let html = autolinker.autolink(text)

// Link specific types
let urlsLinked = autolinker.autolinkURLs(text)
let hashtagsLinked = autolinker.autolinkHashtags(text)

// Customize link attributes
autolinker.setURLClass("custom-link")
autolinker.setNoFollow(false)
```

### Configuration

```swift
import TwitterText

// Use default configuration (v3)
let config = Configuration.default

// Or use specific versions
let v1 = Configuration.v1
let v2 = Configuration.v2
let v3 = Configuration.v3

// Access configuration properties
print("Max tweet length: \(config.maxWeightedTweetLength)")
print("Emoji parsing: \(config.emojiParsingEnabled)")
```

## Testing

Run tests with:

```bash
swift test
```

## Requirements

- Swift 5.9+
- macOS 10.15+ or iOS 13+
- Rust FFI library (built via Bazel)

## License

Same as the parent twitter-text project.
