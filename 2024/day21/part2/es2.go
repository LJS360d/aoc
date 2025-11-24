package main

import (
	"fmt"
	"os"
	"strings"
)

/// Advent of Code 2024 - Day 21 # Part 2
/// https://adventofcode.com/2024/day/21#part2
func main() {
	buffer, _ := os.ReadFile("day21/input.txt")
	content := string(buffer)
	lines := strings.Split(content, "\n")
	fmt.Println(lines)
}
