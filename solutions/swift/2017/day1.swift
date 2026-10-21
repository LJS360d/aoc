#!/usr/bin/env swift
// https://adventofcode.com/2017/day/1
import Foundation

var input: String {
    let path = URL(fileURLWithPath: #filePath).deletingLastPathComponent()
        .appendingPathComponent("../../../inputs/2017/1.txt")
        .path
    guard let content = try? String(contentsOfFile: path, encoding: .utf8) else {
        fatalError("Could not find input file at \(path)")
    }
    return content.trimmingCharacters(in: .whitespacesAndNewlines)
}

print(input)
