# Advent of Code 2020 - Day 21
# https://adventofcode.com/2020/day/21
defmodule Aoc2020.Day21 do
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
{time, res1} = :timer.tc(fn -> Aoc2020.Day21.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day21.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")

