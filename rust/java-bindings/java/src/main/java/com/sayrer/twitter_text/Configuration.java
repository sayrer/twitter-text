package com.sayrer.twitter_text;

import com.sayrer.twitter_text.configuration_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * Java wrapper for TwitterTextConfiguration that provides automatic resource management.
 *
 * Example usage:
 * <pre>
 * try (Configuration config = Configuration.createDefault()) {
 *     try (Validator validator = Validator.withConfig(config)) {
 *         boolean valid = validator.isValidTweet("Hello, world!");
 *     }
 * }
 * </pre>
 */
public final class Configuration implements AutoCloseable {

    private final MemorySegment handle;
    private boolean closed;

    private Configuration(MemorySegment handle) {
        this.handle = handle;
    }

    /**
     * Create a new Configuration instance with default settings.
     *
     * @return a new Configuration instance
     */
    public static Configuration createDefault() {
        try {
            MemorySegment ptr = (MemorySegment) configuration_h
                .twitter_text_config_default$handle()
                .invoke();
            return new Configuration(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create Configuration", t);
        }
    }

    /**
     * Create a new Configuration instance with v3 settings.
     *
     * @return a new Configuration instance with v3 config
     */
    public static Configuration createV3() {
        try {
            MemorySegment ptr = (MemorySegment) configuration_h
                .twitter_text_config_v3$handle()
                .invoke();
            return new Configuration(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create v3 Configuration", t);
        }
    }

    /**
     * Create a Configuration from a JSON string.
     *
     * @param json the JSON configuration string
     * @return a new Configuration instance, or null if parsing fails
     */
    public static Configuration fromJson(String json) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment jsonSegment = arena.allocateFrom(json);
            MemorySegment ptr = (MemorySegment) configuration_h
                .twitter_text_config_from_json$handle()
                .invoke(jsonSegment);

            if (ptr.address() == 0) {
                return null;
            }

            return new Configuration(ptr);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to create Configuration from JSON",
                t
            );
        }
    }

    /**
     * Get the internal handle for use with native functions.
     * Package-private - only accessible within the twitter_text package.
     *
     * @return the native memory segment handle
     */
    MemorySegment getHandle() {
        checkNotClosed();
        return handle;
    }

    /**
     * Get the configuration version.
     *
     * @return the version number
     */
    public int getVersion() {
        checkNotClosed();
        try {
            return (int) configuration_h
                .twitter_text_config_get_version$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to get version", t);
        }
    }

    /**
     * Get the maximum weighted tweet length.
     *
     * @return the maximum weighted tweet length
     */
    public int getMaxWeightedTweetLength() {
        checkNotClosed();
        try {
            return (int) configuration_h
                .twitter_text_config_get_max_weighted_tweet_length$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to get max weighted tweet length",
                t
            );
        }
    }

    /**
     * Get the scale factor.
     *
     * @return the scale factor
     */
    public int getScale() {
        checkNotClosed();
        try {
            return (int) configuration_h
                .twitter_text_config_get_scale$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to get scale", t);
        }
    }

    /**
     * Get the default weight.
     *
     * @return the default weight
     */
    public int getDefaultWeight() {
        checkNotClosed();
        try {
            return (int) configuration_h
                .twitter_text_config_get_default_weight$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to get default weight", t);
        }
    }

    /**
     * Get the transformed URL length.
     *
     * @return the transformed URL length
     */
    public int getTransformedUrlLength() {
        checkNotClosed();
        try {
            return (int) configuration_h
                .twitter_text_config_get_transformed_url_length$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to get transformed URL length",
                t
            );
        }
    }

    /**
     * Check if emoji parsing is enabled.
     *
     * @return true if emoji parsing is enabled
     */
    public boolean isEmojiParsingEnabled() {
        checkNotClosed();
        try {
            return (boolean) configuration_h
                .twitter_text_config_get_emoji_parsing_enabled$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to get emoji parsing enabled",
                t
            );
        }
    }

    /**
     * Set the configuration version.
     *
     * @param version the version to set
     */
    public void setVersion(int version) {
        checkNotClosed();
        try {
            configuration_h
                .twitter_text_config_set_version$handle()
                .invoke(handle, version);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set version", t);
        }
    }

    /**
     * Set the maximum weighted tweet length.
     *
     * @param length the maximum weighted tweet length to set
     */
    public void setMaxWeightedTweetLength(int length) {
        checkNotClosed();
        try {
            configuration_h
                .twitter_text_config_set_max_weighted_tweet_length$handle()
                .invoke(handle, length);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set max weighted tweet length",
                t
            );
        }
    }

    /**
     * Set the scale factor.
     *
     * @param scale the scale to set
     */
    public void setScale(int scale) {
        checkNotClosed();
        try {
            configuration_h
                .twitter_text_config_set_scale$handle()
                .invoke(handle, scale);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set scale", t);
        }
    }

    /**
     * Set the default weight.
     *
     * @param weight the default weight to set
     */
    public void setDefaultWeight(int weight) {
        checkNotClosed();
        try {
            configuration_h
                .twitter_text_config_set_default_weight$handle()
                .invoke(handle, weight);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set default weight", t);
        }
    }

    /**
     * Set the transformed URL length.
     *
     * @param length the transformed URL length to set
     */
    public void setTransformedUrlLength(int length) {
        checkNotClosed();
        try {
            configuration_h
                .twitter_text_config_set_transformed_url_length$handle()
                .invoke(handle, length);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set transformed URL length",
                t
            );
        }
    }

    /**
     * Set whether emoji parsing is enabled.
     *
     * @param enabled true to enable emoji parsing
     */
    public void setEmojiParsingEnabled(boolean enabled) {
        checkNotClosed();
        try {
            configuration_h
                .twitter_text_config_set_emoji_parsing_enabled$handle()
                .invoke(handle, enabled);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set emoji parsing enabled",
                t
            );
        }
    }

    private void checkNotClosed() {
        if (closed) {
            throw new IllegalStateException("Configuration has been closed");
        }
    }

    @Override
    public void close() {
        if (closed) return;
        closed = true;
        try {
            configuration_h.twitter_text_config_free$handle().invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to free Configuration", t);
        }
    }
}
