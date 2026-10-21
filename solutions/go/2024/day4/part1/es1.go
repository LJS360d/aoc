package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// / Advent of Code 2024 - Day 4 # Part 1
// / https://adventofcode.com/2024/day/4
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
	targetWord := "XMAS"
	countDirect := len(searchWord(grid, targetWord))
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

func searchWord(grid [][]rune, word string) [][][2]int {
	if len(word) == 0 || len(grid) == 0 || len(grid[0]) == 0 {
		return nil
	}

	wordRunes := []rune(word)
	wordLen := len(wordRunes)
	rows := len(grid)
	cols := len(grid[0])
	var result [][][2]int

	// Function to check if the word exists from a starting position in a specific direction
	checkDirection := func(x, y, dx, dy int) [][2]int {
		coords := make([][2]int, 0, wordLen)
		for i := 0; i < wordLen; i++ {
			nx, ny := x+dx*i, y+dy*i
			if nx < 0 || ny < 0 || nx >= rows || ny >= cols || grid[nx][ny] != wordRunes[i] {
				return nil
			}
			coords = append(coords, [2]int{nx, ny})
		}
		return coords
	}

	// Iterate over every position in the grid
	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == wordRunes[0] { // Potential start of the word
				for _, dir := range directions {
					coords := checkDirection(i, j, dir[0], dir[1])
					if coords != nil {
						result = append(result, coords)
					}
				}
			}
		}
	}
	return result
}
