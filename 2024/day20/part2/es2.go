package main

import (
	"fmt"
	"math"
	"os"
	"strings"

	pq "github.com/emirpasic/gods/queues/priorityqueue"
)

// / Advent of Code 2024 - Day 20 # Part 2
// / https://adventofcode.com/2024/day/20#part2
func main() {
	buffer, _ := os.ReadFile("day20/input.txt")
	content := string(buffer)
	grid := parse(content)
	start := findStart(grid)
	_, scores := getDijkstraScores(start, grid)

	uniqueCheats := make(map[Cheat]int)
	count := 0

	for p1, d1 := range scores {
		for p2, d2 := range scores {
			if d2-d1-manhattanDistance(p1, p2) >= 100 && manhattanDistance(p1, p2) <= 20 {
				uniqueCheats[Cheat{s: p1, e: p2}] = d2 - d1
				count++
			}
		}
	}

	fmt.Println(count)
}

func parse(content string) [][]string {
	lines := strings.Split(content, "\n")
	grid := make([][]string, len(lines))
	for i := range grid {
		grid[i] = strings.Split(lines[i], "")
	}
	return grid
}

type Position struct {
	x, y int
}

func findStart(grid [][]string) Position {
	for j := range grid {
		for i, char := range grid[j] {
			if char == "S" {
				return Position{i, j}
			}
		}
	}
	return Position{}
}

type Step struct {
	co      Position
	lastDir Position
	score   int
	path    map[Position]int
}

type Point struct {
	co      Position
	lastDir Position
}

var directions = []Position{
	{x: 0, y: -1}, // Up
	{x: 1, y: 0},  // Right
	{x: 0, y: 1},  // Down
	{x: -1, y: 0}, // Left
}

func getDijkstraScores(start Position, matrix [][]string) (int, map[Position]int) {

	priorityQueue := pq.NewWith(func(a, b interface{}) int {
		return a.(Step).score - b.(Step).score
	})

	priorityQueue.Enqueue(Step{start, directions[0], 0, make(map[Position]int)})

	visited := make(map[Point]struct{})

	for !priorityQueue.Empty() {
		element, _ := priorityQueue.Dequeue()

		currentNode := element.(Step)

		if _, ok := visited[Point{currentNode.co, currentNode.lastDir}]; ok {
			continue
		}

		currentNode.path[currentNode.co] = currentNode.score

		if matrix[currentNode.co.y][currentNode.co.x] == "E" {
			return currentNode.score, currentNode.path
		}

		nextSteps := getNextSteps(currentNode, matrix, visited)
		for _, n := range nextSteps {
			priorityQueue.Enqueue(n)
		}

		visited[Point{currentNode.co, currentNode.lastDir}] = struct{}{}
	}
	return -1, make(map[Position]int)
}

func isValidStep(current Position, input [][]string) bool {
	if current.x < 0 || current.y < 0 || current.x >= len(input[0]) || current.y >= len(input) {
		return false
	}
	return true
}

func copyMap(path map[Position]int) map[Position]int {
	new := make(map[Position]int, len(path))
	for key, value := range path {
		new[key] = value
	}
	return new
}

func getAllowedDirections(direction Position) []Position {
	switch direction {
	case directions[0]:
		return []Position{directions[0], directions[3], directions[1]}
	case directions[2]:
		return []Position{directions[2], directions[3], directions[1]}
	case directions[3]:
		return []Position{directions[0], directions[3], directions[2]}
	case directions[1]:
		return []Position{directions[0], directions[2], directions[1]}
	}
	return []Position{}
}

func getNextSteps(current Step, grid [][]string, visited map[Point]struct{}) []Step {
	possibleNext := []Step{}
	for _, dir := range getAllowedDirections(current.lastDir) {
		newPosition := Position{current.co.x + dir.x, current.co.y + dir.y}

		if !isValidStep(newPosition, grid) {
			continue
		}

		if grid[newPosition.y][newPosition.x] == "#" {
			continue
		}

		if _, ok := visited[Point{newPosition, dir}]; ok {
			continue
		}

		score := current.score + 1

		possibleNext = append(possibleNext, Step{
			co:      newPosition,
			lastDir: dir,
			score:   score,
			path:    copyMap(current.path),
		})
	}
	return possibleNext
}

type Cheat struct {
	s Position
	e Position
}

func manhattanDistance(p1, p2 Position) int {
	x := int(math.Abs(float64(p1.x - p2.x)))
	y := int(math.Abs(float64(p1.y - p2.y)))
	return x + y
}
