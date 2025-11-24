package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// / Advent of Code 2024 - Day 4 # Part 2
// / https://adventofcode.com/2024/day/4#part2
func main() {
	input, err := os.Open("day4/input.txt")
	if err != nil {
		panic(err)
	}
	defer input.Close()

	scanner := bufio.NewScanner(input)
	scanner.Split(bufio.ScanLines)

	grid := make([][]rune, 0)
	for scanner.Scan() {
		line := scanner.Text()
		grid = append(grid, parseLine(line))
	}
	targetWord := "MAS"
	countDirect := len(searchCrossedWord(grid, targetWord))
	fmt.Println(countDirect)
	// 3794 too high

}

func parseLine(line string) []rune {
	slice := make([]rune, 0)
	for _, v := range strings.Split(line, "") {
		slice = append(slice, rune(v[0]))
	}
	return slice
}

var directions = [][2]int{
	{-1, 0},  // up
	{1, 0},   // down
	{0, -1},  // left
	{0, 1},   // right
	{-1, -1}, // top-left diagonal
	{-1, 1},  // top-right diagonal
	{1, -1},  // bottom-left diagonal
	{1, 1},   // bottom-right diagonal
}

func searchCrossedWord(grid [][]rune, word string) [][][2]int {
	if len(word) != 3 || len(grid) < 3 || len(grid[0]) < 3 {
		return nil // Word must be length 3, and grid must be at least 3x3.
	}

	wordRunes := []rune(word)
	reversedRunes := []rune{wordRunes[2], wordRunes[1], wordRunes[0]} // Reversed word
	rows := len(grid)
	cols := len(grid[0])
	var results [][][2]int

	// Check if a valid 'X' exists centered at (x, y)
	checkX := func(x, y int) [][2]int {
		if x-1 < 0 || x+1 >= rows || y-1 < 0 || y+1 >= cols {
			return nil // Out of bounds for X shape
		}

		// Check diagonal 1 (top-left ↘ bottom-right)
		if grid[x-1][y-1] != wordRunes[0] || grid[x][y] != wordRunes[1] || grid[x+1][y+1] != wordRunes[2] {
			if grid[x-1][y-1] != reversedRunes[0] || grid[x][y] != reversedRunes[1] || grid[x+1][y+1] != reversedRunes[2] {
				return nil
			}
		}

		// Check diagonal 2 (top-right ↙ bottom-left)
		if grid[x-1][y+1] != wordRunes[0] || grid[x][y] != wordRunes[1] || grid[x+1][y-1] != wordRunes[2] {
			if grid[x-1][y+1] != reversedRunes[0] || grid[x][y] != reversedRunes[1] || grid[x+1][y-1] != reversedRunes[2] {
				return nil
			}
		}

		// Valid X found
		return [][2]int{
			{x - 1, y - 1}, {x, y}, {x + 1, y + 1}, // Diagonal 1
			{x - 1, y + 1}, {x, y}, {x + 1, y - 1}, // Diagonal 2
		}
	}

	// Iterate through every cell as the center of a potential X
	for i := 1; i < rows-1; i++ {
		for j := 1; j < cols-1; j++ {
			if grid[i][j] == wordRunes[1] { // Middle letter matches
				if coords := checkX(i, j); coords != nil {
					results = append(results, coords)
				}
			}
		}
	}

	return results
}
