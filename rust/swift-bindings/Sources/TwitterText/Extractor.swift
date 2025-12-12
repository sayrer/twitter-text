import Foundation
import CTwitterText

/// Extracts entities (URLs, mentions, hashtags, cashtags) from tweet text
public class Extractor {
    private let handle: OpaquePointer

    /// Create a new extractor
    public init() {
        self.handle = twitter_text_extractor_new()!
    }

    deinit {
        twitter_text_extractor_free(handle)
    }

    /// Whether to extract URLs without protocol (e.g., "example.com")
    public var extractUrlWithoutProtocol: Bool {
        get {
            return twitter_text_extractor_get_extract_url_without_protocol(handle)
        }
        set {
            twitter_text_extractor_set_extract_url_without_protocol(handle, newValue)
        }
    }

    /// Extract URLs from the text
    public func extractURLs(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_urls_with_indices(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    /// Extract hashtags from the text
    public func extractHashtags(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_hashtags_with_indices(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    /// Extract mentions from the text
    public func extractMentions(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_mentioned_screennames_with_indices(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    /// Extract cashtags from the text
    public func extractCashtags(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_cashtags_with_indices(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    /// Extract the reply username from the text
    public func extractReplyUsername(from text: String) -> Entity? {
        guard let entityPtr = twitter_text_extractor_extract_reply_username(handle, text) else {
            return nil
        }

        let entity = Entity(from: entityPtr.pointee)
        twitter_text_entity_free(entityPtr)
        return entity
    }

    /// Extract federated mentions (Mastodon-style @user@domain.tld) from the text
    /// Note: This also includes regular mentions for Mastodon compatibility
    public func extractFederatedMentions(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_federated_mentions_with_indices(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    /// Extract all entities including federated mentions from the text
    public func extractEntitiesWithIndicesFederated(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_entities_with_indices_federated(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    /// Extract all entities (URLs, mentions, hashtags, cashtags) excluding federated mentions
    public func extractEntities(from text: String) -> [Entity] {
        let entitiesArray = twitter_text_extractor_extract_entities_with_indices(handle, text)
        return extractEntitiesFromArray(entitiesArray)
    }

    private func extractEntitiesFromArray(_ entitiesArray: TwitterTextEntityArray) -> [Entity] {
        var result: [Entity] = []

        if let array = entitiesArray.entities {
            for i in 0..<Int(entitiesArray.length) {
                let entity = Entity(from: array[i])
                result.append(entity)
            }
        }

        twitter_text_entity_array_free(entitiesArray)
        return result
    }
}
