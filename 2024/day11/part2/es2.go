package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 11 # Part 2
// / https://adventofcode.com/2024/day/11#part2
func main() {
	buffer, _ := os.ReadFile("day11/input.txt")
	content := string(buffer)
	stones := parseStones(content)
	for i := 0; i < 75; i++ {
		stones = blink(stones)
	}
	sum := 0
	for _, v := range stones {
		sum += v
	}
	fmt.Println(sum)
}

func parseStones(content string) map[int]int {
	stones := make(map[int]int, 0)
	for _, v := range strings.Split(content, " ") {
		n, _ := strconv.Atoi(v)
		stones[n] += 1
	}
	return stones
}

func blink(stones map[int]int) map[int]int {
	nextStones := make(map[int]int, 0)
	for value, count := range stones {
		if value == 0 {
			nextStones[1] += count
			continue
		}
		str := strconv.Itoa(value)
		if len(str)%2 == 0 {
			middleIndex := len(str) / 2
			left, _ := strconv.Atoi(str[0:middleIndex])
			right, _ := strconv.Atoi(str[middleIndex:])
			nextStones[left] += count
			nextStones[right] += count
			continue
		}
		nextStones[value*2024] += count
	}
	return nextStones
}
