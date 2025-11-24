package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 22 # Part 1
// / https://adventofcode.com/2024/day/22#part1
func main() {
	buffer, _ := os.ReadFile("day22/input.txt")
	content := string(buffer)
	nums := parse(content)
	res := int64(0)
	for _, n := range nums {
		finalSec := nextSecret(n, 2000)
		res += finalSec
	}
	fmt.Println(res)
	// 14391933445 too high
	// 14119253575 !!!
}

func parse(content string) []int64 {
	lines := strings.Split(content, "\n")
	numbers := make([]int64, len(lines))
	for i := range lines {
		numbers[i], _ = strconv.ParseInt(lines[i], 10, 64)
	}
	return numbers
}

func nextSecret(num int64, i int) int64 {
	pruneNum := int64(16777216)
	for ; i > 0; i-- {
		num = ((num << 6) ^ num) & (pruneNum - 1)
		num = ((num >> 5) ^ num) & (pruneNum - 1)
		num = ((num << 11) ^ num) & (pruneNum - 1)
	}
	return num
}
