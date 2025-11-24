package main

import (
	"adventofcode2024/common"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 6 # Part 2
// / https://adventofcode.com/2024/day/6#part2
func main() {
	buffer, _ := os.ReadFile("day6/input.txt")
	content := string(buffer)
	lines := strings.Split(content, "\n")

	var grid [][]rune
	for _, line := range lines {
		grid = append(grid, parseLine(line))
	}
	// 130x130 grid
	pos := getGuardPosition(grid)
	// x: 60, y: 70
	fmt.Println("Initial Guard position:", pos)
	direction := []int{0, -1} // UP
	path, _ := getPatrolPath(grid, pos, direction)
	pathS := sliceutils.Map(path, func(v []int, i int, s [][]int) string {
		return fmt.Sprintf("%d,%d", v[0], v[1])
	})
	uniquePositionsSet := common.NewSet[string]()
	for _, coord := range pathS {
		uniquePositionsSet.Add(coord)
	}
	uniquePositions := sliceutils.Map(uniquePositionsSet.ToSlice(), func(v string, i int, s []string) []int {
		v1 := strings.Split(v, ",")[0]
		v2 := strings.Split(v, ",")[1]
		n1, _ := strconv.Atoi(v1)
		n2, _ := strconv.Atoi(v2)
		return []int{n1, n2}
	})
	// 4967 !!!
	loops := 0
	for _, uPos := range uniquePositions {
		grid[uPos[1]][uPos[0]] = '#'
		_, loop := getPatrolPath(grid, pos, direction)
		if loop != nil {
			loops++
		}
		grid[uPos[1]][uPos[0]] = '.'
	}

	fmt.Println(loops)
}

func parseLine(line string) []rune {
	slice := make([]rune, 0)
	for _, v := range strings.Split(line, "") {
		slice = append(slice, rune(v[0]))
	}
	return slice
}

func getGuardPosition(grid [][]rune) []int {
	rows := len(grid)
	cols := len(grid[0])
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == '^' {
				return []int{j, i}
			}
		}
	}
	return []int{-1, -1}
}

// [60, 56]
func getPatrolPath(grid [][]rune, initialPos []int, initialDirection []int) ([][]int, error) {
	currentPos := initialPos
	currentDir := initialDirection
	result := make([][]int, 0)

	for {
		nextPos := getNextObstaclePostition(grid, currentPos, currentDir)
		if nextPos[0] == -1 && nextPos[1] == -1 {
			break
		}
		newPos := []int{nextPos[0] - currentDir[0], nextPos[1] - currentDir[1]}
		path := getSegment(currentPos, newPos)
		result = append(result, path...)
		currentPos = newPos
		result = append(result, currentPos)
		currentDir = getNextDirection(currentDir)
		if len(result) > 10000 {
			return nil, errors.New("loop detected")
		}
	}
	lastPos := getLastPosition(grid, currentPos, currentDir)
	path := getSegment(currentPos, lastPos)
	result = append(result, path...)

	return result, nil

}

func getNextObstaclePostition(grid [][]rune, currentPos []int, direction []int) []int {
	rows := len(grid)
	cols := len(grid[0])

	if direction[0] == 0 {
		if direction[1] == -1 {
			// UP
			for i := currentPos[1]; i >= 0; i += direction[1] {
				p := grid[i][currentPos[0]]
				if p == '#' {
					return []int{currentPos[0], i}
				}
			}
		} else {
			// DOWN
			for i := currentPos[1]; i < rows; i += direction[1] {
				if grid[i][currentPos[0]] == '#' {
					return []int{currentPos[0], i}
				}
			}
		}
	}

	if direction[1] == 0 {
		if direction[0] == -1 {
			//LEFT
			for i := currentPos[0]; i >= 0; i += direction[0] {
				if grid[currentPos[1]][i] == '#' {
					return []int{i, currentPos[1]}
				}
			}
		} else {
			//RIGHT
			for i := currentPos[0]; i < cols; i += direction[0] {
				if grid[currentPos[1]][i] == '#' {
					return []int{i, currentPos[1]}
				}
			}
		}
	}
	return []int{-1, -1}
}

func getLastPosition(grid [][]rune, currentPos []int, direction []int) []int {
	rows := len(grid)
	cols := len(grid[0])

	// DOWN
	if direction[0] == 0 && direction[1] == 1 {
		return []int{currentPos[0], cols - 1}
	}
	// UP
	if direction[0] == 0 && direction[1] == -1 {
		return []int{currentPos[0], 0}
	}
	// RIGHT
	if direction[0] == 1 && direction[1] == 0 {
		return []int{rows - 1, currentPos[1]}
	}
	// LEFT
	if direction[0] == 0 && direction[1] == 1 {
		return []int{0, currentPos[1]}
	}
	return []int{-1, -1}
}

func getNextDirection(currentDirection []int) []int {
	// Rotate 90 degrees clockwise
	if currentDirection[0] == 0 && currentDirection[1] == -1 {
		return []int{1, 0}
	}
	if currentDirection[0] == 1 && currentDirection[1] == 0 {
		return []int{0, 1}
	}
	if currentDirection[0] == 0 && currentDirection[1] == 1 {
		return []int{-1, 0}
	}
	return []int{0, -1}
}

func getSegment(a, b []int) [][]int {
	// Initialize the result slice to hold the points in the segment
	var segment [][]int

	// Extract coordinates of the points
	ax, ay := a[0], a[1]
	bx, by := b[0], b[1]

	// Case 1: Vertical segment (x-coordinates are equal)
	if ax == bx {
		step := 1
		if ay > by {
			step = -1
		}
		for y := ay; y != by+step; y += step {
			segment = append(segment, []int{ax, y})
		}
	}

	// Case 2: Horizontal segment (y-coordinates are equal)
	if ay == by {
		step := 1
		if ax > bx {
			step = -1
		}
		for x := ax; x != bx+step; x += step {
			segment = append(segment, []int{x, ay})
		}
	}

	return segment
}
