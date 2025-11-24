package main

import (
	"errors"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

// / Advent of Code 2024 - Day 13 # Part 1
// / https://adventofcode.com/2024/day/13#part1
func main() {
	buffer, _ := os.ReadFile("day13/input.txt")
	content := string(buffer)
	clawMachines := parse(content)
	solve(clawMachines)
	// 33396 too high
	// 32026 !!!
}

type XYPosition struct {
	X int
	Y int
}

type ClawMachine struct {
	A     XYPosition
	B     XYPosition
	prize XYPosition
}

func parse(input string) []ClawMachine {
	clawMachines := []ClawMachine{}
	re := regexp.MustCompile(`(?m)^\s*$`)
	blocks := re.Split(input, -1)
	for _, c := range blocks {
		c = strings.Trim(c, "\n")
		lines := strings.Split(c, "\n")
		cm := ClawMachine{
			A:     parseButton(lines[0]),
			B:     parseButton(lines[1]),
			prize: parsePrize(lines[2]),
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
		claw.A.X, claw.B.X, claw.prize.X,
		claw.A.Y, claw.B.Y, claw.prize.Y,
	)
	return A*3 + B*1, err
}

// SolveAB finds the smallest positive integers A and B that satisfy:
// A * Xa + B * Xb = N
// A * Ya + B * Yb = M
func SolveAB(Xa, Xb, N, Ya, Yb, M int) (int, int, error) {
	found := false
	var A, B int

	// Iterate through all possible values of A and B
	for A = 1; A < 100; A++ {
		for B = 1; B < 100; B++ {
			if A*Xa+B*Xb == N && A*Ya+B*Yb == M {
				found = true
				break
			}
		}
		if found {
			break
		}
	}
	if !found {
		return 0, 0, errors.New("no solution found")
	}

	return A, B, nil
}
