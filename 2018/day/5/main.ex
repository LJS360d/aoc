# Advent of Code 2018 - Day 5
# https://adventofcode.com/2018/day/5
defmodule Aoc2018.Day5 do
  def run do
    input = Path.join([__DIR__, "input.txt"]) |> File.read!()

    # part 1
    {time, res1} = :timer.tc(fn -> Aoc2018.Day5.part1(input) end)
    IO.inspect(res1)
    IO.puts("Part 1 solved in: #{time}µs\n")

    # part 2
    {time, res2} = :timer.tc(fn -> Aoc2018.Day5.part2(input) end)
    IO.inspect(res2)
    IO.puts("Part 2 solved in: #{time}µs")
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.trim()
    |> reduce_polymer()
    |> String.length()
  end

  def reduce_polymer(input) do
    rep = Regex.replace(~r"([a-zA-Z])(?!\1)(?i:\1)", input, "")

    if rep == input do
      input
    else
      reduce_polymer(rep)
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input =
      input
      |> String.trim()

    units =
      input
      |> String.upcase()
      |> String.split("")
      |> Enum.uniq()
      |> Enum.drop(1)

    Enum.reduce(units, :infinity, fn unit, acc ->
      pl =
        String.replace(input, unit, "")
        |> String.replace(String.downcase(unit), "")
        |> reduce_polymer()
        |> String.length()

      if pl < acc do
        pl
      else
        acc
      end
    end)
  end
end

unless Code.ensure_loaded?(IEx) and IEx.started?() do
  if length(System.argv()) >= 0 do
    Aoc2018.Day5.run()
  end
end
