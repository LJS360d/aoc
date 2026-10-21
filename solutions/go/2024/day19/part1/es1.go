package main

import (
	"fmt"
	"os"
	"strings"
)

// / Advent of Code 2024 - Day 19 # Part 1
// / https://adventofcode.com/2024/day/19#part1
func main() {
	buffer, _ := os.ReadFile("day19/input.txt")
	content := string(buffer)
	towels, designs := parse(content)
	res := 0
	for _, d := range designs {
		if isDesignPossible(towels, d, map[string]bool{}) {
			res++
		}
	}
	fmt.Println(res)
}

func parse(content string) ([]string, []string) {
	sections := strings.Split(content, "\n\n")
	towels := strings.Split(sections[0], ", ")
	designs := strings.Split(sections[1], "\n")
	return towels, designs
}

func isDesignPossible(words []string, target string, memo map[string]bool) bool {
	if val, found := memo[target]; found {
		return val
	}
	if target == "" {
		return true
	}

	for _, word := range words {
		// Check if the target starts with the current word
		if len(target) >= len(word) && target[:len(word)] == word {
			// Recursively check the rest of the target
			rest := target[len(word):]
			if isDesignPossible(words, rest, memo) {
				memo[target] = true
				return true
			}
		}
	}
	memo[target] = false
	return false
}
