import Foundation
import CTwitterText

/// Represents a range to highlight in text
public struct Hit {
    public let start: Int
    public let end: Int

    public init(start: Int, end: Int) {
        self.start = start
        self.end = end
    }
}

/// Highlights search hits in tweet text
public class HitHighlighter {
    private let handle: OpaquePointer

    /// Create a new hit highlighter with default tag ("em")
    public init() {
        self.handle = twitter_text_hit_highlighter_new()!
    }

    /// Create a new hit highlighter with a custom tag
    public init(tag: String) {
        self.handle = twitter_text_hit_highlighter_new_with_tag(tag)!
    }

    deinit {
        twitter_text_hit_highlighter_free(handle)
    }

    /// Set the highlight tag
    public func setTag(_ tag: String) {
        twitter_text_hit_highlighter_set_tag(handle, tag)
    }

    /// Highlight the given text with the provided hit ranges
    /// - Parameters:
    ///   - text: The text to highlight
    ///   - hits: Array of Hit ranges to highlight
    /// - Returns: HTML string with highlight tags around the hit ranges
    public func highlight(_ text: String, hits: [Hit]) -> String {
        guard !hits.isEmpty else {
            return text
        }

        // Convert Swift Hits to C TwitterTextHit array
        var cHits = hits.map { hit in
            TwitterTextHit(start: hit.start, end: hit.end)
        }

        guard let resultPtr = cHits.withUnsafeMutableBufferPointer({ buffer in
            twitter_text_hit_highlighter_highlight(
                handle,
                text,
                buffer.baseAddress,
                buffer.count
            )
        }) else {
            return text
        }

        let result = String(cString: resultPtr)
        twitter_text_string_free(resultPtr)
        return result
    }
}
