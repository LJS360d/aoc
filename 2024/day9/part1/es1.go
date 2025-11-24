package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	"github.com/Goldziher/go-utils/sliceutils"
)

// / Advent of Code 2024 - Day 9 # Part 1
// / https://adventofcode.com/2024/day/9#part1
func main() {
	buffer, _ := os.ReadFile("day9/input.txt")
	content := string(buffer)
	diskMap := parseLine(content)
	decomp := decompress(diskMap)
	rearranged := rearrange(decomp)
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

func decompress(diskMap []int) []string {
	result := make([]string, 0)
	id := 0
	for i := 0; i < len(diskMap); i++ {
		if i%2 != 0 {
			freeSpace := make([]string, diskMap[i])
			for j := 0; j < len(freeSpace); j++ {
				freeSpace[j] = "."
			}
			result = append(result, freeSpace...)
		} else {
			fileBlock := make([]string, diskMap[i])
			for j := 0; j < len(fileBlock); j++ {
				fileBlock[j] = strconv.Itoa(id)
			}
			id++
			result = append(result, fileBlock...)
		}
	}
	return result
}

func rearrange(decomp []string) []string {
	for i := 0; i < len(decomp); i++ {
		if decomp[i] != "." {
			continue
		}
		if sliceutils.Every(decomp[i:], func(v string, i int, s []string) bool {
			return v == "."
		}) {
			break
		}
		l := getLastPopulatedIndex(decomp)
		decomp[i] = decomp[l]
		decomp[l] = "."
	}
	return decomp
}

func getLastPopulatedIndex(decomp []string) int {
	for i := len(decomp) - 1; i > 0; i-- {
		if decomp[i] == "." {
			continue
		}
		return i
	}
	return -1
}

func getChecksum(rearranged []string) int {
	i := getLastPopulatedIndex(rearranged)
	usedSpace := rearranged[:i+1]
	res := 0
	for i := 0; i < len(usedSpace); i++ {
		n, _ := strconv.Atoi(usedSpace[i])
		res += i * n
	}
	return res
}
