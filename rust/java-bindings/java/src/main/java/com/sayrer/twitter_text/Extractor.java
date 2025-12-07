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

    private MemorySegment handle;
    private boolean closed;

    private Extractor(MemorySegment handle) {
        this.handle = handle;
    }

    /**
     * Create a new Extractor instance using the default constructor.
     * This is the preferred constructor for most use cases.
     */
    public Extractor() {
        try {
            this.handle = (MemorySegment) extractor_h.twitter_text_extractor_new$handle().invoke();
            this.closed = false;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create Extractor", t);
        }
    }

    /**
     * Create a new Extractor instance.
     *
     * @return a new Extractor instance
     */
    public static Extractor create() {
        return new Extractor();
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
    public String extractReplyScreenname(String text) {
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
     * Extract URLs with their indices.
     *
     * @param text the text to extract URLs from
     * @return list of entities with start/end indices
     */
    public java.util.List<Entity> extractURLsWithIndices(String text) {
        return extractEntitiesWithIndices(text, Entity.Type.URL,
            extractor_h.twitter_text_extractor_extract_urls_with_indices$handle());
    }

    /**
     * Extract hashtags with their indices.
     *
     * @param text the text to extract hashtags from
     * @return list of entities with start/end indices
     */
    public java.util.List<Entity> extractHashtagsWithIndices(String text) {
        return extractEntitiesWithIndices(text, Entity.Type.HASHTAG,
            extractor_h.twitter_text_extractor_extract_hashtags_with_indices$handle());
    }

    /**
     * Extract mentioned screennames with their indices.
     *
     * @param text the text to extract mentions from
     * @return list of entities with start/end indices
     */
    public java.util.List<Entity> extractMentionedScreennamesWithIndices(String text) {
        return extractEntitiesWithIndices(text, Entity.Type.MENTION,
            extractor_h.twitter_text_extractor_extract_mentioned_screennames_with_indices$handle());
    }

    /**
     * Extract cashtags with their indices.
     *
     * @param text the text to extract cashtags from
     * @return list of entities with start/end indices
     */
    public java.util.List<Entity> extractCashtagsWithIndices(String text) {
        return extractEntitiesWithIndices(text, Entity.Type.CASHTAG,
            extractor_h.twitter_text_extractor_extract_cashtags_with_indices$handle());
    }

    /**
     * Helper method to extract entities with indices.
     */
    private java.util.List<Entity> extractEntitiesWithIndices(String text, Entity.Type type,
                                                               java.lang.invoke.MethodHandle methodHandle) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment arraySegment = (MemorySegment) methodHandle.invoke(arena, handle, textSegment);

            java.util.List<Entity> result = new java.util.ArrayList<>();

            // Read the TwitterTextEntityArray struct
            long length = com.sayrer.twitter_text.TwitterTextEntityArray.length(arraySegment);

            if (length > 0) {
                MemorySegment entitiesPtr = com.sayrer.twitter_text.TwitterTextEntityArray.entities(arraySegment);
                long entitySize = TwitterTextEntity.sizeof();

                for (int i = 0; i < length; i++) {
                    MemorySegment entitySegment = entitiesPtr.asSlice(i * entitySize, entitySize);

                    int start = TwitterTextEntity.start(entitySegment);
                    int end = TwitterTextEntity.end(entitySegment);
                    MemorySegment valuePtr = TwitterTextEntity.value(entitySegment);
                    String value = valuePtr.reinterpret(Long.MAX_VALUE).getString(0);

                    Entity entity = new Entity(start, end, value, type);

                    // Extract optional fields if present
                    MemorySegment displayUrlPtr = TwitterTextEntity.display_url(entitySegment);
                    if (displayUrlPtr.address() != 0) {
                        entity.setDisplayURL(displayUrlPtr.reinterpret(Long.MAX_VALUE).getString(0));
                    }

                    MemorySegment expandedUrlPtr = TwitterTextEntity.expanded_url(entitySegment);
                    if (expandedUrlPtr.address() != 0) {
                        entity.setExpandedURL(expandedUrlPtr.reinterpret(Long.MAX_VALUE).getString(0));
                    }

                    result.add(entity);
                }
            }

            // Free the array
            extractor_h.twitter_text_entity_array_free(arraySegment);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to extract entities with indices", t);
        }
    }

    /**
     * Modify entity indices from UTF-16 (Java char offsets) to Unicode (code point offsets).
     * This is useful when working with text containing supplementary characters.
     *
     * @param text the text that was parsed
     * @param entities the entities to modify
     */
    public void modifyIndicesFromUTF16ToUnicode(String text, java.util.List<Entity> entities) {
        for (Entity entity : entities) {
            entity.start = text.codePointCount(0, entity.start);
            entity.end = text.codePointCount(0, entity.end);
        }
    }

    /**
     * Modify entity indices from Unicode (code point offsets) to UTF-16 (Java char offsets).
     * This is the inverse of modifyIndicesFromUTF16ToUnicode.
     *
     * @param text the text that was parsed
     * @param entities the entities to modify
     */
    public void modifyIndicesFromUnicodeToUTF16(String text, java.util.List<Entity> entities) {
        for (Entity entity : entities) {
            entity.start = text.offsetByCodePoints(0, entity.start);
            entity.end = text.offsetByCodePoints(0, entity.end);
        }
    }

    /**
     * Set whether to extract URLs without protocol.
     * Alias for setExtractUrlWithoutProtocol for test compatibility.
     */
    public void setExtractURLWithoutProtocol(boolean extract) {
        setExtractUrlWithoutProtocol(extract);
    }

    /**
     * Extract URLs from text.
     * Converts the array result to a List for test compatibility.
     */
    public java.util.List<String> extractURLs(String text) {
        return java.util.Arrays.asList(extractUrls(text));
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
