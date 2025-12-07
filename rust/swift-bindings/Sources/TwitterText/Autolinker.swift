import Foundation
import CTwitterText

/// Automatically links URLs, mentions, hashtags, and cashtags in tweet text
public class Autolinker {
    private let handle: OpaquePointer

    /// Create a new autolinker
    /// - Parameter noFollow: Whether to add rel="nofollow" to links
    public init(noFollow: Bool = true) {
        self.handle = twitter_text_autolinker_new(noFollow)!
    }

    deinit {
        twitter_text_autolinker_free(handle)
    }

    /// Autolink all entities (URLs, mentions, hashtags, cashtags) in the text
    public func autolink(_ text: String) -> String {
        guard let resultPtr = twitter_text_autolinker_autolink(handle, text) else {
            return text
        }

        let result = String(cString: resultPtr)
        twitter_text_string_free(resultPtr)
        return result
    }

    /// Autolink only URLs in the text
    public func autolinkURLs(_ text: String) -> String {
        guard let resultPtr = twitter_text_autolinker_autolink_urls(handle, text) else {
            return text
        }

        let result = String(cString: resultPtr)
        twitter_text_string_free(resultPtr)
        return result
    }

    /// Autolink only usernames and lists in the text
    public func autolinkUsernamesAndLists(_ text: String) -> String {
        guard let resultPtr = twitter_text_autolinker_autolink_usernames_and_lists(handle, text) else {
            return text
        }

        let result = String(cString: resultPtr)
        twitter_text_string_free(resultPtr)
        return result
    }

    /// Autolink only hashtags in the text
    public func autolinkHashtags(_ text: String) -> String {
        guard let resultPtr = twitter_text_autolinker_autolink_hashtags(handle, text) else {
            return text
        }

        let result = String(cString: resultPtr)
        twitter_text_string_free(resultPtr)
        return result
    }

    /// Autolink only cashtags in the text
    public func autolinkCashtags(_ text: String) -> String {
        guard let resultPtr = twitter_text_autolinker_autolink_cashtags(handle, text) else {
            return text
        }

        let result = String(cString: resultPtr)
        twitter_text_string_free(resultPtr)
        return result
    }

    // MARK: - Configuration Properties

    /// Set the URL class attribute
    public func setURLClass(_ className: String) {
        twitter_text_autolinker_set_url_class(handle, className)
    }

    /// Set the URL target attribute
    public func setURLTarget(_ target: String) {
        twitter_text_autolinker_set_url_target(handle, target)
    }

    /// Set the symbol tag
    public func setSymbolTag(_ tag: String) {
        twitter_text_autolinker_set_symbol_tag(handle, tag)
    }

    /// Set the text with symbol tag
    public func setTextWithSymbolTag(_ tag: String) {
        twitter_text_autolinker_set_text_with_symbol_tag(handle, tag)
    }

    /// Set the list class attribute
    public func setListClass(_ className: String) {
        twitter_text_autolinker_set_list_class(handle, className)
    }

    /// Set the username class attribute
    public func setUsernameClass(_ className: String) {
        twitter_text_autolinker_set_username_class(handle, className)
    }

    /// Set the hashtag class attribute
    public func setHashtagClass(_ className: String) {
        twitter_text_autolinker_set_hashtag_class(handle, className)
    }

    /// Set the cashtag class attribute
    public func setCashtagClass(_ className: String) {
        twitter_text_autolinker_set_cashtag_class(handle, className)
    }

    /// Set the username URL base
    public func setUsernameURLBase(_ base: String) {
        twitter_text_autolinker_set_username_url_base(handle, base)
    }

    /// Set the list URL base
    public func setListURLBase(_ base: String) {
        twitter_text_autolinker_set_list_url_base(handle, base)
    }

    /// Set the hashtag URL base
    public func setHashtagURLBase(_ base: String) {
        twitter_text_autolinker_set_hashtag_url_base(handle, base)
    }

    /// Set the cashtag URL base
    public func setCashtagURLBase(_ base: String) {
        twitter_text_autolinker_set_cashtag_url_base(handle, base)
    }

    /// Set the invisible tag attributes
    public func setInvisibleTagAttrs(_ attrs: String) {
        twitter_text_autolinker_set_invisible_tag_attrs(handle, attrs)
    }

    /// Set whether to include the @ symbol in username links
    public func setUsernameIncludeSymbol(_ include: Bool) {
        twitter_text_autolinker_set_username_include_symbol(handle, include)
    }

    /// Set whether to add nofollow attribute
    public func setNoFollow(_ noFollow: Bool) {
        twitter_text_autolinker_set_no_follow(handle, noFollow)
    }
}
