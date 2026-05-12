#!/bin/bash
session=53616c7465645f5fcab68ed7bfc5f9340aebfcbacc9a193145af62211d1e3c146a8c70e1067319089e6040552c3ce037b33666a7ed4c6017e6f3ed17e940e37f
year=2020
# Boilerplate code
boilerplate_code() {
    cat <<EOF
# Advent of Code $year - Day ${1}
# https://adventofcode.com/$year/day/${1}
defmodule Aoc$year.Day1 do
  @spec part1(String.t()) :: integer()
  def part1(_input) do
    0
  end

  @spec part2(String.t()) :: integer()
  def part2(_input) do
    0
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc$year.Day1.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc$year.Day1.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")

EOF
}

# Find the highest dayN folder in the current directory
highest_day=$(ls day/ | sort -n | tail -n 1)
highest_day=${highest_day:-0}

# Determine the next day number
next_day=$((highest_day + 1))
next_day=${1:-$next_day}
day_dir="day/${next_day}"
main_file="${day_dir}/main.exs"
input_file="${day_dir}/input.txt"

# Create directories and files
mkdir -p "$day_dir"

# Create es1.go and es2.go files with boilerplate code
if [ ! -f "$main_file" ]; then
    boilerplate_code "$next_day" >"$main_file"
fi

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
