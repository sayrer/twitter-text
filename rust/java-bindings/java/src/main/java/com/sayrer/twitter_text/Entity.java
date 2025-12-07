package com.sayrer.twitter_text;

import java.util.regex.Matcher;

/**
 * Represents an entity found in text (URL, hashtag, mention, cashtag).
 */
public class Entity {
    public int start;
    public int end;
    public String value;
    public final Type type;

    private String displayURL;
    private String expandedURL;
    private String listSlug;

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

    /**
     * Constructor for creating an entity from a Matcher.
     * Used by tests to create entities from regex matches.
     */
    public Entity(Matcher matcher, Type type, int groupNumber, int groupNumber2) {
        this.start = matcher.start();
        this.end = matcher.end();
        this.value = matcher.group();
        this.type = type;
    }

    public Integer getStart() {
        return start;
    }

    public Integer getEnd() {
        return end;
    }

    public String getValue() {
        return value;
    }

    public Type getType() {
        return type;
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

    public void setListSlug(String listSlug) {
        this.listSlug = listSlug;
    }

    public String getListSlug() {
        return listSlug;
    }

    @Override
    public String toString() {
        return type + "(" + value + " [" + start + "," + end + "])";
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (!(obj instanceof Entity)) return false;

        Entity other = (Entity) obj;
        return start == other.start &&
               end == other.end &&
               type == other.type &&
               (value == null ? other.value == null : value.equals(other.value));
    }

    @Override
    public int hashCode() {
        int result = start;
        result = 31 * result + end;
        result = 31 * result + (value != null ? value.hashCode() : 0);
        result = 31 * result + type.hashCode();
        return result;
    }
}
