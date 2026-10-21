package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

var bounds = [2]int{70, 70}

// / Advent of Code 2024 - Day 18 # Part 2
// / https://adventofcode.com/2024/day/18#part2
func main() {
	buffer, _ := os.ReadFile("day18/input.txt")
	content := string(buffer)
	initial, stream := parse(content)
	start := Position{x: 0, y: 0}
	end := Position{x: bounds[0], y: bounds[1]}
	for _, s := range stream {
		initial = append(initial, s)
		path := bfs(start, end, initial)
		visualize(initial, path, bounds)
		if path == nil {
			cls()
			fmt.Println(s)
			return
		}
	}
}

type Position struct {
	x, y int
}

func parse(content string) ([]Position, []Position) {
	result := make([]Position, 0)
	stream := make([]Position, 0)
	amount := 12
	lines := strings.Split(content, "\n")
	for i := 0; i < amount; i++ {
		line := lines[i]
		v := strings.Split(line, ",")
		xStr, yStr := v[0], v[1]
		x, _ := strconv.Atoi(xStr)
		y, _ := strconv.Atoi(yStr)
		pos := Position{x: x, y: y}
		result = append(result, pos)
	}
	for i := amount; i < len(lines); i++ {
		line := lines[i]
		v := strings.Split(line, ",")
		xStr, yStr := v[0], v[1]
		x, _ := strconv.Atoi(xStr)
		y, _ := strconv.Atoi(yStr)
		pos := Position{x: x, y: y}
		stream = append(stream, pos)
	}
	return result, stream
}

func visualize(walls, path []Position, bounds [2]int) {
	grid := make([][]rune, bounds[1]+1)
	for i := range grid {
		grid[i] = make([]rune, bounds[0]+1)
		for j := range grid[i] {
			pos := Position{x: j, y: i}
			foundWall := slices.Contains(walls, pos)
			if foundWall {
				grid[i][j] = '#'
			} else {
				foundPath := slices.Contains(path, pos)
				if foundPath {
					grid[i][j] = 'O'
				} else {
					grid[i][j] = '.'
				}
			}
		}
	}
	str := ""
	for _, row := range grid {
		str += string(row) + "\n"
	}
	cls()
	fmt.Println(str)
}

func cls() {
	fmt.Print("\033[H\033[2J")
}

func bfs(start, target Position, blocked []Position) []Position {
	// Map to store blocked positions
	blockedSet := make(map[Position]bool)
	for _, b := range blocked {
		blockedSet[b] = true
	}

	// Queue for BFS
	queue := []Position{start}
	// Map to store the parent of each position (for path reconstruction)
	parents := make(map[Position]*Position)
	parents[start] = nil

	// BFS Loop
	for len(queue) > 0 {
		curr := queue[0]
		queue = queue[1:]

		// If we reached the target
		if curr == target {
			return reconstructPath(parents, target)
		}

		// Explore neighbors
		for _, d := range directions {
			neighbor := Position{x: curr.x + d.x, y: curr.y + d.y}

			// Check bounds and if the position is traversable
			if neighbor.x >= 0 && neighbor.x <= bounds[0] && neighbor.y >= 0 && neighbor.y <= bounds[1] && !blockedSet[neighbor] {
				// If not already visited
				if _, visited := parents[neighbor]; !visited {
					parents[neighbor] = &curr
					queue = append(queue, neighbor)
				}
			}
		}
	}

	// If we exhaust the BFS without finding a path, return empty
	return nil
}

// Reconstruct the path using the parents map
func reconstructPath(parents map[Position]*Position, target Position) []Position {
	var path []Position
	for curr := &target; curr != nil; curr = parents[*curr] {
		path = append([]Position{*curr}, path...)
	}
	return path
}

var directions = []Position{
	{x: 0, y: -1},
	{x: 1, y: 0},
	{x: 0, y: 1},
	{x: -1, y: 0},
}
