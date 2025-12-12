import Foundation
import CTwitterText

/// Configuration for tweet text parsing and validation
public class Configuration {
    let handle: OpaquePointer

    /// Create a configuration from a JSON string
    public init(json: String) throws {
        guard let handle = twitter_text_config_from_json(json) else {
            throw TwitterTextError.invalidConfiguration
        }
        self.handle = handle
    }

    /// Get the default configuration (v3)
    public static var `default`: Configuration {
        return Configuration(handle: twitter_text_config_default()!)
    }

    /// Get configuration v3
    public static var v3: Configuration {
        return Configuration(handle: twitter_text_config_v3()!)
    }

    private init(handle: OpaquePointer) {
        self.handle = handle
    }

    deinit {
        twitter_text_config_free(handle)
    }

    /// Maximum weighted tweet length
    public var maxWeightedTweetLength: Int {
        return Int(twitter_text_config_get_max_weighted_tweet_length(handle))
    }

    /// Scale value for weight calculation
    public var scale: Int {
        return Int(twitter_text_config_get_scale(handle))
    }

    /// Default character weight
    public var defaultWeight: Int {
        return Int(twitter_text_config_get_default_weight(handle))
    }

    /// Transformed URL length
    public var transformedUrlLength: Int {
        return Int(twitter_text_config_get_transformed_url_length(handle))
    }

    /// Whether emoji parsing is enabled
    public var emojiParsingEnabled: Bool {
        return twitter_text_config_get_emoji_parsing_enabled(handle)
    }
}

/// Errors that can occur during Twitter text processing
public enum TwitterTextError: Error {
    case invalidConfiguration
    case invalidInput
    case allocationFailure
}
