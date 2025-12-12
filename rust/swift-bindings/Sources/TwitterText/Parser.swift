import Foundation
import CTwitterText

/// Results from parsing tweet text
public struct ParseResults {
    /// The weighted character count
    public let weightedLength: Int
    /// The length as a fraction of 1000 (for progress meters)
    public let permillage: Int
    /// Whether the tweet is valid
    public let isValid: Bool
    /// The range to display (start index)
    public let displayRangeStart: Int
    /// The range to display (end index)
    public let displayRangeEnd: Int
    /// The valid text range (start index)
    public let validRangeStart: Int
    /// The valid text range (end index)
    public let validRangeEnd: Int

    init(from results: TwitterTextParseResults) {
        self.weightedLength = Int(results.weighted_length)
        self.permillage = Int(results.permillage)
        self.isValid = results.is_valid
        self.displayRangeStart = Int(results.display_text_range.start)
        self.displayRangeEnd = Int(results.display_text_range.end)
        self.validRangeStart = Int(results.valid_text_range.start)
        self.validRangeEnd = Int(results.valid_text_range.end)
    }
}

/// Parser for tweet text
public class TwitterTextParser {
    /// Parse tweet text with the given configuration
    /// - Parameters:
    ///   - text: The tweet text to parse
    ///   - config: The configuration to use (defaults to v3)
    ///   - extractUrls: Whether to extract and weight URLs (defaults to true)
    /// - Returns: Parse results containing validity, weighted length, and ranges
    public static func parse(
        text: String,
        config: Configuration = .default,
        extractUrls: Bool = true
    ) -> ParseResults {
        let results = twitter_text_parse(text, config.handle, extractUrls)
        return ParseResults(from: results)
    }
}
