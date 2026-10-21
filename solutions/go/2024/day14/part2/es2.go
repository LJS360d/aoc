package main

import (
	"fmt"
	"os"
	"strings"
)

/// Advent of Code 2024 - Day 14 # Part 2
/// https://adventofcode.com/2024/day/14#part2
func main() {
	buffer, _ := os.ReadFile("day14/input.txt")
	content := string(buffer)
	lines := strings.Split(content, "\n")
	fmt.Println(lines)
}
