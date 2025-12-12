// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "TwitterText",
    platforms: [
        .macOS(.v10_15),
        .iOS(.v13),
    ],
    products: [
        .library(
            name: "TwitterText",
            targets: ["TwitterText"]
        ),
    ],
    targets: [
        // C module that wraps the Rust FFI
        .target(
            name: "CTwitterText",
            path: "Sources/CTwitterText",
            exclude: ["module.modulemap"],
            publicHeadersPath: "include"
        ),
        // Swift wrapper library
        .target(
            name: "TwitterText",
            dependencies: ["CTwitterText"],
            path: "Sources/TwitterText"
        ),
        // Tests
        .testTarget(
            name: "TwitterTextTests",
            dependencies: ["TwitterText"],
            path: "Tests/TwitterTextTests"
        ),
    ]
)
