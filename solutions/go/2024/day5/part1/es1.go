package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 5 # Part 1
// / https://adventofcode.com/2024/day/5
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
		middleValue := seq[len(seq)/2]
		middleSum += middleValue
	}

	fmt.Println(middleSum)
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
		for i, page := range seq {
			afterCurrentPage := seq[i:]
			allNext := rules[page]
			mandatoryNext := sliceutils.Intersection(allNext, seq)
			if !containsAll(afterCurrentPage, mandatoryNext) {
				// this sequence is not valid
				break
			}
			if i == len(seq)-1 {
				validUpdates = append(validUpdates, seq)
			}
		}
	}
	return validUpdates
}

func containsAll[T comparable](superset, subset []T) bool {
	// Create a map to store elements of the superset
	elementMap := make(map[T]struct{}, len(superset))

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
