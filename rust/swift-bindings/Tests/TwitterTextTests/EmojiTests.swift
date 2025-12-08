import Foundation
import TwitterText

#if os(macOS)
import XCTest

final class EmojiTests: XCTestCase {
    var extractor: Extractor!

    override func setUp() {
        super.setUp()
        extractor = Extractor()
    }

    func testEmojiParsing() {
        guard let contents = loadTextFile(from: "unicode11-emoji-test.txt") else {
            XCTFail("Could not load unicode11-emoji-test.txt")
            return
        }

        var emojiCount = 0
        var failures = 0

        let lines = contents.components(separatedBy: .newlines)
        for line in lines {
            let trimmed = line.trimmingCharacters(in: .whitespaces)
            if trimmed.isEmpty || trimmed.hasPrefix("#") {
                continue
            }

            guard let semicolonIndex = line.firstIndex(of: ";") else {
                continue
            }

            let codePointsStr = String(line[..<semicolonIndex]).trimmingCharacters(in: .whitespaces)
            let codePoints = codePointsStr.components(separatedBy: " ")

            var emojiString = ""
            for codePoint in codePoints {
                if let value = UInt32(codePoint, radix: 16),
                   let scalar = Unicode.Scalar(value) {
                    emojiString.append(Character(scalar))
                }
            }

            if emojiString.isEmpty {
                continue
            }

            let testString = "test|\(emojiString)|test"

            let mentions = extractor.extractMentions(from: testString)
            let hashtags = extractor.extractHashtags(from: testString)
            let cashtags = extractor.extractCashtags(from: testString)

            if !mentions.isEmpty || !hashtags.isEmpty || !cashtags.isEmpty {
                failures += 1
            }
            emojiCount += 1
        }

        XCTAssertEqual(failures, 0, "\(failures) emoji sequences incorrectly parsed as entities")
    }
}

#else

class EmojiTests {
    var extractor: Extractor!

    func setUp() {
        extractor = Extractor()
    }

    func run() {
        print("\n=== Emoji Parsing Tests ===")
        setUp()
        testEmojiParsing()
    }

    func testEmojiParsing() {
        guard let contents = loadTextFile(from: "unicode11-emoji-test.txt") else {
            XCTFail("Could not load unicode11-emoji-test.txt")
            return
        }

        var emojiCount = 0
        var failures = 0

        let lines = contents.components(separatedBy: .newlines)
        for line in lines {
            let trimmed = line.trimmingCharacters(in: .whitespaces)
            if trimmed.isEmpty || trimmed.hasPrefix("#") {
                continue
            }

            guard let semicolonIndex = line.firstIndex(of: ";") else {
                continue
            }

            let codePointsStr = String(line[..<semicolonIndex]).trimmingCharacters(in: .whitespaces)
            let codePoints = codePointsStr.components(separatedBy: " ")

            var emojiString = ""
            for codePoint in codePoints {
                if let value = UInt32(codePoint, radix: 16),
                   let scalar = Unicode.Scalar(value) {
                    emojiString.append(Character(scalar))
                }
            }

            if emojiString.isEmpty {
                continue
            }

            let testString = "test|\(emojiString)|test"

            let mentions = extractor.extractMentions(from: testString)
            let hashtags = extractor.extractHashtags(from: testString)
            let cashtags = extractor.extractCashtags(from: testString)

            if !mentions.isEmpty || !hashtags.isEmpty || !cashtags.isEmpty {
                failures += 1
            }
            emojiCount += 1
        }

        XCTAssertEqual(failures, 0, "\(failures) emoji sequences incorrectly parsed as entities")
        print("Tested \(emojiCount) emoji sequences")
    }
}

#endif
