package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 2 # Part 2
// / https://adventofcode.com/2024/day/2#part2
func main() {
	input, err := os.Open("day2/input.txt")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	reports := make([][]int, 0)
	for scanner.Scan() {
		line := scanner.Text()
		nums := parseLine(line)
		reports = append(reports, nums)
	}
	safe := 0

	for _, report := range reports {
		if isSafeDampened(report) {
			safe++
		}
	}

	fmt.Println(safe)
}

func parseLine(line string) []int {
	var numbers []int
	for _, number := range strings.Split(line, " ") {
		num, err := strconv.Atoi(number)
		if err != nil {
			panic(err)
		}
		numbers = append(numbers, num)
	}
	return numbers

}

func isSafe(report []int) bool {
	// Conditions for safety:
	// The levels are either all increasing or all decreasing.
	// Any two adjacent levels differ by at least one and at most three.
	lastDiff := 0
	for i := 0; i < len(report)-1; i++ {
		curr := report[i]
		next := report[i+1]
		diff := curr - next
		if diff == 0 || lastDiff < 0 && diff > 0 || lastDiff > 0 && diff < 0 {
			return false
		}
		if diff > 3 || diff < -3 {
			return false
		}
		lastDiff = diff
	}
	return true
}

func isSafeDampened(report []int) bool {
	// Conditions for safety:
	// isSafe(report)
	// There is a tolerance of a single Bad level, so if its unsafe BUT by removing a single Bad level it becomes safe, then the entire structure is safe.
	if isSafe(report) {
		return true
	}

	// Try to remove a single Bad level
	for i := 0; i < len(report); i++ {
		dampened := append([]int{}, report[:i]...)   // Copy left part
		dampened = append(dampened, report[i+1:]...) // Append right part

		if isSafe(dampened) {
			return true
		}
	}
	return false
}
