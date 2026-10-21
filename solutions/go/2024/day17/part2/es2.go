package main

import (
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 17 # Part 2
// / https://adventofcode.com/2024/day/17#part2
func main() {
	buffer, _ := os.ReadFile("day17/input.txt")
	emu := parse(string(buffer))
	fmt.Println(solve(emu, 0, 0))
}

func solve(emu Emu, A int64, n int) int64 {
	if n == len(emu.Program) {
		return A
	}

	for i := 0; i < 8; i++ {
		candidate := A*8 + int64(i)
		out := run(emu, candidate)

		if len(out) > 0 && out[0] == emu.Program[len(emu.Program)-(n+1)] {
			result := solve(emu, candidate, n+1)
			if result != 0 {
				return result
			}
		}
	}

	return 0
}

type Emu struct {
	A, B, C int64
	Program []int
	Out     []int
}

func parse(content string) Emu {
	emu := Emu{}

	emu.A = 0

	re := regexp.MustCompile(`Register\sB:\s(\d+)`)
	match := re.FindStringSubmatch(content)
	n, _ := strconv.Atoi(match[1])
	emu.B = int64(n)
	re = regexp.MustCompile(`Register\sC:\s(\d+)`)
	match = re.FindStringSubmatch(content)
	n, _ = strconv.Atoi(match[1])
	emu.C = int64(n)
	re = regexp.MustCompile(`Program:\s(.*)`)
	match = re.FindStringSubmatch(content)
	programStr := match[1]
	emu.Program = sliceutils.Map(strings.Split(programStr, ","), func(s string, i int, slice []string) int {
		n, _ := strconv.Atoi(s)
		return n
	})
	return emu

}

func run(emuCp Emu, A int64) []int {
	emu := &Emu{
		A:       A,
		B:       emuCp.B,
		C:       emuCp.C,
		Program: emuCp.Program,
		Out:     emuCp.Out,
	}
	pc := 0
	for {
		if pc >= len(emu.Program) {
			break
		}

		switch emu.Program[pc] {
		case 0: // adv
			comboOp := getComboOperand(emu, pc)
			shiftAmount := math.Exp2(float64(comboOp))
			divisor := int(shiftAmount)
			emu.A = int64(math.Trunc(float64(emu.A) / float64(divisor)))
			pc += 2
		case 1: // XOR
			literalOp := int64(emu.Program[pc+1])
			emu.B = emu.B ^ literalOp
			pc += 2
		case 2: // mod8
			emu.B = getComboOperand(emu, pc) % 8
			pc += 2
		case 3: // JMPIF
			if emu.A == 0 {
				pc += 2
				continue
			}
			literalOp := emu.Program[pc+1]
			pc = literalOp
		case 4: // Bitwise XOR
			_ = emu.Program[pc+1]
			emu.B = emu.B ^ emu.C
			pc += 2
		case 5: // Out mod8
			comboOp := getComboOperand(emu, pc)
			n := comboOp % 8
			// s := strconv.Itoa(n)
			emu.Out = append(emu.Out, int(n))
			pc += 2
		case 6: // bdv
			comboOp := getComboOperand(emu, pc)
			shiftAmount := math.Exp2(float64(comboOp))
			divisor := int(shiftAmount)
			emu.B = int64(math.Trunc(float64(emu.A) / float64(divisor)))
			pc += 2
		case 7: // bdv
			comboOp := getComboOperand(emu, pc)
			shiftAmount := math.Exp2(float64(comboOp))
			divisor := int(shiftAmount)
			emu.C = int64(math.Trunc(float64(emu.A) / float64(divisor)))
			pc += 2
		}
	}
	return emu.Out
}

func getComboOperand(emu *Emu, pc int) int64 {
	op := emu.Program[pc+1]
	switch op {
	case 4:
		return emu.A
	case 5:
		return emu.B
	case 6:
		return emu.C
	case 7:
		panic("Reserved operand")
	default:
		return int64(op)
	}
}
