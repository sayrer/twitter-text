package com.sayrer.twitter_text;

import com.sayrer.twitter_text.autolink_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * Java wrapper for TwitterTextAutolinker that provides automatic resource management.
 *
 * Example usage:
 * <pre>
 * try (Autolinker autolinker = Autolinker.create(true)) {
 *     String result = autolinker.autolink("Check out https://example.com and @user");
 *     System.out.println(result);
 * }
 * </pre>
 */
public final class Autolinker implements AutoCloseable {

    private final MemorySegment handle;
    private boolean closed;

    private Autolinker(MemorySegment handle) {
        this.handle = handle;
    }

    /**
     * Create a new Autolinker instance.
     *
     * @param noFollow whether to add rel="nofollow" to generated links
     * @return a new Autolinker instance
     */
    public static Autolinker create(boolean noFollow) {
        try {
            MemorySegment ptr =
                (MemorySegment) autolink_h.twitter_text_autolinker_new$handle().invoke(noFollow);
            return new Autolinker(ptr);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create Autolinker", t);
        }
    }

    /**
     * Auto-link all entities (URLs, hashtags, mentions, cashtags) in the text.
     *
     * @param text the text to process
     * @return HTML string with entities converted to links
     */
    public String autolink(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment resultPtr =
                (MemorySegment) autolink_h.twitter_text_autolinker_autolink$handle()
                    .invoke(handle, textSegment);

            String result = resultPtr.getString(0);

            // Free the returned string
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink text", t);
        }
    }

    /**
     * Auto-link only @username and @username/list references.
     *
     * @param text the text to process
     * @return HTML string with usernames/lists converted to links
     */
    public String autolinkUsernamesAndLists(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment resultPtr =
                (MemorySegment) autolink_h.twitter_text_autolinker_autolink_usernames_and_lists$handle()
                    .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink usernames and lists", t);
        }
    }

    /**
     * Auto-link only #hashtag references.
     *
     * @param text the text to process
     * @return HTML string with hashtags converted to links
     */
    public String autolinkHashtags(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment resultPtr =
                (MemorySegment) autolink_h.twitter_text_autolinker_autolink_hashtags$handle()
                    .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink hashtags", t);
        }
    }

    /**
     * Auto-link only URLs.
     *
     * @param text the text to process
     * @return HTML string with URLs converted to links
     */
    public String autolinkUrls(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment resultPtr =
                (MemorySegment) autolink_h.twitter_text_autolinker_autolink_urls$handle()
                    .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink URLs", t);
        }
    }

    /**
     * Auto-link only $cashtag references.
     *
     * @param text the text to process
     * @return HTML string with cashtags converted to links
     */
    public String autolinkCashtags(String text) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment textSegment = arena.allocateFrom(text);
            MemorySegment resultPtr =
                (MemorySegment) autolink_h.twitter_text_autolinker_autolink_cashtags$handle()
                    .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink cashtags", t);
        }
    }

    /**
     * Set the CSS class for URL links.
     *
     * @param urlClass the CSS class to use
     */
    public void setUrlClass(String urlClass) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment classSegment = arena.allocateFrom(urlClass);
            autolink_h.twitter_text_autolinker_set_url_class$handle()
                .invoke(handle, classSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set URL class", t);
        }
    }

    /**
     * Set the target attribute for URL links.
     *
     * @param urlTarget the target attribute (e.g., "_blank")
     */
    public void setUrlTarget(String urlTarget) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment targetSegment = arena.allocateFrom(urlTarget);
            autolink_h.twitter_text_autolinker_set_url_target$handle()
                .invoke(handle, targetSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set URL target", t);
        }
    }

    /**
     * Set the CSS class for hashtag links.
     *
     * @param hashtagClass the CSS class to use
     */
    public void setHashtagClass(String hashtagClass) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment classSegment = arena.allocateFrom(hashtagClass);
            autolink_h.twitter_text_autolinker_set_hashtag_class$handle()
                .invoke(handle, classSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set hashtag class", t);
        }
    }

    /**
     * Set the CSS class for username links.
     *
     * @param usernameClass the CSS class to use
     */
    public void setUsernameClass(String usernameClass) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment classSegment = arena.allocateFrom(usernameClass);
            autolink_h.twitter_text_autolinker_set_username_class$handle()
                .invoke(handle, classSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set username class", t);
        }
    }

    /**
     * Set the CSS class for cashtag links.
     *
     * @param cashtagClass the CSS class to use
     */
    public void setCashtagClass(String cashtagClass) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment classSegment = arena.allocateFrom(cashtagClass);
            autolink_h.twitter_text_autolinker_set_cashtag_class$handle()
                .invoke(handle, classSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set cashtag class", t);
        }
    }

    private void checkNotClosed() {
        if (closed) {
            throw new IllegalStateException("Autolinker has been closed");
        }
    }

    @Override
    public void close() {
        if (closed) return;
        closed = true;
        try {
            autolink_h.twitter_text_autolinker_free$handle().invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to free Autolinker", t);
        }
    }
}
