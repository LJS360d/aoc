package main

import (
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 10 # Part 2
// / https://adventofcode.com/2024/day/10#part2
func main() {
	buffer, _ := os.ReadFile("day10/input.txt")
	content := string(buffer)
	grid := parseGrid(content)
	th := getTrailheads(grid)
	scoreMap := make(map[string][]string)

	for _, head := range th {
		trails, err := resolveTrails(grid, head, 0)
		if err != nil {
			continue
		}
		headStr := strconv.Itoa(head[0]) + strconv.Itoa(head[1])
		if _, ok := scoreMap[headStr]; !ok {
			scoreMap[headStr] = make([]string, 0)
		}
		for _, path := range trails {
			pathStr := getPathString(path)
			scoreMap[headStr] = append(scoreMap[headStr], pathStr)
		}
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

func resolveTrails(grid [][]int, head []int, height int) ([][][]int, error) {
	type Path struct {
		Pos   []int   // Current position
		Trace [][]int // Path trace
	}

	directions := [][]int{
		{0, -1}, // Up
		{0, 1},  // Down
		{-1, 0}, // Left
		{1, 0},  // Right
	}

	var results [][][]int

	// Recursive DFS function to explore paths
	var explore func(Path, int)
	explore = func(currentPath Path, currentHeight int) {
		currentPos := currentPath.Pos
		if currentHeight == 9 {
			results = append(results, append(currentPath.Trace, currentPos))
			return
		}
		for _, dir := range directions {
			nextPos := []int{currentPos[0] + dir[0], currentPos[1] + dir[1]}
			if isValidPosition(grid, nextPos) && grid[nextPos[0]][nextPos[1]] == currentHeight+1 {
				newPath := Path{
					Pos:   nextPos,
					Trace: append(append([][]int{}, currentPath.Trace...), currentPos),
				}
				explore(newPath, currentHeight+1)
			}
		}
	}

	// Start exploring from the head
	initialPath := Path{
		Pos:   head,
		Trace: [][]int{},
	}
	explore(initialPath, height)

	if len(results) == 0 {
		return nil, errors.New("no paths to height 9 found")
	}
	return results, nil
}

func isValidPosition(grid [][]int, pos []int) bool {
	return pos[0] >= 0 && pos[0] < len(grid) &&
		pos[1] >= 0 && pos[1] < len(grid[0])
}

func getPathString(path [][]int) string {
	pathStr := ""
	for _, step := range path {
		stepStr := fmt.Sprint(step[0], step[1])
		pathStr += stepStr
		pathStr += ";"
	}
	return pathStr

}
