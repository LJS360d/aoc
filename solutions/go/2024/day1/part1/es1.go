package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

/// Advent of Code 2024 - Day 1 # Part 1
/// https://adventofcode.com/2024/day/1
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
		l, r := parseLine(line)
		left = append(left, l)
		right = append(right, r)
	}

	sortNumbers(left)
	sortNumbers(right)

	fmt.Println(len(left))
	fmt.Println(len(right))
	diff := 0
	for i := 0; i < len(left); i++ {
		l := left[i]
		r := right[i]
		if l > r {
			diff += l - r
		} else {
			diff += r - l
		}
	}
	fmt.Println(diff)
}

func parseLine(line string) (int, int) {
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

func sortNumbers(numbers []int) {
	for i := 0; i < len(numbers); i++ {
		for j := i + 1; j < len(numbers); j++ {
			if numbers[i] > numbers[j] {
				numbers[i], numbers[j] = numbers[j], numbers[i]
			}
		}
	}
}
