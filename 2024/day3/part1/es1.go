package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

// / Advent of Code 2024 - Day 3 # Part 1
// / https://adventofcode.com/2024/day/3
func main() {
	input, err := os.Open("day3/input.txt")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	results := make([]int, 0)
	for scanner.Scan() {
		line := scanner.Text()
		slr := parseLine(line)
		results = append(results, slr...)
	}

	res := 0
	for i := 0; i < len(results); i++ {
		res += results[i]
	}
	fmt.Println(res)
}

func parseLine(line string) []int {
	var ops []int
	regex, _ := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
	for _, match := range regex.FindAllStringSubmatch(line, -1) {
		num1, _ := strconv.Atoi(match[1])
		num2, _ := strconv.Atoi(match[2])
		ops = append(ops, num1*num2)
	}
	return ops
}
