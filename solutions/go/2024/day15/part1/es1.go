package main

import (
	"fmt"
	"os"
	"strings"
)

/// Advent of Code 2024 - Day 15 # Part 1
/// https://adventofcode.com/2024/day/15#part1
func main() {
	buffer, _ := os.ReadFile("day15/input.txt")
	content := string(buffer)
	lines := strings.Split(content, "\n")
	fmt.Println(lines)
}
