package com.sayrer.twitter_text;

import com.sayrer.twitter_text.autolink_h;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;

/**
 * Java wrapper for TwitterTextAutolinker that provides automatic resource management.
 *
 * Example usage:
 * <pre>
 * try (Autolink autolink = Autolink.create(true)) {
 *     String result = autolink.autolink("Check out https://example.com and @user");
 *     System.out.println(result);
 * }
 * </pre>
 */
public final class Autolink implements AutoCloseable {

    public static final String DEFAULT_HASHTAG_CLASS = "tweet-url hashtag";
    public static final String DEFAULT_USERNAME_CLASS = "tweet-url username";
    public static final String DEFAULT_CASHTAG_CLASS = "tweet-url cashtag";
    public static final String DEFAULT_LIST_CLASS = "tweet-url list-slug";

    private final MemorySegment handle;
    private boolean closed;

    private Autolink(MemorySegment handle) {
        this.handle = handle;
    }

    /**
     * Create a new Autolink instance with rel="nofollow" enabled by default.
     */
    public Autolink() {
        this(createHandle(true));
    }

    private static MemorySegment createHandle(boolean noFollow) {
        try {
            return (MemorySegment) autolink_h
                .twitter_text_autolinker_new$handle()
                .invoke(noFollow);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to create Autolink", t);
        }
    }

    /**
     * Create a new Autolink instance.
     *
     * @param noFollow whether to add rel="nofollow" to generated links
     * @return a new Autolink instance
     */
    public static Autolink create(boolean noFollow) {
        return new Autolink(createHandle(noFollow));
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
            MemorySegment resultPtr = (MemorySegment) autolink_h
                .twitter_text_autolinker_autolink$handle()
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
     * Auto-link all entities (alias for autolink).
     *
     * @param text the text to process
     * @return HTML string with entities converted to links
     */
    public String autoLink(String text) {
        return autolink(text);
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
            MemorySegment resultPtr = (MemorySegment) autolink_h
                .twitter_text_autolinker_autolink_usernames_and_lists$handle()
                .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to autolink usernames and lists",
                t
            );
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
            MemorySegment resultPtr = (MemorySegment) autolink_h
                .twitter_text_autolinker_autolink_hashtags$handle()
                .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink hashtags", t);
        }
    }

    /**
     * Auto-link only #hashtag references (alias for autolinkHashtags).
     *
     * @param text the text to process
     * @return HTML string with hashtags converted to links
     */
    public String autoLinkHashtags(String text) {
        return autolinkHashtags(text);
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
            MemorySegment resultPtr = (MemorySegment) autolink_h
                .twitter_text_autolinker_autolink_urls$handle()
                .invoke(handle, textSegment);

            String result = resultPtr.getString(0);
            autolink_h.twitter_text_string_free(resultPtr);

            return result;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to autolink URLs", t);
        }
    }

    /**
     * Auto-link only URLs (alias for autolinkUrls).
     *
     * @param text the text to process
     * @return HTML string with URLs converted to links
     */
    public String autoLinkURLs(String text) {
        return autolinkUrls(text);
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
            MemorySegment resultPtr = (MemorySegment) autolink_h
                .twitter_text_autolinker_autolink_cashtags$handle()
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
            autolink_h
                .twitter_text_autolinker_set_url_class$handle()
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
            autolink_h
                .twitter_text_autolinker_set_url_target$handle()
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
            autolink_h
                .twitter_text_autolinker_set_hashtag_class$handle()
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
            autolink_h
                .twitter_text_autolinker_set_username_class$handle()
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
            autolink_h
                .twitter_text_autolinker_set_cashtag_class$handle()
                .invoke(handle, classSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set cashtag class", t);
        }
    }

    /**
     * Set whether to add rel="nofollow" to generated links.
     *
     * @param noFollow whether to add rel="nofollow"
     */
    public void setNoFollow(boolean noFollow) {
        checkNotClosed();
        try {
            autolink_h
                .twitter_text_autolinker_set_no_follow$handle()
                .invoke(handle, noFollow);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set noFollow", t);
        }
    }

    /**
     * Set whether to include the @ symbol in username links.
     *
     * @param usernameIncludeSymbol whether to include the @ symbol
     */
    public void setUsernameIncludeSymbol(boolean usernameIncludeSymbol) {
        checkNotClosed();
        try {
            autolink_h
                .twitter_text_autolinker_set_username_include_symbol$handle()
                .invoke(handle, usernameIncludeSymbol);
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set usernameIncludeSymbol",
                t
            );
        }
    }

    /**
     * Set the HTML tag to wrap around symbols (e.g., "#" in hashtags).
     *
     * @param symbolTag the HTML tag name
     */
    public void setSymbolTag(String symbolTag) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment tagSegment = arena.allocateFrom(symbolTag);
            autolink_h
                .twitter_text_autolinker_set_symbol_tag$handle()
                .invoke(handle, tagSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set symbol tag", t);
        }
    }

    /**
     * Set the HTML tag to wrap around text after symbols.
     *
     * @param textWithSymbolTag the HTML tag name
     */
    public void setTextWithSymbolTag(String textWithSymbolTag) {
        checkNotClosed();
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment tagSegment = arena.allocateFrom(textWithSymbolTag);
            autolink_h
                .twitter_text_autolinker_set_text_with_symbol_tag$handle()
                .invoke(handle, tagSegment);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to set text with symbol tag", t);
        }
    }

    /**
     * Auto-link entities with custom display URLs.
     * TODO: Not yet implemented in FFM bindings.
     */
    public String autoLinkEntities(String text, java.util.List<?> entities) {
        throw new UnsupportedOperationException(
            "autoLinkEntities not yet implemented"
        );
    }

    /**
     * Set a callback to modify link attributes.
     * TODO: Not yet implemented in FFM bindings.
     */
    public void setLinkAttributeModifier(LinkAttributeModifier modifier) {
        throw new UnsupportedOperationException(
            "setLinkAttributeModifier not yet implemented"
        );
    }

    /**
     * Set a callback to modify link text.
     * TODO: Not yet implemented in FFM bindings.
     */
    public void setLinkTextModifier(LinkTextModifier modifier) {
        throw new UnsupportedOperationException(
            "setLinkTextModifier not yet implemented"
        );
    }

    /**
     * Callback interface for modifying link attributes.
     */
    public interface LinkAttributeModifier {
        void modify(Entity entity, java.util.Map<String, String> attributes);
    }

    /**
     * Callback interface for modifying link text.
     */
    public interface LinkTextModifier {
        CharSequence modify(Entity entity, CharSequence text);
    }

    private void checkNotClosed() {
        if (closed) {
            throw new IllegalStateException("Autolink has been closed");
        }
    }

    @Override
    public void close() {
        if (closed) return;
        closed = true;
        try {
            autolink_h.twitter_text_autolinker_free$handle().invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to free Autolink", t);
        }
    }
}
