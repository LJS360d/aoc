#!/bin/bash
session=53616c7465645f5f2a561f1f5e613a8668b23d43cc0093de3e86c4781a546425996093c881b76f0de1097cab6aeb28b72d689bfe017a84bd53d9bfaaff9c0b30
year=2022
# Boilerplate code
boilerplate_code() {
    cat <<EOF
/// Advent of Code $year - Day ${1}
/// https://adventofcode.com/$year/day/${1}
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
EOF
}

# Find the highest dayN folder in the current directory
highest_day=$(ls -d src/bin/day*/ 2>/dev/null | sed -E 's/src\/bin\/day([0-9]+)\//\1/' | sort -n | tail -n 1)
highest_day=${highest_day:-0}

# Determine the next day number
next_day=$((highest_day + 1))
next_day=${1:-next_day}
day_dir="src/bin/day${next_day}"
main_file="${day_dir}/main.rs"
input_file="${day_dir}/input.txt"

# Create directories and files
mkdir -p "$day_dir"

# Create es1.go and es2.go files with boilerplate code
boilerplate_code "$next_day" >"$main_file"

# Fetch input.txt using curl
if [[ -f "$input_file" ]]; then
    echo "Input file already exists: ${input_file}"
else
    echo "Fetching input.txt for Day ${next_day}..."
    curl -s -o "$input_file" --cookie "session=$session" "https://adventofcode.com/$year/day/$next_day/input"
    if [[ -s "$input_file" ]]; then
        echo "Input file downloaded successfully: ${input_file}"
    else
        echo "Warning: Failed to download input file or file is empty. Check your session and permissions."
    fi
fi


# Output the results
echo "Created structure for day${next_day}:"
tree "$day_dir" 2>/dev/null || ls -R "$day_dir"
