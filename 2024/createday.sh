#!/bin/bash

# Boilerplate code
boilerplate_code() {
    cat <<EOF
package main

import (
	"fmt"
	"os"
	"strings"
)

/// Advent of Code 2024 - Day ${1} # Part ${2}
/// https://adventofcode.com/2024/day/${1}#part${2}
func main() {
	buffer, _ := os.ReadFile("day${1}/input.txt")
	content := string(buffer)
	lines := strings.Split(content, "\n")
	fmt.Println(lines)
}
EOF
}

# Find the highest dayN folder in the current directory
highest_day=$(ls -d day*/ 2>/dev/null | sed -E 's/day([0-9]+)\//\1/' | sort -n | tail -n 1)
highest_day=${highest_day:-0}

# Determine the next day number
next_day=$((highest_day + 1))
day_dir="day${next_day}"
part1_dir="${day_dir}/part1"
part2_dir="${day_dir}/part2"
es1_file="${part1_dir}/es1.go"
es2_file="${part2_dir}/es2.go"
input_file="${day_dir}/input.txt"

# Create directories and files
mkdir -p "$part1_dir" "$part2_dir"

# Create es1.go and es2.go files with boilerplate code
boilerplate_code "$next_day" 1 >"$es1_file"
boilerplate_code "$next_day" 2 >"$es2_file"

# Fetch input.txt using curl
echo "Fetching input.txt for Day ${next_day}..."
curl -s -o "$input_file" --cookie "session=53616c7465645f5f46b9e939db2dc2d63e579eed4628e498e0f2c60fa6f886790b9593394fd5f90b047109c22627c7788095e854f1076971c66c4135a4d32209" "https://adventofcode.com/2024/day/${next_day}/input"

if [[ -s "$input_file" ]]; then
    echo "Input file downloaded successfully: ${input_file}"
else
    echo "Warning: Failed to download input file or file is empty. Check your session and permissions."
fi

# Output the results
echo "Created structure for day${next_day}:"
tree "$day_dir" 2>/dev/null || ls -R "$day_dir"
