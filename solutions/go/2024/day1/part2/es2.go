package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

/// Advent of Code 2024 - Day 1 # Part 2
/// https://adventofcode.com/2024/day/1#part2
/// Read the file, each line is separated by "   " and of the left and right theres numbers
/// Parse those numbers into arrays and sort them by how big they are (smallest ones first)

func main() {
	input, err := os.Open("day1/input.txt")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	var left []int
	var right []int
	for scanner.Scan() {
		line := scanner.Text()
		l, r := ParseLine(line)
		left = append(left, l)
		right = append(right, r)
	}

	score := 0
	for i := 0; i < len(left); i++ {
		l := left[i]
		score += l * findDuplicatesOf(right, l)
	}
	fmt.Println(score)
}

func findDuplicatesOf(numbers []int, of int) int {
	duplicates := 0
	for _, n := range numbers {
		if n == of {
			duplicates++
		}
	}
	return duplicates
}

func ParseLine(line string) (int, int) {
	var numbers []int
	for _, number := range strings.Split(line, "   ") {
		num, err := strconv.Atoi(number)
		if err != nil {
			panic(err)
		}
		numbers = append(numbers, num)
	}
	return numbers[0], numbers[1]
}
