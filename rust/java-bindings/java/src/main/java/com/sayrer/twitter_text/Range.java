package com.sayrer.twitter_text;

/**
 * Represents a range with start and end indices.
 */
public final class Range {
    public final int start;
    public final int end;

    public Range(int start, int end) {
        this.start = start;
        this.end = end;
    }

    @Override
    public String toString() {
        return "[" + start + ", " + end + ")";
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (!(obj instanceof Range)) return false;
        Range other = (Range) obj;
        return start == other.start && end == other.end;
    }

    @Override
    public int hashCode() {
        return 31 * start + end;
    }
}
