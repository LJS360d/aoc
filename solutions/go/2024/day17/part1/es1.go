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

// / Advent of Code 2024 - Day 17 # Part 1
// / https://adventofcode.com/2024/day/17#part1
func main() {
	buffer, _ := os.ReadFile("day17/input.txt")
	emu := parse(string(buffer))
	run(&emu)
}

type Emu struct {
	A, B, C int
	Program []int
	Out     []string
}

func parse(content string) Emu {
	emu := Emu{}
	re := regexp.MustCompile(`Register\sA:\s(\d+)`)
	match := re.FindStringSubmatch(content)
	emu.A, _ = strconv.Atoi(match[1])
	re = regexp.MustCompile(`Register\sB:\s(\d+)`)
	match = re.FindStringSubmatch(content)
	emu.B, _ = strconv.Atoi(match[1])
	re = regexp.MustCompile(`Register\sC:\s(\d+)`)
	match = re.FindStringSubmatch(content)
	emu.C, _ = strconv.Atoi(match[1])
	re = regexp.MustCompile(`Program:\s(.*)`)
	match = re.FindStringSubmatch(content)
	programStr := match[1]
	emu.Program = sliceutils.Map(strings.Split(programStr, ","), func(s string, i int, slice []string) int {
		n, _ := strconv.Atoi(s)
		return n
	})
	return emu

}

func run(emu *Emu) {
	pc := 0
	defer func() {
		if err := recover(); err != nil {
			fmt.Println(strings.Join(emu.Out, ","))
		}
	}()
	for {
		switch emu.Program[pc] {
		case 0: // adv
			comboOp := getComboOperand(emu, pc)
			shiftAmount := math.Exp2(float64(comboOp))
			divisor := int(shiftAmount)
			emu.A = int(math.Trunc(float64(emu.A) / float64(divisor)))
			pc += 2
		case 1: // XOR
			literalOp := emu.Program[pc+1]
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
			s := strconv.Itoa(n)
			emu.Out = append(emu.Out, s)
			pc += 2
		case 6: // bdv
			comboOp := getComboOperand(emu, pc)
			shiftAmount := math.Exp2(float64(comboOp))
			divisor := int(shiftAmount)
			emu.B = int(math.Trunc(float64(emu.A) / float64(divisor)))
			pc += 2
		case 7: // bdv
			comboOp := getComboOperand(emu, pc)
			shiftAmount := math.Exp2(float64(comboOp))
			divisor := int(shiftAmount)
			emu.C = int(math.Trunc(float64(emu.A) / float64(divisor)))
			pc += 2
		}
		if pc >= len(emu.Program) {
			break
		}
	}
}

func getComboOperand(emu *Emu, pc int) int {
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
		return op
	}
}
