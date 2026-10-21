# Advent of Code 2020 - Day 15
# https://adventofcode.com/2020/day/15
defmodule Aoc2020.Day15 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split(",", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> play(2020)
  end

  def play(starters, until, turn \\ 1, last \\ nil, mem \\ %{}) do
    z =
      case Map.get(mem, last) do
        nil -> nil
        {_, z} -> z
      end

    mem = Map.put(mem, last, {z, turn})

    if turn == until + 1 do
      last
    else
      if turn - 1 < length(starters) do
        n = Enum.at(starters, turn - 1)
        play(starters, until, turn + 1, n, mem)
      else
        case Map.get(mem, last) do
          nil ->
            play(starters, until, turn + 1, 0, mem)

          {nil, _} ->
            play(starters, until, turn + 1, 0, mem)

          {n, m} ->
            play(starters, until, turn + 1, m - n, mem)
        end
      end
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input
    |> String.split(",", trim: true)
    |> Enum.map(&String.to_integer/1)
    |> play(30_000_000)
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day15.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day15.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
