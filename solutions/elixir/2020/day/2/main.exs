# Advent of Code 2020 - Day 2
# https://adventofcode.com/2020/day/2
defmodule Aoc2020.Day2 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    String.split(input, "\n", trim: true)
    |> Enum.map(fn line -> String.split(line, ":", trim: true) end)
    |> Enum.count(fn [policy, password] ->
      [range, char] = String.split(policy, " ")
      [min, max] = String.split(range, "-") |> Enum.map(&String.to_integer/1)
      char_amount = String.graphemes(password) |> Enum.count(&(&1 == char))
      char_amount >= min && char_amount <= max
    end)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    String.split(input, "\n", trim: true)
    |> Enum.map(fn line -> String.split(line, ": ", trim: true) end)
    |> Enum.count(fn [policy, password] ->
      [range, char] = String.split(policy, " ", trim: true)
      [low, high] = String.split(range, "-") |> Enum.map(&String.to_integer/1)
      chars = String.graphemes(password) |> List.to_tuple()
      elem(chars, low - 1) == char != (elem(chars, high - 1) == char)
    end)
  end
end

# runnerx
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day2.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day2.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
