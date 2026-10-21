# Advent of Code 2018 - Day 1
# https://adventofcode.com/2018/day/1
defmodule Aoc2018.Day1 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.reduce(0, fn val, acc ->
      acc + String.to_integer(val)
    end)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    in_list =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn val ->
        String.to_integer(val)
      end)

    solve_p2(in_list, MapSet.new())
  end

  def solve_p2(input, set, idx \\ 0, acc \\ 0) do
    new_acc = acc + (input |> Enum.at(idx))

    if MapSet.member?(set, new_acc) do
      new_acc
    else
      solve_p2(input, set |> MapSet.put(new_acc), rem(idx + 1, length(input)), new_acc)
    end
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2018.Day1.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2018.Day1.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
