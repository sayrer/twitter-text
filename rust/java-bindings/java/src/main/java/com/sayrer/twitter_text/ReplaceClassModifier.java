package com.sayrer.twitter_text;

import com.sayrer.twitter_text.autolink_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * A modifier that replaces the class attribute value.
 */
public final class ReplaceClassModifier implements AutoCloseable {
    private final MemorySegment handle;
    private boolean closed;

    /**
     * Create a new ReplaceClassModifier.
     *
     * @param newClass The new class attribute value
     */
    public ReplaceClassModifier(String newClass) {
        this.handle = createHandle(newClass);
    }

    private static MemorySegment createHandle(String newClass) {
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment classSegment = arena.allocateFrom(newClass);

            return (MemorySegment) autolink_h
                .twitter_text_replace_class_modifier_new$handle()
                .invoke(classSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create ReplaceClassModifier", t);
        }
    }

    MemorySegment getHandle() {
        if (closed) {
            throw new IllegalStateException("ReplaceClassModifier is closed");
        }
        return handle;
    }

    @Override
    public void close() {
        if (!closed) {
            try {
                autolink_h
                    .twitter_text_replace_class_modifier_free$handle()
                    .invoke(handle);
            } catch (Throwable t) {
                throw new RuntimeException("Failed to free ReplaceClassModifier", t);
            }
            closed = true;
        }
    }
}
