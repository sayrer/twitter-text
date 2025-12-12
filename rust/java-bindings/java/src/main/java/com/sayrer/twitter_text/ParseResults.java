package com.sayrer.twitter_text;

/**
 * Represents the results of parsing tweet text.
 * Contains information about weighted length, validity, and display ranges.
 */
public final class ParseResults {
    public final int weightedLength;
    public final int permillage;
    public final boolean isValid;
    public final Range displayTextRange;
    public final Range validTextRange;

    public ParseResults(
        int weightedLength,
        int permillage,
        boolean isValid,
        Range displayTextRange,
        Range validTextRange
    ) {
        this.weightedLength = weightedLength;
        this.permillage = permillage;
        this.isValid = isValid;
        this.displayTextRange = displayTextRange;
        this.validTextRange = validTextRange;
    }

    @Override
    public String toString() {
        return String.format(
            "ParseResults{weightedLength=%d, permillage=%d, isValid=%s, " +
            "displayTextRange=%s, validTextRange=%s}",
            weightedLength,
            permillage,
            isValid,
            displayTextRange,
            validTextRange
        );
    }
}
