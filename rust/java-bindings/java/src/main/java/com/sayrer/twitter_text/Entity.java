package com.sayrer.twitter_text;

/**
 * Represents an entity found in text (URL, hashtag, mention, cashtag).
 * TODO: This is a stub implementation for test compatibility.
 */
public class Entity {
    public final int start;
    public final int end;
    public final String value;
    public final Type type;

    private String displayURL;
    private String expandedURL;

    public enum Type {
        URL,
        HASHTAG,
        MENTION,
        CASHTAG
    }

    public Entity(int start, int end, String value, Type type) {
        this.start = start;
        this.end = end;
        this.value = value;
        this.type = type;
    }

    public void setDisplayURL(String displayURL) {
        this.displayURL = displayURL;
    }

    public String getDisplayURL() {
        return displayURL;
    }

    public void setExpandedURL(String expandedURL) {
        this.expandedURL = expandedURL;
    }

    public String getExpandedURL() {
        return expandedURL;
    }
}
