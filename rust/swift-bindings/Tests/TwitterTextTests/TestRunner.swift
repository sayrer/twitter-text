import Foundation

// Test runner for Linux (XCTest not usable via Bazel on Linux)
// On macOS, swift_test uses XCTest directly

#if os(Linux)

var totalPassed = 0
var totalFailed = 0

func runTests() {
    print("Twitter Text Swift Conformance Tests")
    print("=====================================")

    let autolinkTests = AutolinkTests()
    autolinkTests.run()

    let extractTests = ExtractTests()
    extractTests.run()

    let validateTests = ValidateTests()
    validateTests.run()

    let hitHighlightTests = HitHighlightTests()
    hitHighlightTests.run()

    let tldTests = TldTests()
    tldTests.run()

    let emojiTests = EmojiTests()
    emojiTests.run()

    print("\n=====================================")
    print("Results: \(totalPassed) passed, \(totalFailed) failed")

    if totalFailed > 0 {
        exit(1)
    } else {
        print("All tests passed!")
    }
}

// Custom assertion helpers for Linux

func XCTAssertEqual<T: Equatable>(_ lhs: T, _ rhs: T, _ message: String = "", file: String = #file, line: Int = #line) {
    if lhs != rhs {
        print("  FAILED: \(message)")
        print("    Expected: \(rhs)")
        print("    Got:      \(lhs)")
        totalFailed += 1
    } else {
        totalPassed += 1
    }
}

func XCTAssertTrue(_ condition: Bool, _ message: String = "", file: String = #file, line: Int = #line) {
    if !condition {
        print("  FAILED: \(message)")
        totalFailed += 1
    } else {
        totalPassed += 1
    }
}

func XCTAssertFalse(_ condition: Bool, _ message: String = "", file: String = #file, line: Int = #line) {
    XCTAssertTrue(!condition, message, file: file, line: line)
}

func XCTAssertNotNil<T>(_ value: T?, _ message: String = "", file: String = #file, line: Int = #line) {
    if value == nil {
        print("  FAILED: \(message)")
        totalFailed += 1
    } else {
        totalPassed += 1
    }
}

func XCTAssertNil<T>(_ value: T?, _ message: String = "", file: String = #file, line: Int = #line) {
    if value != nil {
        print("  FAILED: \(message)")
        totalFailed += 1
    } else {
        totalPassed += 1
    }
}

func XCTAssertGreaterThan<T: Comparable>(_ lhs: T, _ rhs: T, _ message: String = "", file: String = #file, line: Int = #line) {
    if !(lhs > rhs) {
        print("  FAILED: \(message) - \(lhs) is not greater than \(rhs)")
        totalFailed += 1
    } else {
        totalPassed += 1
    }
}

func XCTFail(_ message: String = "", file: String = #file, line: Int = #line) {
    print("  FAILED: \(message)")
    totalFailed += 1
}

@main
struct Main {
    static func main() {
        runTests()
    }
}

#endif
