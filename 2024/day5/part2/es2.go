package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 5 # Part 2
// / https://adventofcode.com/2024/day/5#part2
func main() {
	content, _ := os.ReadFile("day5/input.txt")
	fileAsString := string(content)
	rulesStr := strings.Split(fileAsString, "\n\n")[0]
	updatesStr := strings.Split(fileAsString, "\n\n")[1]

	rules := parseRulesMap(rulesStr)
	allUpdates := parseUpdatesSlice(updatesStr)

	validUpdates := getValidUpdates(allUpdates, rules)

	middleSum := 0

	// from the validUpdates slice, we need to find the middle element
	for i := 0; i < len(validUpdates); i++ {
		seq := validUpdates[i]
		if len(seq) == 0 {
			continue
		}
		middleValue := seq[len(seq)/2]
		middleSum += middleValue
	}

	fmt.Println(middleSum)
	// 10842 too high
	// 6042 too high
	// 5922 incorrect...
	// 5331 Correct !!!
}

func parseRulesMap(rulesStr string) map[int][]int {
	rulesMap := make(map[int][]int)
	lines := strings.Split(rulesStr, "\n")
	for _, line := range lines {
		parts := strings.Split(line, "|")
		num1, _ := strconv.Atoi(parts[0])
		num2, _ := strconv.Atoi(parts[1])
		rulesMap[num1] = append(rulesMap[num1], num2)
	}
	return rulesMap
}

func parseUpdatesSlice(updatesStr string) [][]int {
	updates := make([][]int, 0)
	lines := strings.Split(updatesStr, "\n")
	for _, line := range lines {
		parts := strings.Split(line, ",")
		line := make([]int, 0)
		for _, part := range parts {
			num, _ := strconv.Atoi(part)
			line = append(line, num)
		}
		updates = append(updates, line)
	}
	return updates
}

func getValidUpdates(updates [][]int, rules map[int][]int) [][]int {
	validUpdates := make([][]int, 0)
	for _, seq := range updates {
		if isValid(seq, rules) {
			// validUpdates = append(validUpdates, seq)
		} else {
			validUpdates = append(validUpdates, makeValid(seq, rules))
		}
	}
	return validUpdates
}

func containsAll(superset, subset []int) bool {
	// Create a map to store elements of the superset
	elementMap := make(map[int]struct{}, len(superset))

	// Fill the map with elements from the superset
	for _, val := range superset {
		elementMap[val] = struct{}{}
	}

	// Check if every element of subset exists in the map
	for _, val := range subset {
		if _, exists := elementMap[val]; !exists {
			return false // If any element is missing, return false
		}
	}

	return true // All elements found
}

func isValid(seq []int, rules map[int][]int) bool {
	for i, page := range seq {
		afterCurrentPage := seq[i:]
		mandatoryNext := sliceutils.Intersection(rules[page], seq)
		if !containsAll(afterCurrentPage, mandatoryNext) {
			return false
		}
	}
	return true
}

func makeValid(seq []int, rules map[int][]int) []int {
	if isValid(seq, rules) {
		return seq
	}
	for i, page := range seq {
		invalids := sliceutils.Intersection(sliceutils.Intersection(rules[page], seq), seq[:i])
		for _, inv := range invalids {
			indexInSeq := slices.Index(seq, inv)
			seq = slices.Delete(seq, indexInSeq, indexInSeq+1)
			seq = slices.Insert(seq, i, inv)
		}
	}
	return makeValid(seq, rules)
}
