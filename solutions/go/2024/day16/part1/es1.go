package main

import (
	"container/heap"
	"fmt"
	"os"
	"strings"
)

var directions = []Position{
	{x: 0, y: -1}, // North
	{x: 1, y: 0},  // East
	{x: 0, y: 1},  // South
	{x: -1, y: 0}, // West
}

type Node struct {
	pos      Position
	dir      Position // Direction the node is reached from
	cost     int      // Total cost to reach this node
	turns    int      // Number of turns made to reach this node
	steps    int      // Number of steps taken
	priority int      // Priority for the priority queue (cost + heuristic)
	prev     *Node    // To reconstruct the path
}

type PriorityQueue []*Node

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(*Node))
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	*pq = old[0 : n-1]
	return item
}

func main() {
	buffer, _ := os.ReadFile("day16/input.txt")
	content := string(buffer)
	maze := parse(content)
	start := getStart(maze)
	end := getEnd(maze)

	_, turns, steps := findPath(maze, start, end)

	fmt.Println(turns*1000 + steps)
}

type Position struct {
	x int
	y int
}

func parse(content string) map[Position]string {
	result := make(map[Position]string)
	lines := strings.Split(content, "\n")
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		for j := 0; j < len(line); j++ {
			pos := Position{x: i, y: j}
			result[pos] = string(line[j])
		}
	}
	return result
}

func getStart(maze map[Position]string) Position {
	for p, v := range maze {
		if v == "S" {
			return p
		}
	}
	return Position{}
}

func getEnd(maze map[Position]string) Position {
	for p, v := range maze {
		if v == "E" {
			return p
		}
	}
	return Position{}
}

func visualize(maze map[Position]string) {

	var maxX, maxY int
	for pos := range maze {
		if pos.x > maxX {
			maxX = pos.x
		}
		if pos.y > maxY {
			maxY = pos.y
		}
	}

	grid := make([][]string, maxY+1)
	for i := range grid {
		grid[i] = make([]string, maxX+1)
		for j := range grid[i] {
			grid[i][j] = " "
		}
	}

	for pos, val := range maze {
		grid[pos.y][pos.x] = val
	}

	for _, row := range grid {
		fmt.Println(row)
	}
}

func findPath(maze map[Position]string, start, end Position) ([]Position, int, int) {
	pq := &PriorityQueue{}
	heap.Init(pq)

	visited := make(map[Position]map[Position]bool)
	startNode := &Node{pos: start, dir: Position{0, 1}, cost: 0, turns: 0, steps: 0, priority: 0}
	heap.Push(pq, startNode)

	for pq.Len() > 0 {
		current := heap.Pop(pq).(*Node)

		// If we reached the end, return the path
		if current.pos == end {
			return reconstructPath(current), current.turns, current.steps
		}

		// Mark current position as visited in the current direction
		if _, ok := visited[current.pos]; !ok {
			visited[current.pos] = make(map[Position]bool)
		}
		visited[current.pos][current.dir] = true

		// Explore neighbors
		for _, dir := range directions {
			nextPos := Position{x: current.pos.x + dir.x, y: current.pos.y + dir.y}
			if maze[nextPos] == "#" || visited[nextPos][dir] {
				continue
			}

			// Calculate cost
			stepCost := current.cost + 1
			turnCost := 1000
			newDir := dir != current.dir
			newCost := stepCost
			if newDir {
				newCost += turnCost
			}

			// Heuristic (Manhattan distance to the end)
			heuristic := abs(nextPos.x-end.x) + abs(nextPos.y-end.y)
			priority := newCost + heuristic

			// Create the new node
			nextNode := &Node{
				pos:      nextPos,
				dir:      dir,
				cost:     newCost,
				turns:    current.turns + btoi(newDir),
				steps:    current.steps + 1,
				priority: priority,
				prev:     current,
			}
			heap.Push(pq, nextNode)
		}
	}

	return nil, -1, -1 // No path found
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func btoi(b bool) int {
	if b {
		return 1
	}
	return 0
}

func reconstructPath(endNode *Node) []Position {
	var path []Position
	for n := endNode; n != nil; n = n.prev {
		path = append([]Position{n.pos}, path...)
	}
	return path
}
