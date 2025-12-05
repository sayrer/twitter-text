# Twitter Text FFM Examples

This directory contains Java wrapper classes and examples for using the twitter-text library via Java's Foreign Function & Memory (FFM) API.

## Requirements

- Java 23 or later (for FFM support)
- The generated `twitter_text_bindings.srcjar` from `//rust_bindings/java_ffm:generate_java_bindings`
- The compiled native library `libtwitter_text_java_ffm.dylib` (macOS) or `libtwitter_text_java_ffm.so` (Linux)

## Wrapper Classes

The wrapper classes provide idiomatic Java APIs with automatic resource management:

### Autolinker

Converts entities (URLs, hashtags, mentions, cashtags) in text to HTML links.

```java
try (Autolinker autolinker = Autolinker.create(true)) {
    autolinker.setUrlClass("tweet-url");
    autolinker.setHashtagClass("tweet-hashtag");
    
    String result = autolinker.autolink(
        "Check out https://example.com, follow @twitter, and use #java!"
    );
    System.out.println(result);
}
```

**Methods:**
- `autolink(String text)` - Auto-link all entities
- `autolinkUrls(String text)` - Auto-link only URLs
- `autolinkHashtags(String text)` - Auto-link only hashtags
- `autolinkUsernamesAndLists(String text)` - Auto-link only @mentions
- `autolinkCashtags(String text)` - Auto-link only $cashtags
- `setUrlClass(String)`, `setHashtagClass(String)`, etc. - Configure CSS classes

### Extractor

Extracts entities from text.

```java
try (Extractor extractor = Extractor.create()) {
    extractor.setExtractUrlWithoutProtocol(true);
    
    String[] urls = extractor.extractUrls("Visit example.com and test.org");
    String[] hashtags = extractor.extractHashtags("Love #java and #programming");
    String[] mentions = extractor.extractMentionedScreennames("Thanks @alice and @bob");
    String[] cashtags = extractor.extractCashtags("Investing in $AAPL and $GOOGL");
    
    String replyTo = extractor.extractReplyUsername("@user this is a reply");
}
```

**Methods:**
- `extractUrls(String text)` - Extract URLs
- `extractHashtags(String text)` - Extract hashtags (without #)
- `extractCashtags(String text)` - Extract cashtags (without $)
- `extractMentionedScreennames(String text)` - Extract @mentions (without @)
- `extractReplyUsername(String text)` - Extract the @reply target at the start

### Validator

Validates tweets, usernames, hashtags, and URLs according to Twitter's rules.

```java
try (Validator validator = Validator.create()) {
    boolean validTweet = validator.isValidTweet("Hello, world!");
    boolean validUsername = validator.isValidUsername("twitter");
    boolean validHashtag = validator.isValidHashtag("java");
    boolean validUrl = validator.isValidUrl("https://example.com");
    
    int maxLength = validator.getMaxTweetLength();
    int shortUrlLength = validator.getShortUrlLength();
}
```

**Methods:**
- `isValidTweet(String text)` - Check if tweet is valid
- `isValidUsername(String username)` - Check if username is valid
- `isValidHashtag(String hashtag)` - Check if hashtag is valid
- `isValidList(String list)` - Check if list name is valid
- `isValidUrl(String url)` - Check if URL is valid
- `isValidUrlWithoutProtocol(String url)` - Check if URL without protocol is valid
- `getMaxTweetLength()`, `getShortUrlLength()`, etc. - Get configuration values

## Running the Example

### Using Bazel (recommended)

```bash
# Run the example
bazel run //rust_bindings/java_ffm/examples:example
```

That's it! Bazel will automatically:
1. Build the Rust native library
2. Generate the Java FFM bindings
3. Compile the wrapper classes
4. Run the example with the correct JVM flags and library path

### Manual build (without Bazel)

If you want to build manually:

```bash
# 1. Build the native library and bindings
bazel build //rust_bindings/java_ffm:twitter_text_java_ffm
bazel build //rust_bindings/java_ffm:generate_java_bindings

# 2. Extract the srcjar
cd rust_bindings/java_ffm/examples
mkdir -p target/generated-sources
cd target/generated-sources
unzip ../../../bazel-bin/rust_bindings/java_ffm/twitter_text_bindings.srcjar
cd ../..

# 3. Compile
javac --enable-preview --release 23 \
    -d target/classes \
    target/generated-sources/com/sayrer/twitter_text/*.java \
    src/main/java/com/sayrer/twitter_text/examples/*.java

# 4. Run
java --enable-preview \
    -cp target/classes \
    -Djava.library.path=../bazel-bin/rust_bindings/java_ffm \
    com.sayrer.twitter_text.examples.Example
```

## Memory Management

All wrapper classes implement `AutoCloseable` and should be used with try-with-resources:

```java
try (Autolinker autolinker = Autolinker.create(true)) {
    // Use autolinker
} // Automatically freed here
```

The wrappers ensure that:
1. Native resources are freed when `close()` is called
2. Operations on closed instances throw `IllegalStateException`
3. Memory allocated by the native library (strings, arrays) is properly freed

## How It Works

The wrapper classes use Java's Foreign Function & Memory API to:
1. Call native functions via `MethodHandle.invoke()`
2. Allocate native memory with `Arena` for string parameters
3. Read results from native memory segments
4. Free native resources by calling the `_free` functions

For example, `Autolinker.autolink()`:
1. Creates a confined `Arena` for the lifetime of the call
2. Allocates native memory for the input string
3. Calls `twitter_text_autolinker_autolink()` via FFM
4. Reads the result string from native memory
5. Calls `twitter_text_string_free()` to free the result
6. Arena automatically frees the input string when closed

## Notes

- The FFM API requires `--enable-preview` flag in Java 23
- The native library must be on `java.library.path`
- Confined arenas are used for short-lived allocations
- All native strings are UTF-8 encoded
