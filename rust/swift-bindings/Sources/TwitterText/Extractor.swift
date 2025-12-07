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
            return twitter_text_extractor_extract_url_without_protocol(handle)
        }
        set {
            twitter_text_extractor_set_extract_url_without_protocol(handle, newValue)
        }
    }

    /// Extract all entities from the text
    public func extractEntities(from text: String) -> [Entity] {
        let entitiesPtr = twitter_text_extractor_extract_entities_with_indices(handle, text)
        return extractEntitiesFromPointer(entitiesPtr)
    }

    /// Extract URLs from the text
    public func extractURLs(from text: String) -> [Entity] {
        let entitiesPtr = twitter_text_extractor_extract_urls_with_indices(handle, text)
        return extractEntitiesFromPointer(entitiesPtr)
    }

    /// Extract hashtags from the text
    public func extractHashtags(from text: String) -> [Entity] {
        let entitiesPtr = twitter_text_extractor_extract_hashtags_with_indices(handle, text)
        return extractEntitiesFromPointer(entitiesPtr)
    }

    /// Extract mentions from the text
    public func extractMentions(from text: String) -> [Entity] {
        let entitiesPtr = twitter_text_extractor_extract_mentioned_screennames_with_indices(handle, text)
        return extractEntitiesFromPointer(entitiesPtr)
    }

    /// Extract mentions or lists from the text
    public func extractMentionsOrLists(from text: String) -> [Entity] {
        let entitiesPtr = twitter_text_extractor_extract_mentions_or_lists_with_indices(handle, text)
        return extractEntitiesFromPointer(entitiesPtr)
    }

    /// Extract cashtags from the text
    public func extractCashtags(from text: String) -> [Entity] {
        let entitiesPtr = twitter_text_extractor_extract_cashtags_with_indices(handle, text)
        return extractEntitiesFromPointer(entitiesPtr)
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

    private func extractEntitiesFromPointer(_ entitiesPtr: UnsafeMutablePointer<TwitterTextEntities>?) -> [Entity] {
        guard let entitiesPtr = entitiesPtr else {
            return []
        }

        let entities = entitiesPtr.pointee
        var result: [Entity] = []

        if let array = entities.entities {
            for i in 0..<Int(entities.count) {
                let entity = Entity(from: array[i])
                result.append(entity)
            }
        }

        twitter_text_entities_free(entitiesPtr)
        return result
    }
}
