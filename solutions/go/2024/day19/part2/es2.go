package main

import (
	"fmt"
	"os"
	"strings"
)

// / Advent of Code 2024 - Day 19 # Part 2
// / https://adventofcode.com/2024/day/19#part2
func main() {
	buffer, _ := os.ReadFile("day19/input.txt")
	content := string(buffer)
	towels, designs := parse(content)
	res := 0
	for _, d := range designs {
		// results := [][]string{}
		// findAllWays(d, towels, []string{}, &results)
		// res += len(results)
		res += countWays(d, towels)
	}
	fmt.Println(res)
}

func parse(content string) ([]string, []string) {
	sections := strings.Split(content, "\n\n")
	towels := strings.Split(sections[0], ", ")
	designs := strings.Split(sections[1], "\n")
	return towels, designs
}

func findAllWays(target string, words []string, current []string, results *[][]string) {
	if target == "" {
		*results = append(*results, append([]string{}, current...))
		return
	}

	for _, word := range words {
		if len(target) >= len(word) && target[:len(word)] == word {
			findAllWays(target[len(word):], words, append(current, word), results)
		}
	}
}

func countWays(target string, words []string) int {
	n := len(target)
	dp := make([]int, n+1)
	dp[0] = 1

	for i := 1; i <= n; i++ {
		for _, word := range words {
			wordLen := len(word)
			if i >= wordLen && target[i-wordLen:i] == word {
				dp[i] += dp[i-wordLen]
			}
		}
	}

	return dp[n]
}
