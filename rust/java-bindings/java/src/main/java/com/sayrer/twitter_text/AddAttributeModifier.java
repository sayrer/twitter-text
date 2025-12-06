package com.sayrer.twitter_text;

import com.sayrer.twitter_text.autolink_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * A modifier that adds a custom attribute to links of specific entity types.
 */
public final class AddAttributeModifier implements AutoCloseable {
    private final MemorySegment handle;
    private boolean closed;

    /**
     * Create a new AddAttributeModifier.
     *
     * @param entityTypes Array of entity types to apply the modifier to (URL, HASHTAG, MENTION, CASHTAG)
     * @param key The attribute key to add
     * @param value The attribute value to add
     */
    public AddAttributeModifier(Entity.Type[] entityTypes, String key, String value) {
        this.handle = createHandle(entityTypes, key, value);
    }

    private static MemorySegment createHandle(Entity.Type[] entityTypes, String key, String value) {
        try (Arena arena = Arena.ofConfined()) {
            // Convert entity types to C enum values
            int[] types = new int[entityTypes.length];
            for (int i = 0; i < entityTypes.length; i++) {
                types[i] = entityTypeToC(entityTypes[i]);
            }

            MemorySegment typesSegment = arena.allocateFrom(java.lang.foreign.ValueLayout.JAVA_INT, types);
            MemorySegment keySegment = arena.allocateFrom(key);
            MemorySegment valueSegment = arena.allocateFrom(value);

            return (MemorySegment) autolink_h
                .twitter_text_add_attribute_modifier_new$handle()
                .invoke(typesSegment, (long) entityTypes.length, keySegment, valueSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create AddAttributeModifier", t);
        }
    }

    private static int entityTypeToC(Entity.Type type) {
        switch (type) {
            case URL: return 0;
            case HASHTAG: return 1;
            case MENTION: return 2;
            case CASHTAG: return 3;
            default: throw new IllegalArgumentException("Unknown entity type: " + type);
        }
    }

    MemorySegment getHandle() {
        if (closed) {
            throw new IllegalStateException("AddAttributeModifier is closed");
        }
        return handle;
    }

    @Override
    public void close() {
        if (!closed) {
            try {
                autolink_h
                    .twitter_text_add_attribute_modifier_free$handle()
                    .invoke(handle);
            } catch (Throwable t) {
                throw new RuntimeException("Failed to free AddAttributeModifier", t);
            }
            closed = true;
        }
    }
}
