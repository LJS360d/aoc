package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

// / Advent of Code 2024 - Day 12 # Part 2
// / https://adventofcode.com/2024/day/12#part2
func main() {
	in, _ := os.Open("day12/input.txt")
	defer in.Close()
	var input [][]byte
	var scanner = bufio.NewScanner(in)
	for scanner.Scan() {
		var line = scanner.Text()
		input = append(input, []byte(line))
	}

	fmt.Println(solve(input))
}

func solve(input [][]byte) int {
	var totalPrice = 0

	var visited = make(map[string]bool)

	for i := 0; i < len(input); i++ {
		for j := 0; j < len(input[0]); j++ {
			if !visited[strconv.Itoa(i)+","+strconv.Itoa(j)] {
				var area, _, numSides = findAreaAndPerimeter(i, j, visited, input)
				totalPrice += area * numSides
			}
		}
	}

	return totalPrice
}

func findAreaAndPerimeter(i, j int, visited map[string]bool, input [][]byte) (int, int, int) {
	var area, perimeter int

	var stack = [][]int{{i, j}}
	visited[strconv.Itoa(i)+","+strconv.Itoa(j)] = true

	var topSides, bottomSides, leftSides, rightSides [][]int

	for len(stack) > 0 {
		i, j = stack[len(stack)-1][0], stack[len(stack)-1][1]
		stack = stack[:len(stack)-1]

		if i-1 >= 0 && input[i-1][j] == input[i][j] {
			if !visited[strconv.Itoa(i-1)+","+strconv.Itoa(j)] {
				stack = append(stack, []int{i - 1, j})
				visited[strconv.Itoa(i-1)+","+strconv.Itoa(j)] = true
			}
		} else {
			perimeter++
			topSides = append(topSides, []int{i, j})
		}
		if i+1 < len(input) && input[i+1][j] == input[i][j] {
			if !visited[strconv.Itoa(i+1)+","+strconv.Itoa(j)] {
				stack = append(stack, []int{i + 1, j})
				visited[strconv.Itoa(i+1)+","+strconv.Itoa(j)] = true
			}
		} else {
			perimeter++
			bottomSides = append(bottomSides, []int{i, j})
		}
		if j-1 >= 0 && input[i][j-1] == input[i][j] {
			if !visited[strconv.Itoa(i)+","+strconv.Itoa(j-1)] {
				stack = append(stack, []int{i, j - 1})
				visited[strconv.Itoa(i)+","+strconv.Itoa(j-1)] = true
			}
		} else {
			perimeter++
			leftSides = append(leftSides, []int{i, j})
		}
		if j+1 < len(input[0]) && input[i][j+1] == input[i][j] {
			if !visited[strconv.Itoa(i)+","+strconv.Itoa(j+1)] {
				stack = append(stack, []int{i, j + 1})
				visited[strconv.Itoa(i)+","+strconv.Itoa(j+1)] = true
			}
		} else {
			perimeter++
			rightSides = append(rightSides, []int{i, j})
		}
		area++
	}

	sort.Sort(ByFirst(topSides))
	sort.Sort(ByFirst(bottomSides))
	sort.Sort(BySecond(leftSides))
	sort.Sort(BySecond(rightSides))

	var numSides = perimeter

	for i := 1; i < len(topSides); i++ {
		if topSides[i][0] == topSides[i-1][0] && topSides[i][1] == topSides[i-1][1]+1 {
			numSides--
		}
	}

	for i := 1; i < len(bottomSides); i++ {
		if bottomSides[i][0] == bottomSides[i-1][0] && bottomSides[i][1] == bottomSides[i-1][1]+1 {
			numSides--
		}
	}

	for i := 1; i < len(leftSides); i++ {
		if leftSides[i][0] == leftSides[i-1][0]+1 && leftSides[i][1] == leftSides[i-1][1] {
			numSides--
		}
	}

	for i := 1; i < len(rightSides); i++ {
		if rightSides[i][0] == rightSides[i-1][0]+1 && rightSides[i][1] == rightSides[i-1][1] {
			numSides--
		}
	}

	return area, perimeter, numSides
}

type ByFirst [][]int

func (a ByFirst) Len() int      { return len(a) }
func (a ByFirst) Swap(i, j int) { a[i], a[j] = a[j], a[i] }
func (a ByFirst) Less(i, j int) bool {
	if a[i][0] == a[j][0] {
		return a[i][1] < a[j][1]
	}
	return a[i][0] < a[j][0]
}

type BySecond [][]int

func (a BySecond) Len() int      { return len(a) }
func (a BySecond) Swap(i, j int) { a[i], a[j] = a[j], a[i] }
func (a BySecond) Less(i, j int) bool {
	if a[i][1] == a[j][1] {
		return a[i][0] < a[j][0]
	}
	return a[i][1] < a[j][1]
}
