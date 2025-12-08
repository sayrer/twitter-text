package com.sayrer.twitter_text;

import com.sayrer.twitter_text.autolink_h;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

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
    private Arena callbackArena;
    private MemorySegment linkTextModifierStub;
    private LinkTextModifier currentModifier;

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
     * Auto-link usernames and lists (compatibility method).
     *
     * @param text the tweet text to process
     * @return the text with auto-linked usernames and lists
     */
    public String autoLinkUsernamesAndLists(String text) {
        return autolinkUsernamesAndLists(text);
    }

    /**
     * Auto-link cashtags (compatibility method).
     *
     * @param text the tweet text to process
     * @return the text with auto-linked cashtags
     */
    public String autoLinkCashtags(String text) {
        return autolinkCashtags(text);
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
     */
    public String autoLinkEntities(String text, java.util.List<?> entities) {
        // This is a pure Java implementation that doesn't use the native autolinker
        final StringBuilder builder = new StringBuilder(text.length() * 2);
        int beginIndex = 0;

        for (Object obj : entities) {
            Entity entity = (Entity) obj;
            builder.append(text.subSequence(beginIndex, entity.start));

            switch (entity.type) {
                case URL:
                    linkToURL(entity, text, builder);
                    break;
                case HASHTAG:
                    linkToHashtag(entity, text, builder);
                    break;
                case MENTION:
                    linkToMentionAndList(entity, text, builder);
                    break;
                case CASHTAG:
                    linkToCashtag(entity, text, builder);
                    break;
                default:
                    break;
            }
            beginIndex = entity.end;
        }
        builder.append(text.subSequence(beginIndex, text.length()));

        return builder.toString();
    }

    private static final String INVISIBLE_TAG_ATTRS =
        "style='position:absolute;left:-9999px;'";

    private void linkToURL(Entity entity, String text, StringBuilder builder) {
        final CharSequence url = entity.value;
        CharSequence linkText = escapeHTML(url);

        if (entity.getDisplayURL() != null && entity.getExpandedURL() != null) {
            final String displayURLSansEllipses = entity
                .getDisplayURL()
                .replace("…", "");
            final int displayURLIndexInExpandedURL = entity
                .getExpandedURL()
                .indexOf(displayURLSansEllipses);

            if (displayURLIndexInExpandedURL != -1) {
                final String beforeDisplayURL = entity
                    .getExpandedURL()
                    .substring(0, displayURLIndexInExpandedURL);
                final String afterDisplayURL = entity
                    .getExpandedURL()
                    .substring(
                        displayURLIndexInExpandedURL +
                            displayURLSansEllipses.length()
                    );
                final String precedingEllipsis = entity
                        .getDisplayURL()
                        .startsWith("…")
                    ? "…"
                    : "";
                final String followingEllipsis = entity
                        .getDisplayURL()
                        .endsWith("…")
                    ? "…"
                    : "";
                final String invisibleSpan =
                    "<span " + INVISIBLE_TAG_ATTRS + ">";

                final StringBuilder sb = new StringBuilder(
                    "<span class='tco-ellipsis'>"
                );
                sb.append(precedingEllipsis);
                sb.append(invisibleSpan).append("&nbsp;</span></span>");
                sb
                    .append(invisibleSpan)
                    .append(escapeHTML(beforeDisplayURL))
                    .append("</span>");
                sb
                    .append("<span class='js-display-url'>")
                    .append(escapeHTML(displayURLSansEllipses))
                    .append("</span>");
                sb
                    .append(invisibleSpan)
                    .append(escapeHTML(afterDisplayURL))
                    .append("</span>");
                sb
                    .append("<span class='tco-ellipsis'>")
                    .append(invisibleSpan)
                    .append("&nbsp;</span>")
                    .append(followingEllipsis)
                    .append("</span>");

                linkText = sb;
            } else {
                linkText = entity.getDisplayURL();
            }
        }

        builder.append("<a href=\"").append(url).append("\"");
        builder.append(" rel=\"nofollow\"");
        builder.append(">").append(linkText).append("</a>");
    }

    private void linkToHashtag(
        Entity entity,
        String text,
        StringBuilder builder
    ) {
        final CharSequence hashtag = text.subSequence(entity.start, entity.end);
        builder
            .append("<a href=\"https://twitter.com/search?q=%23")
            .append(entity.value)
            .append("\" title=\"")
            .append(escapeHTML(hashtag))
            .append("\" class=\"tweet-url hashtag\" rel=\"nofollow\">")
            .append(escapeHTML(hashtag))
            .append("</a>");
    }

    private void linkToMentionAndList(
        Entity entity,
        String text,
        StringBuilder builder
    ) {
        final CharSequence mention = text.subSequence(entity.start, entity.end);
        builder
            .append(
                "@<a class=\"tweet-url username\" href=\"https://twitter.com/"
            )
            .append(entity.value)
            .append("\" rel=\"nofollow\">")
            .append(escapeHTML(mention.subSequence(1, mention.length()))) // Skip the @
            .append("</a>");
    }

    private void linkToCashtag(
        Entity entity,
        String text,
        StringBuilder builder
    ) {
        final CharSequence cashtag = text.subSequence(entity.start, entity.end);
        builder
            .append("<a href=\"https://twitter.com/search?q=%24")
            .append(entity.value)
            .append("\" title=\"")
            .append(escapeHTML(cashtag))
            .append("\" class=\"tweet-url cashtag\" rel=\"nofollow\">")
            .append(escapeHTML(cashtag))
            .append("</a>");
    }

    private String escapeHTML(CharSequence text) {
        StringBuilder result = new StringBuilder(text.length());
        for (int i = 0; i < text.length(); i++) {
            char c = text.charAt(i);
            switch (c) {
                case '&':
                    result.append("&amp;");
                    break;
                case '<':
                    result.append("&lt;");
                    break;
                case '>':
                    result.append("&gt;");
                    break;
                case '"':
                    result.append("&quot;");
                    break;
                case '\'':
                    result.append("&#39;");
                    break;
                default:
                    result.append(c);
                    break;
            }
        }
        return result.toString();
    }

    /**
     * Set a modifier to add custom attributes to links.
     *
     * @param modifier The AddAttributeModifier to use
     */
    public void setAddAttributeModifier(AddAttributeModifier modifier) {
        try {
            autolink_h
                .twitter_text_autolinker_set_add_attribute_modifier$handle()
                .invoke(handle, modifier.getHandle());
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set add attribute modifier",
                t
            );
        }
    }

    /**
     * Set a modifier to replace the class attribute.
     *
     * @param modifier The ReplaceClassModifier to use
     */
    public void setReplaceClassModifier(ReplaceClassModifier modifier) {
        try {
            autolink_h
                .twitter_text_autolinker_set_replace_class_modifier$handle()
                .invoke(handle, modifier.getHandle());
        } catch (Throwable t) {
            throw new RuntimeException(
                "Failed to set replace class modifier",
                t
            );
        }
    }

    /**
     * Set a callback to modify link attributes.
     * @deprecated Use setAddAttributeModifier or setReplaceClassModifier instead
     */
    @Deprecated
    public void setLinkAttributeModifier(LinkAttributeModifier modifier) {
        throw new UnsupportedOperationException(
            "setLinkAttributeModifier not yet implemented. Use setAddAttributeModifier or setReplaceClassModifier instead."
        );
    }

    /**
     * Set a callback to modify link text.
     */
    public void setLinkTextModifier(LinkTextModifier modifier) {
        checkNotClosed();

        // Clean up previous callback if any
        if (callbackArena != null) {
            callbackArena.close();
            callbackArena = null;
            linkTextModifierStub = null;
            currentModifier = null;
        }

        if (modifier == null) {
            return;
        }

        try {
            // Store the modifier
            currentModifier = modifier;

            // Create an arena for the callback that lives as long as this Autolink
            callbackArena = Arena.ofShared();

            // Define the callback function descriptor
            // char* callback(const CEntity* entity, const char* text, void* user_data)
            FunctionDescriptor callbackDescriptor = FunctionDescriptor.of(
                ValueLayout.ADDRESS, // return: char*
                ValueLayout.ADDRESS, // param 1: const CEntity*
                ValueLayout.ADDRESS, // param 2: const char*
                ValueLayout.ADDRESS // param 3: void* user_data
            );

            // Create a MethodHandle for the callback method
            MethodHandle callbackHandle = MethodHandles.lookup()
                .findVirtual(
                    Autolink.class,
                    "linkTextModifierCallback",
                    MethodType.methodType(
                        MemorySegment.class,
                        MemorySegment.class,
                        MemorySegment.class,
                        MemorySegment.class
                    )
                )
                .bindTo(this);

            // Create the upcall stub
            linkTextModifierStub = Linker.nativeLinker().upcallStub(
                callbackHandle,
                callbackDescriptor,
                callbackArena
            );

            // Set the callback in the native autolinker
            autolink_h.twitter_text_autolinker_set_link_text_modifier(
                handle,
                linkTextModifierStub,
                MemorySegment.NULL
            );
        } catch (Throwable t) {
            if (callbackArena != null) {
                callbackArena.close();
                callbackArena = null;
                linkTextModifierStub = null;
                currentModifier = null;
            }
            throw new RuntimeException("Failed to set link text modifier", t);
        }
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

    // Private callback method for LinkTextModifier - called from native code
    private MemorySegment linkTextModifierCallback(
        MemorySegment entityPtr,
        MemorySegment textPtr,
        MemorySegment userData
    ) {
        try {
            System.err.println("DEBUG Java: linkTextModifierCallback entered");

            // The entityPtr has byteSize 0, so we need to reinterpret it with the correct size
            // CEntity struct has: int32_t entity_type (4 bytes) + int32_t start (4 bytes) + int32_t end (4 bytes) = 12 bytes
            MemorySegment entitySegment = entityPtr.reinterpret(12);

            // Read entity type from CEntity struct
            int entityTypeValue = entitySegment.get(ValueLayout.JAVA_INT, 0);
            Entity.Type entityType = Entity.Type.values()[entityTypeValue];
            System.err.println("DEBUG Java: entityType=" + entityType);

            // Read start and end from CEntity struct
            int start = entitySegment.get(ValueLayout.JAVA_INT, 4);
            int end = entitySegment.get(ValueLayout.JAVA_INT, 8);

            // Read the text - reinterpret as unbounded since we don't know the C string length
            String text = textPtr.reinterpret(Long.MAX_VALUE).getString(0);
            System.err.println("DEBUG Java: input text=" + text);

            // Create Entity object (value is the text itself)
            Entity entity = new Entity(start, end, text, entityType);

            // Call the Java modifier
            System.err.println("DEBUG Java: calling currentModifier.modify");
            CharSequence modifiedText = currentModifier.modify(entity, text);
            System.err.println("DEBUG Java: modifiedText=" + modifiedText);

            // Allocate and return the modified text as a C string
            // Use Arena.ofAuto() so the memory will be freed by C code, not by our arena
            MemorySegment result = Arena.ofAuto().allocateFrom(
                modifiedText.toString()
            );
            System.err.println("DEBUG Java: returning result");
            return result;
        } catch (Throwable t) {
            System.err.println(
                "DEBUG Java: Exception caught: " + t.getMessage()
            );
            t.printStackTrace(System.err);
            // On error, return the original text
            return textPtr;
        }
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

        // Clean up callback arena if it exists
        if (callbackArena != null) {
            callbackArena.close();
            callbackArena = null;
            linkTextModifierStub = null;
        }

        try {
            autolink_h.twitter_text_autolinker_free$handle().invoke(handle);
        } catch (Throwable t) {
            throw new RuntimeException("Failed to free Autolink", t);
        }
    }
}
