import Foundation
import CTwitterText

/// Configuration for tweet text parsing and validation
public class Configuration {
    let handle: OpaquePointer

    /// Create a configuration from a JSON string
    public init(json: String) throws {
        guard let handle = twitter_text_configuration_from_json(json) else {
            throw TwitterTextError.invalidConfiguration
        }
        self.handle = handle
    }

    /// Create a configuration from a JSON file path
    public init(path: String) throws {
        guard let handle = twitter_text_configuration_from_path(path) else {
            throw TwitterTextError.invalidConfiguration
        }
        self.handle = handle
    }

    /// Get the default configuration (v3)
    public static var `default`: Configuration {
        return Configuration(handle: twitter_text_default_config()!)
    }

    /// Get configuration v1
    public static var v1: Configuration {
        return Configuration(handle: twitter_text_config_v1()!)
    }

    /// Get configuration v2
    public static var v2: Configuration {
        return Configuration(handle: twitter_text_config_v2()!)
    }

    /// Get configuration v3
    public static var v3: Configuration {
        return Configuration(handle: twitter_text_config_v3()!)
    }

    private init(handle: OpaquePointer) {
        self.handle = handle
    }

    deinit {
        twitter_text_configuration_free(handle)
    }

    /// Maximum weighted tweet length
    public var maxWeightedTweetLength: Int {
        return Int(twitter_text_configuration_max_weighted_tweet_length(handle))
    }

    /// Scale value for weight calculation
    public var scale: Int {
        return Int(twitter_text_configuration_scale(handle))
    }

    /// Default character weight
    public var defaultWeight: Int {
        return Int(twitter_text_configuration_default_weight(handle))
    }

    /// Transformed URL length
    public var transformedUrlLength: Int {
        return Int(twitter_text_configuration_transformed_url_length(handle))
    }

    /// Whether emoji parsing is enabled
    public var emojiParsingEnabled: Bool {
        return twitter_text_configuration_emoji_parsing_enabled(handle)
    }
}

/// Errors that can occur during Twitter text processing
public enum TwitterTextError: Error {
    case invalidConfiguration
    case invalidInput
    case allocationFailure
}
