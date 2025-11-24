package main

import (
	"errors"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 13 # Part 2
// / https://adventofcode.com/2024/day/13#part2
func main() {
	buffer, _ := os.ReadFile("day13/input.txt")
	content := string(buffer)
	clawMachines := parse(content)
	// solve(clawMachines)
	// 148568310235378 too high
	// 1545093008498 too low
	// 3237700974427 too low
	var res int
	for _, c := range clawMachines {
		b := (c.p.y*c.a.x - c.p.x*c.a.y) / (c.b.y*c.a.x - c.b.x*c.a.y)
		a := (c.p.x - b*c.b.x) / c.a.x

		if a*c.a.x+c.b.x*b != c.p.x || a*c.a.y+c.b.y*b != c.p.y {
			continue
		}

		res += (3 * a) + b
	}
	fmt.Println(res)
}

type XYPosition struct {
	x int
	y int
}

type ClawMachine struct {
	a XYPosition
	b XYPosition
	p XYPosition
}

func parse(input string) []ClawMachine {
	clawMachines := []ClawMachine{}
	re := regexp.MustCompile(`(?m)^\s*$`)
	blocks := re.Split(input, -1)
	for _, c := range blocks {
		c = strings.Trim(c, "\n")
		lines := strings.Split(c, "\n")
		cm := ClawMachine{
			a: parseButton(lines[0]),
			b: parseButton(lines[1]),
			p: parsePrize(lines[2]),
		}
		clawMachines = append(clawMachines, cm)
	}
	return clawMachines
}

func parseButton(str string) XYPosition {
	numsRaw := strings.Split(str, ": ")[1]
	re := regexp.MustCompile(`X\+(\d+),\s*Y\+(\d+)`)
	nums := re.FindStringSubmatch(numsRaw)
	x, _ := strconv.Atoi(nums[1])
	y, _ := strconv.Atoi(nums[2])
	return XYPosition{x, y}
}

func parsePrize(str string) XYPosition {
	numsRaw := strings.Split(str, ": ")[1]
	re := regexp.MustCompile(`X=(\d+),\s*Y=(\d+)`)
	nums := re.FindStringSubmatch(numsRaw)
	x, _ := strconv.Atoi(nums[1])
	y, _ := strconv.Atoi(nums[2])
	x += 10000000000000
	y += 10000000000000
	return XYPosition{x, y}
}

func solve(clawMachines []ClawMachine) {
	minTokens := 0
	for _, c := range clawMachines {
		tokens, err := getMinTokens(&c)
		if err != nil {
			continue
		}
		minTokens += tokens
	}
	fmt.Println(minTokens)
}

func getMinTokens(claw *ClawMachine) (int, error) {
	A, B, err := SolveAB(
		claw.a.x, claw.b.x, claw.p.x,
		claw.a.y, claw.b.y, claw.p.y,
	)
	return A*3 + B*1, err
}

// Solve for A and B
func SolveAB(Xa, Xb, Ya, Yb, N, M int) (int, int, error) {
	matrix := [2][2]float64{
		{float64(Xa), float64(Xb)},
		{float64(Ya), float64(Yb)},
	}

	constants := [2]float64{float64(N), float64(M)}

	// Compute determinant of the matrix
	det := matrix[0][0]*matrix[1][1] - matrix[0][1]*matrix[1][0]
	if det == 0 {
		return 0, 0, errors.New("no unique solution exists")
	}

	// Compute inverse matrix
	invMatrix := [2][2]float64{
		{matrix[1][1] / det, -matrix[0][1] / det},
		{-matrix[1][0] / det, matrix[0][0] / det},
	}

	// Multiply inverse matrix with constants to find A and B
	A := invMatrix[0][0]*constants[0] + invMatrix[0][1]*constants[1]
	B := invMatrix[1][0]*constants[0] + invMatrix[1][1]*constants[1]

	// Ensure A and B are integers
	A = math.Round(A)
	B = math.Round(B)

	// Validate the constraints
	if A > 0 && B > 0 {
		return int(A), int(B), nil
	} else {
		return 0, 0, errors.New("no solution found")
	}
}
