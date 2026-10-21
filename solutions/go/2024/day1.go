///usr/bin/env go run "$0" "$@"; exit $?
// https://adventofcode.com/2024/day/1

package main

import (
	_ "embed"
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"strings"
)

func main() {
	_, filename, _, _ := runtime.Caller(0)
	inputPath := filepath.Join(filepath.Dir(filename), "../../../inputs/2024/1.txt")

	content, err := os.ReadFile(inputPath)
	if err != nil {
		panic(err)
	}

	input := strings.TrimSpace(string(content))
	fmt.Println(input)
}
