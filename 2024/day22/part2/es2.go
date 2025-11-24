package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 22 # Part 2
// / https://adventofcode.com/2024/day/22#part2
func main() {
	buffer, _ := os.ReadFile("day22/input.txt")
	content := string(buffer)
	nums := parse(content)
	secretsList := make([][]int, 0)
	changesList := make([][]int, 0)
	for _, n := range nums {
		secretsSeq := getSecretsSequence(n, 2000)
		changesSeq := getChangesSequence(n, secretsSeq)
		secretsList = append(secretsList, secretsSeq)
		changesList = append(changesList, changesSeq)
	}
	findBestSequence(secretsList, changesList)
}

func parse(content string) []int64 {
	lines := strings.Split(content, "\n")
	numbers := make([]int64, len(lines))
	for i := range lines {
		numbers[i], _ = strconv.ParseInt(lines[i], 10, 64)
	}
	return numbers
}

func getSecretsSequence(num int64, i int) []int {
	pruneNum := int64(16777216)
	seq := make([]int, 0)
	for ; i > 0; i-- {
		num = ((num << 6) ^ num) & (pruneNum - 1)
		num = ((num >> 5) ^ num) & (pruneNum - 1)
		num = ((num << 11) ^ num) & (pruneNum - 1)
		lastDigit := num % 10
		seq = append(seq, int(lastDigit))
	}
	return seq
}

func getChangesSequence(num int64, secrets []int) []int {
	lastDigit := num % 10
	changesSeq := make([]int, 0)
	for i, sec := range secrets {
		if i == 0 {
			changesSeq = append(changesSeq, sec-int(lastDigit))
		} else {
			changesSeq = append(changesSeq, sec-secrets[i-1])
		}
	}
	return changesSeq
}

func findBestSequence(pricesList [][]int, changesList [][]int) ([]int, int) {
	sequenceBananaMap := make(map[string]int)
	var bestSequence []int
	maxBananas := 0

	// Generate sequences from all change slices
	for _, changes := range changesList {
		sequences := generateSequences(changes)
		for _, sequence := range sequences {
			seqKey := fmt.Sprintf("%v", sequence) // Use a string key for map
			if _, exists := sequenceBananaMap[seqKey]; !exists {
				// Simulate buying for this sequence
				totalBananas := 0
				for i := range pricesList {
					totalBananas += simulateBuying(pricesList[i], changesList[i], sequence)
				}
				sequenceBananaMap[seqKey] = totalBananas
				if totalBananas > maxBananas {
					maxBananas = totalBananas
					bestSequence = sequence
				}
			}
		}
	}
	return bestSequence, maxBananas
}

func simulateBuying(prices []int, changes []int, sequence []int) int {
	bananas := 0
	for i := 0; i <= len(changes)-4; i++ {
		if equal(changes[i:i+4], sequence) {
			// Buy at the price corresponding to the 4th change
			bananas += prices[i+3]
		}
	}
	return bananas
}

// Helper function to check if two slices are equal
func equal(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

func generateSequences(changes []int) [][]int {
	var sequences [][]int
	if len(changes) < 4 {
		return sequences // No valid sequences
	}
	for i := 0; i <= len(changes)-4; i++ {
		sequences = append(sequences, changes[i:i+4])
	}
	return sequences
}
