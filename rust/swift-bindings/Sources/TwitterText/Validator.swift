import Foundation
import CTwitterText

/// Validates tweets, usernames, hashtags, URLs, and lists
public class Validator {
    private let handle: OpaquePointer

    /// Create a new validator with default configuration
    public init() {
        self.handle = twitter_text_validator_new()!
    }

    /// Create a new validator with a specific configuration
    public init(configuration: Configuration) {
        self.handle = twitter_text_validator_with_config(configuration.handle)!
    }

    deinit {
        twitter_text_validator_free(handle)
    }

    /// Check if the given text is a valid tweet
    public func isValidTweet(_ text: String) -> Bool {
        return twitter_text_validator_is_valid_tweet(handle, text)
    }

    /// Check if the given username is valid (without the @ prefix)
    public func isValidUsername(_ username: String) -> Bool {
        return twitter_text_validator_is_valid_username(handle, username)
    }

    /// Check if the given list is valid (format: @username/list-name)
    public func isValidList(_ list: String) -> Bool {
        return twitter_text_validator_is_valid_list(handle, list)
    }

    /// Check if the given hashtag is valid (without the # prefix)
    public func isValidHashtag(_ hashtag: String) -> Bool {
        return twitter_text_validator_is_valid_hashtag(handle, hashtag)
    }

    /// Check if the given URL is valid
    public func isValidURL(_ url: String) -> Bool {
        return twitter_text_validator_is_valid_url(handle, url)
    }

    /// Check if the given URL is valid (without requiring a protocol)
    public func isValidURLWithoutProtocol(_ url: String) -> Bool {
        return twitter_text_validator_is_valid_url_without_protocol(handle, url)
    }

    // MARK: - Properties

    /// The maximum tweet length
    public var maxTweetLength: Int32 {
        return twitter_text_validator_get_max_tweet_length(handle)
    }

    /// The length used for short URLs (http)
    public var shortURLLength: Int32 {
        get {
            return twitter_text_validator_get_short_url_length(handle)
        }
        set {
            twitter_text_validator_set_short_url_length(handle, newValue)
        }
    }

    /// The length used for short URLs (https)
    public var shortURLLengthHTTPS: Int32 {
        get {
            return twitter_text_validator_get_short_url_length_https(handle)
        }
        set {
            twitter_text_validator_set_short_url_length_https(handle, newValue)
        }
    }
}
