package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

// / Advent of Code 2024 - Day 16 # Part 2
// / https://adventofcode.com/2024/day/16#part2
func main() {
	grid, sIdx, eIdx := parseInput("day16/input.txt")
	p1, p2 := BFS(grid, sIdx, eIdx)
	fmt.Println("Part One:", p1)
	fmt.Println("Part Two:", p2)
}

const (
	wall = '#'
	path = '.'
)

type Point struct {
	x, y int
}

type Direction struct {
	dx, dy int
}

type Node struct {
	pos Point
	dir Direction
}

type State struct {
	reindeer Node
	path     []Point
	score    int
}

var directions = []Direction{
	{-1, 0},
	{0, 1},
	{1, 0},
	{0, -1},
}

func parseInput(fileName string) (grid [][]rune, sIdx, eIdx Point) {
	file, _ := os.Open(fileName)
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		for i, r := range []rune(scanner.Text()) {
			if r == 'S' {
				sIdx = Point{x: len(grid), y: i}
			} else if r == 'E' {
				eIdx = Point{x: len(grid), y: i}
			}
		}
		grid = append(grid, []rune(scanner.Text()))
	}
	return
}

func BFS(grid [][]rune, sIdx, eIdx Point) (minScore int, bestSeatCount int) {
	minScore = math.MaxInt
	reindeer := Node{pos: sIdx, dir: Direction{dx: 0, dy: 1}}
	queue := []State{
		{
			reindeer: reindeer,
			path:     []Point{sIdx},
			score:    0,
		},
	}
	visited := make(map[Node]int)
	sizeToIndices := make(map[int][]Point)

	for len(queue) > 0 {
		currState := queue[0]
		queue = queue[1:]

		if currState.score > minScore {
			continue
		}

		if currState.reindeer.pos == eIdx {
			if currState.score <= minScore {
				minScore = currState.score
				sizeToIndices[minScore] = append(sizeToIndices[minScore], currState.path...)
			}
			continue
		}

		for _, n := range getNeighbours(currState.reindeer) {
			if grid[n.pos.x][n.pos.y] == wall {
				continue
			}
			score := currState.score + 1
			if currState.reindeer.dir != n.dir {
				score += 1000
			}
			if previous, has := visited[n]; has {
				if previous < score {
					continue
				}
			}
			visited[n] = score

			nPath := make([]Point, len(currState.path))
			copy(nPath, currState.path)

			queue = append(queue, State{
				reindeer: n,
				path:     append(nPath, n.pos),
				score:    score,
			})
		}
	}

	countMap := make(map[Point]bool)
	for _, index := range sizeToIndices[minScore] {
		countMap[index] = true
	}

	return minScore, len(countMap)
}

func getNeighbours(reindeer Node) (neighbours []Node) {
	neighbours = make([]Node, 0, 4)
	currDir, currIdx := reindeer.dir, reindeer.pos
	oppositeDir := Direction{dx: -currDir.dx, dy: -currDir.dy}

	for _, dir := range directions {
		if dir == oppositeDir {
			continue
		}
		nIdx := Point{x: currIdx.x + dir.dx, y: currIdx.y + dir.dy}
		neighbours = append(neighbours, Node{pos: nIdx, dir: dir})
	}

	return
}
