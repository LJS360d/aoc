package main

import (
	"fmt"
	"os"
	"strings"

	pq "github.com/emirpasic/gods/queues/priorityqueue"
)

// / Advent of Code 2024 - Day 20 # Part 1
// / https://adventofcode.com/2024/day/20#part1
func main() {
	buffer, _ := os.ReadFile("day20/input.txt")
	content := string(buffer)
	grid := parse(content)
	start := findStart(grid)
	_, scores := getDijkstraScores(start, grid)

	uniqueCheats := make(map[Cheat]int)
	count := 0

	for pos, score := range scores {
		for _, dir1 := range directions {
			cheat1 := Position{pos.x + dir1.x, pos.y + dir1.y}
			if !isValidStep(cheat1, grid) {
				continue
			}
			if grid[cheat1.y][cheat1.x] != "#" {
				continue
			}

			for _, dir2 := range directions {
				cheat2 := Position{cheat1.x + dir2.x, cheat1.y + dir2.y}
				if !isValidStep(cheat2, grid) {
					continue
				}
				if val2, ok := scores[cheat2]; ok && val2-score-2 > 0 {
					uniqueCheats[Cheat{s: cheat1, e: cheat2}] = val2 - score - 2
					if val2-score-2 >= 100 {
						count++
					}
				}
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
