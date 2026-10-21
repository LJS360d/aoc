package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 9 # Part 2
// / https://adventofcode.com/2024/day/9#part2
func main() {
	buffer, _ := os.ReadFile("day9/input.txt")
	content := string(buffer)
	diskMap := parseLine(content)
	decomp := decompress(diskMap)
	rearranged := rearrange(decomp, 9999999999999)
	Visualize(rearranged)
	fmt.Println(getChecksum(rearranged))
}

func parseLine(line string) []int {
	slice := make([]int, 0)
	for _, v := range strings.Split(line, "") {
		n, _ := strconv.Atoi(v)
		slice = append(slice, n)
	}
	return slice
}

func decompress(diskMap []int) []int {
	result := make([]int, 0)
	id := 0
	for i := 0; i < len(diskMap); i++ {
		if i%2 != 0 {
			freeSpace := make([]int, diskMap[i])
			for j := 0; j < len(freeSpace); j++ {
				freeSpace[j] = -1
			}
			result = append(result, freeSpace...)
		} else {
			fileBlock := make([]int, diskMap[i])
			for j := 0; j < len(fileBlock); j++ {
				fileBlock[j] = id
			}
			id++
			result = append(result, fileBlock...)
		}
	}
	return result
}

func rearrange(decomp []int, under int) []int {
	for i := 0; i < len(decomp); i++ {
		if decomp[i] != -1 {
			continue
		}
		if under == 0 {
			break
		}
		f1, f2 := getFileBlock(decomp, under)
		block := decomp[f1:f2]
		s1, s2 := getFreeSpaceBlock(decomp, len(block))
		under = block[0]
		if s1 == -1 || s1 >= f1 {
			continue
		}
		freeSpaceLen := s2 - s1 + 1
		if len(block) <= freeSpaceLen {
			for j := 0; j < len(block); j++ {
				decomp[s1+j] = block[j]
			}
		}
		for j := f1; j < f2; j++ {
			decomp[j] = -1
		}

	}
	return decomp
}

func getFileBlock(decomp []int, under int) (int, int) {
	for i := len(decomp) - 1; i > 0; i-- {
		if decomp[i] == -1 {
			continue
		}
		if decomp[i] >= under {
			continue
		}
		f := sliceutils.FindIndexOf(decomp, decomp[i])
		return f, i + 1
	}
	return -1, -1
}

func getFreeSpaceBlock(decomp []int, minSize int) (int, int) {
	for i := 0; i < len(decomp); i++ {
		if decomp[i] == -1 {
			for j := 0; j < len(decomp[i-1:]); j++ {
				if i+j >= len(decomp) || decomp[i+j] == -1 {
					continue
				}
				m := i + j - 1
				freeSpaceLen := m - i + 1
				if freeSpaceLen < minSize {
					i = m
					break
				}
				return i, m
			}
		}
	}
	return -1, -1
}

func getChecksum(rearranged []int) int {
	res := 0
	for i := 0; i < len(rearranged); i++ {
		id := rearranged[i]
		if id == -1 {
			continue
		}
		res += id * i
	}
	return res
}

func Visualize(rearranged []int) {
	fmt.Println(sliceutils.Map(rearranged, func(v int, i int, slice []int) string {
		if v == -1 {
			return "."
		}
		return strconv.Itoa(v)
	}))
}
