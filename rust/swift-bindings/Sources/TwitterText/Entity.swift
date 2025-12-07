import Foundation
import CTwitterText

/// Type of entity found in tweet text
public enum EntityType: Int32 {
    case url = 0
    case hashtag = 1
    case mention = 2
    case cashtag = 3
}

/// An entity extracted from tweet text
public struct Entity {
    /// The type of entity
    public let type: EntityType

    /// Start index in the text
    public let start: Int

    /// End index in the text
    public let end: Int

    /// The extracted value (e.g., the URL, hashtag text, username)
    public let value: String

    /// For lists: the list slug
    public let listSlug: String?

    /// For URLs: the display URL
    public let displayUrl: String?

    /// For URLs: the expanded URL
    public let expandedUrl: String?

    internal init(from cEntity: TwitterTextEntity) {
        self.type = EntityType(rawValue: cEntity.entity_type) ?? .url
        self.start = Int(cEntity.start)
        self.end = Int(cEntity.end)

        if let valuePtr = cEntity.value {
            self.value = String(cString: valuePtr)
        } else {
            self.value = ""
        }

        if let slugPtr = cEntity.list_slug {
            self.listSlug = String(cString: slugPtr)
        } else {
            self.listSlug = nil
        }

        if let displayPtr = cEntity.display_url {
            self.displayUrl = String(cString: displayPtr)
        } else {
            self.displayUrl = nil
        }

        if let expandedPtr = cEntity.expanded_url {
            self.expandedUrl = String(cString: expandedPtr)
        } else {
            self.expandedUrl = nil
        }
    }

    /// The range of the entity in the text
    public var range: Range<Int> {
        return start..<end
    }
}
