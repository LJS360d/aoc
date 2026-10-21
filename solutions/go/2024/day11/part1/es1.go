package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 11 # Part 1
// / https://adventofcode.com/2024/day/11#part1
func main() {
	buffer, _ := os.ReadFile("day11/input.txt")
	content := string(buffer)
	stones := parseStones(content)
	for i := 0; i < 75; i++ {
		stones = blink(stones)
	}
	fmt.Println(len(stones))
}

func parseStones(content string) []int {
	stones := sliceutils.Map(strings.Split(content, " "), func(v string, i int, s []string) int {
		n, _ := strconv.Atoi(v)
		return n
	})
	return stones
}

func blink(stones []int) []int {
	nextStones := make([]int, 0)
	for _, value := range stones {
		if value == 0 {
			nextStones = append(nextStones, 1)
			continue
		}
		str := strconv.Itoa(value)
		if len(str)%2 == 0 {
			middleIndex := len(str) / 2
			left, _ := strconv.Atoi(str[0:middleIndex])
			right, _ := strconv.Atoi(str[middleIndex:])
			nextStones = append(nextStones, left)
			nextStones = append(nextStones, right)
			continue
		}
		nextStones = append(nextStones, value*2024)
	}
	return nextStones
}
