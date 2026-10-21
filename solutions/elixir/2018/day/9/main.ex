# Advent of Code 2018 - Day 9
# https://adventofcode.com/2018/day/9
defmodule Aoc2018.Day9 do
  def run do
    input = Path.join([__DIR__, "input.txt"]) |> File.read!()

    # part 1
    {time, res1} = :timer.tc(fn -> Aoc2018.Day9.part1(input) end)
    IO.inspect(res1)
    IO.puts("Part 1 solved in: #{time}µs\n")

    # part 2
    {time, res2} = :timer.tc(fn -> Aoc2018.Day9.part2(input) end)
    IO.inspect(res2)
    IO.puts("Part 2 solved in: #{time}µs")
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    [players, marbles] =
      Regex.run(~r"(\d+) players; last marble is worth (\d+) points", input)
      |> Enum.drop(1)
      |> Enum.map(&String.to_integer/1)
  end

  @spec part2(String.t()) :: integer()
  def part2(_input) do
    0
  end
end

unless Code.ensure_loaded?(IEx) and IEx.started?() do
  if length(System.argv()) >= 0 do
    Aoc2018.Day9.run()
  end
end
