# Advent of Code 2020 - Day 6
# https://adventofcode.com/2020/day/6
defmodule Aoc2020.Day1 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn group ->
      group |> String.graphemes() |> MapSet.new() |> MapSet.delete("\n")
    end)
    |> Enum.map(&MapSet.size/1)
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input
    |> String.split("\n\n", trim: true)
    |> Enum.map(fn group ->
      group
      |> String.split("\n", trim: true)
      |> Enum.reduce(nil, fn el, acc ->
        case acc do
          nil -> el |> String.graphemes() |> MapSet.new()
          _ -> MapSet.intersection(acc, el |> String.graphemes() |> MapSet.new())
        end
      end)
    end)
    |> Enum.map(&MapSet.size/1)
    |> Enum.sum()
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day1.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day1.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
