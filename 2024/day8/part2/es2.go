package main

import (
	"fmt"
	"os"
	"strings"
)

// / Advent of Code 2024 - Day 8 # Part 2
// / https://adventofcode.com/2024/day/8#part2
func main() {
	buffer, _ := os.ReadFile("day8/input.txt")
	content := string(buffer)
	lines := strings.Split(content, "\n")
	fmt.Println(lines)
}
