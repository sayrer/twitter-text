// Copyright 2024 Robert Sayre
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

import Foundation
import Yams

@objc public class YAMLHelper: NSObject {

    @objc public static func loadYAML(fromPath path: String) -> NSDictionary? {
        guard let contents = try? String(contentsOfFile: path, encoding: .utf8),
              let yaml = try? Yams.load(yaml: contents) as? [String: Any] else {
            return nil
        }
        return yaml as NSDictionary
    }

    @objc public static func loadYAML(fromString string: String) -> NSDictionary? {
        guard let yaml = try? Yams.load(yaml: string) as? [String: Any] else {
            return nil
        }
        return yaml as NSDictionary
    }
}
