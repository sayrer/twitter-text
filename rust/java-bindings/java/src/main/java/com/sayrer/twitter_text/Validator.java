package com.sayrer.twitter_text;


import com.sayrer.twitter_text.validator_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * Java wrapper for TwitterTextValidator that provides automatic resource management.
 *
 * Example usage:
 * <pre>
 * try (Validator validator = Validator.create()) {
 *     boolean valid = validator.isValidTweet("Hello, world!");
 *     System.out.println("Valid tweet: " + valid);
 * }
 * </pre>
 */
public final class Validator implements AutoCloseable {


    private final MemorySegment handle;
    private boolean closed;

    private Validator(MemorySegment handle) {
        this.handle = handle;
    }

    /**
     * Create a new Validator instance with default configuration.
     *
     * @return a new Validator instance
     */
    public static Validator create() {
        try {
            MemorySegment ptr = (MemorySegment) validator_h
                .twitter_text_validator_new$handle()
                .invoke();
            return new Validator(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create Validator", t);
        }
    }

    /**
     * Create a new Validator instance with a custom configuration.
     *
     * @param config the Configuration to use
     * @return a new Validator instance
     */
    public static Validator withConfig(Configuration config) {
        if (config == null) {
            return create();
        }
        try {
            MemorySegment ptr = (MemorySegment) validator_h
                .twitter_text_validator_with_config$handle()
                .invoke(config.getHandle());
            return new Validator(ptr);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to create Validator with config",
                t
            );
        }
    }

    /**
     * Check if the text is a valid tweet according to Twitter's rules.
     *
     * @param text the text to validate
     * @return true if valid, false otherwise
     */
    public boolean isValidTweet(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            boolean result = (boolean) validator_h
                .twitter_text_validator_is_valid_tweet$handle()
                .invoke(handle, textSegment);
            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to validate tweet", t);
        }
    }

    /**
     * Check if the username is valid according to Twitter's rules.
     *
     * @param username the username to validate (without @)
     * @return true if valid, false otherwise
     */
    public boolean isValidUsername(String username) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment usernameSegment = arena.allocateFrom(username);
            return (boolean) validator_h
                .twitter_text_validator_is_valid_username$handle()
                .invoke(handle, usernameSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to validate username", t);
        }
    }

    /**
     * Check if the list name is valid according to Twitter's rules.
     *
     * @param list the list name to validate
     * @return true if valid, false otherwise
     */
    public boolean isValidList(String list) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment listSegment = arena.allocateFrom(list);
            return (boolean) validator_h
                .twitter_text_validator_is_valid_list$handle()
                .invoke(handle, listSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to validate list", t);
        }
    }

    /**
     * Check if the hashtag is valid according to Twitter's rules.
     *
     * @param hashtag the hashtag to validate (without #)
     * @return true if valid, false otherwise
     */
    public boolean isValidHashtag(String hashtag) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment hashtagSegment = arena.allocateFrom(hashtag);
            return (boolean) validator_h
                .twitter_text_validator_is_valid_hashtag$handle()
                .invoke(handle, hashtagSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to validate hashtag", t);
        }
    }

    /**
     * Check if the URL is valid according to Twitter's rules.
     *
     * @param url the URL to validate
     * @return true if valid, false otherwise
     */
    public boolean isValidUrl(String url) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment urlSegment = arena.allocateFrom(url);
            return (boolean) validator_h
                .twitter_text_validator_is_valid_url$handle()
                .invoke(handle, urlSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to validate URL", t);
        }
    }

    /**
     * Check if the URL without protocol is valid according to Twitter's rules.
     *
     * @param url the URL to validate (without protocol)
     * @return true if valid, false otherwise
     */
    public boolean isValidUrlWithoutProtocol(String url) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment urlSegment = arena.allocateFrom(url);
            return (boolean) validator_h
                .twitter_text_validator_is_valid_url_without_protocol$handle()
                .invoke(handle, urlSegment);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to validate URL without protocol",
                t
            );
        }
    }

    /**
     * Get the maximum tweet length.
     *
     * @return the maximum tweet length
     */
    public int getMaxTweetLength() {
        checkNotClosed();
        try {
            return (int) validator_h
                .twitter_text_validator_get_max_tweet_length$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to get max tweet length", t);
        }
    }

    /**
     * Get the short URL length.
     *
     * @return the short URL length
     */
    public int getShortUrlLength() {
        checkNotClosed();
        try {
            return (int) validator_h
                .twitter_text_validator_get_short_url_length$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to get short URL length", t);
        }
    }

    /**
     * Get the short URL length for HTTPS URLs.
     *
     * @return the short URL length for HTTPS
     */
    public int getShortUrlLengthHttps() {
        checkNotClosed();
        try {
            return (int) validator_h
                .twitter_text_validator_get_short_url_length_https$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to get short URL length HTTPS",
                t
            );
        }
    }

    /**
     * Set the short URL length.
     *
     * @param length the short URL length to set
     */
    public void setShortUrlLength(int length) {
        checkNotClosed();
        try {
            validator_h
                .twitter_text_validator_set_short_url_length$handle()
                .invoke(handle, length);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set short URL length", t);
        }
    }

    /**
     * Set the short URL length for HTTPS URLs.
     *
     * @param length the short URL length for HTTPS to set
     */
    public void setShortUrlLengthHttps(int length) {
        checkNotClosed();
        try {
            validator_h
                .twitter_text_validator_set_short_url_length_https$handle()
                .invoke(handle, length);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set short URL length HTTPS",
                t
            );
        }
    }

    private void checkNotClosed() {
        if (closed) {
            throw new IllegalStateException("Validator has been closed");
        }
    }

    @Override
    public void close() {
        if (closed) return;
        closed = true;
        try {
            validator_h.twitter_text_validator_free$handle().invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to free Validator", t);
        }
    }
}
