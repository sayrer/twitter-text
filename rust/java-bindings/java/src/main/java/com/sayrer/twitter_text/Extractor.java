package com.sayrer.twitter_text;

import com.sayrer.twitter_text.extractor_h;
import com.sayrer.twitter_text.TwitterTextEntity;
import com.sayrer.twitter_text.TwitterTextStringArray;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;

/**
 * Java wrapper for TwitterTextExtractor that provides automatic resource management.
 *
 * Example usage:
 * <pre>
 * try (Extractor extractor = Extractor.create()) {
 *     String[] urls = extractor.extractUrls("Check out https://example.com");
 *     for (String url : urls) {
 *         System.out.println(url);
 *     }
 * }
 * </pre>
 */
public final class Extractor implements AutoCloseable {

    private final MemorySegment handle;
    private boolean closed;

    private Extractor(MemorySegment handle) {
        this.handle = handle;
    }

    /**
     * Create a new Extractor instance.
     *
     * @return a new Extractor instance
     */
    public static Extractor create() {
        try {
            MemorySegment ptr =
                (MemorySegment) extractor_h.twitter_text_extractor_new$handle().invoke();
            return new Extractor(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create Extractor", t);
        }
    }

    /**
     * Set whether to extract URLs without protocol (e.g., "example.com").
     *
     * @param extract true to extract URLs without protocol, false otherwise
     */
    public void setExtractUrlWithoutProtocol(boolean extract) {
        checkNotClosed();
        try {
            extractor_h.twitter_text_extractor_set_extract_url_without_protocol$handle()
                .invoke(handle, extract);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set extract URL without protocol", t);
        }
    }

    /**
     * Get whether URLs without protocol are extracted.
     *
     * @return true if URLs without protocol are extracted
     */
    public boolean getExtractUrlWithoutProtocol() {
        checkNotClosed();
        try {
            return (boolean) extractor_h.twitter_text_extractor_get_extract_url_without_protocol$handle()
                .invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to get extract URL without protocol", t);
        }
    }

    /**
     * Extract URLs from text as simple strings.
     *
     * @param text the text to extract URLs from
     * @return array of extracted URLs
     */
    public String[] extractUrls(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment arraySegment =
                (MemorySegment) extractor_h.twitter_text_extractor_extract_urls$handle()
                    .invoke(arena, handle, textSegment);

            String[] result = extractStringArray(arraySegment);

            // Free the array
            extractor_h.twitter_text_string_array_free(arraySegment);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to extract URLs", t);
        }
    }

    /**
     * Extract hashtags from text as simple strings.
     *
     * @param text the text to extract hashtags from
     * @return array of extracted hashtags (without the # symbol)
     */
    public String[] extractHashtags(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment arraySegment =
                (MemorySegment) extractor_h.twitter_text_extractor_extract_hashtags$handle()
                    .invoke(arena, handle, textSegment);

            String[] result = extractStringArray(arraySegment);
            extractor_h.twitter_text_string_array_free(arraySegment);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to extract hashtags", t);
        }
    }

    /**
     * Extract cashtags from text as simple strings.
     *
     * @param text the text to extract cashtags from
     * @return array of extracted cashtags (without the $ symbol)
     */
    public String[] extractCashtags(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment arraySegment =
                (MemorySegment) extractor_h.twitter_text_extractor_extract_cashtags$handle()
                    .invoke(arena, handle, textSegment);

            String[] result = extractStringArray(arraySegment);
            extractor_h.twitter_text_string_array_free(arraySegment);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to extract cashtags", t);
        }
    }

    /**
     * Extract mentioned screennames from text as simple strings.
     *
     * @param text the text to extract mentions from
     * @return array of extracted screennames (without the @ symbol)
     */
    public String[] extractMentionedScreennames(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment arraySegment =
                (MemorySegment) extractor_h.twitter_text_extractor_extract_mentioned_screennames$handle()
                    .invoke(arena, handle, textSegment);

            String[] result = extractStringArray(arraySegment);
            extractor_h.twitter_text_string_array_free(arraySegment);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to extract mentioned screennames", t);
        }
    }

    /**
     * Extract the reply username from text (the @username at the very beginning).
     *
     * @param text the text to extract reply username from
     * @return the reply username (without @), or null if none found
     */
    public String extractReplyUsername(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment entityPtr =
                (MemorySegment) extractor_h.twitter_text_extractor_extract_reply_username$handle()
                    .invoke(handle, textSegment);

            if (entityPtr.address() == 0) {
                return null;
            }

            // Extract the value field from the entity
            MemorySegment valuePtr = TwitterTextEntity.value(entityPtr);
            String result = valuePtr.getString(0);

            // Free the entity
            extractor_h.twitter_text_entity_free(entityPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to extract reply username", t);
        }
    }

    /**
     * Helper method to extract a string array from a TwitterTextStringArray struct.
     */
    private String[] extractStringArray(MemorySegment arraySegment) {
        long length = TwitterTextStringArray.length(arraySegment);

        if (length == 0) {
            return new String[0];
        }

        MemorySegment stringsPtr = TwitterTextStringArray.strings(arraySegment);
        // Reinterpret the pointer array with the correct size
        MemorySegment stringsPtrArray = stringsPtr.reinterpret(length * ValueLayout.ADDRESS.byteSize());

        String[] result = new String[(int) length];
        for (int i = 0; i < length; i++) {
            MemorySegment stringPtr = stringsPtrArray.getAtIndex(ValueLayout.ADDRESS, i);
            // Reinterpret as unbounded to allow reading the null-terminated string
            result[i] = stringPtr.reinterpret(Long.MAX_VALUE).getString(0);
        }

        return result;
    }

    private void checkNotClosed() {
        if (closed) {
            throw new IllegalStateException("Extractor has been closed");
        }
    }

    @Override
    public void close() {
        if (closed) return;
        closed = true;
        try {
            extractor_h.twitter_text_extractor_free$handle().invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to free Extractor", t);
        }
    }
}
