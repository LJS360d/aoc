package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 12 # Part 1
// / https://adventofcode.com/2024/day/12#part1
func main() {
	buffer, _ := os.ReadFile("day12/input.txt")
	content := string(buffer)
	grid := parse(content)
	res := 0
	for start := getNextAreaStart(grid); start != [2]int{-1, -1}; start = getNextAreaStart(grid) {
		cells := floodFill(grid, start)
		area := len(cells)
		perimeter := getCellsPerimeter(cells)
		res += area * perimeter
	}
	fmt.Println(res)
}

func parse(content string) [][]rune {
	lines := strings.Split(content, "\n")
	result := make([][]rune, 0)
	for _, line := range lines {
		line = strings.TrimSpace(line)
		result = append(result, []rune(line))
	}
	return result
}

var directions = [][2]int{
	{0, 1},  // right
	{1, 0},  // down
	{0, -1}, // left
	{-1, 0}, // up
}

func floodFill(grid [][]rune, start [2]int) [][2]int {
	rows, cols := len(grid), len(grid[0])
	startX, startY := start[0], start[1]
	target := grid[startX][startY]
	lowerTarget := unicode.ToLower(target)

	if target == lowerTarget {
		return [][2]int{}
	}

	// Queue for BFS
	queue := [][2]int{{startX, startY}}

	cells := make([][2]int, 0)

	for len(queue) > 0 {
		// Dequeue
		current := queue[0]
		queue = queue[1:]

		x, y := current[0], current[1]

		// Skip if the cell is already processed
		if grid[x][y] == lowerTarget {
			continue
		}

		// Lowercase the current cell
		grid[x][y] = lowerTarget
		cells = append(cells, [2]int{x, y})

		// Explore neighbors
		for _, d := range directions {
			nx, ny := x+d[0], y+d[1]

			if nx >= 0 && ny >= 0 && nx < rows && ny < cols && grid[nx][ny] == target {
				queue = append(queue, [2]int{nx, ny})
			}
		}
	}

	return cells
}

func getNextAreaStart(grid [][]rune) [2]int {
	for j, row := range grid {
		for i, cell := range row {
			if cell != unicode.ToLower(cell) {
				return [2]int{j, i}
			}
		}
	}
	return [2]int{-1, -1}
}

func getCellsPerimeter(cells [][2]int) int {
	perimeter := 0
	for _, cell := range cells {
		adj := getAdjacentCellsValidPositions(cells, cell)
		perimeter += len(directions) - len(adj)
	}
	return perimeter
}

func isValidPosition(cells [][2]int, pos [2]int) bool {
	return sliceutils.Includes(cells, pos)
}

func getAdjacentCellsValidPositions(cells [][2]int, pos [2]int) [][2]int {
	adjacent := make([][2]int, 0)
	for _, dir := range directions {
		adj := [2]int{pos[0] + dir[0], pos[1] + dir[1]}
		if isValidPosition(cells, adj) {
			adjacent = append(adjacent, adj)
		}
	}
	return adjacent
}
