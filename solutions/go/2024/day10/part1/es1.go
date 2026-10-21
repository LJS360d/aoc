package main

import (
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 10 # Part 1
// / https://adventofcode.com/2024/day/10#part1
func main() {
	buffer, _ := os.ReadFile("day10/input.txt")
	content := string(buffer)
	grid := parseGrid(content)
	th := getTrailheads(grid)
	scoreMap := make(map[string][][]int)
	for _, head := range th {
		tails, err := resolveTrails(grid, head, 0)
		if err != nil {
			continue
		}
		headStr := strconv.Itoa(head[0]) + strconv.Itoa(head[1])
		if _, ok := scoreMap[headStr]; !ok {
			scoreMap[headStr] = make([][]int, 0)
		}
		scoreMap[headStr] = append(scoreMap[headStr], tails...)
	}

	score := 0
	for _, v := range scoreMap {
		score += len(v)
	}
	fmt.Println(score)
}

func parseGrid(content string) [][]int {
	lines := strings.Split(content, "\n")
	grid := make([][]int, len(lines))
	for i, line := range lines {
		grid[i] = make([]int, len(line))
		for j, char := range line {
			n, _ := strconv.Atoi(string(char))
			grid[i][j] = n
		}
	}
	return grid
}

func getTrailheads(grid [][]int) [][]int {
	trailheads := make([][]int, 0)
	for i, line := range grid {
		for j, height := range line {
			if height == 0 {
				trailheads = append(trailheads, []int{i, j})
			}
		}
	}
	return trailheads
}

func resolveTrails(grid [][]int, head []int, height int) ([][]int, error) {
	directions := [][]int{
		{0, -1}, // Up
		{0, 1},  // Down
		{-1, 0}, // Left
		{1, 0},  // Right
	}

	var results [][]int

	// Recursive DFS function to explore paths
	var explore func([]int, int)
	explore = func(currentPos []int, currentHeight int) {
		if currentHeight == 9 {
			if !includesPos(results, currentPos) {
				results = append(results, currentPos)
			}
			return
		}
		for _, dir := range directions {
			nextPos := []int{currentPos[0] + dir[0], currentPos[1] + dir[1]}
			if isValidPosition(grid, nextPos) && grid[nextPos[0]][nextPos[1]] == currentHeight+1 {
				explore(nextPos, currentHeight+1)
			}
		}
	}

	explore(head, height)

	if len(results) == 0 {
		return nil, errors.New("no paths to height 9 found")
	}
	return results, nil
}

func isValidPosition(grid [][]int, pos []int) bool {
	return pos[0] >= 0 && pos[0] < len(grid) &&
		pos[1] >= 0 && pos[1] < len(grid[0])
}

func includesPos(positions [][]int, pos []int) bool {
	for _, p := range positions {
		if p[0] == pos[0] && p[1] == pos[1] {
			return true
		}
	}
	return false
}
