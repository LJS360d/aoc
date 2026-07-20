# Advent of Code 2020 - Day 22
# https://adventofcode.com/2020/day/22
defmodule Aoc2020.Day22 do
  alias Stack

  @spec part1(String.t()) :: integer()
  def part1(input) do
    {s1, s2} =
      String.split(input, "\n\n", trim: true)
      |> Enum.map(fn sec ->
        String.split(sec, ":\n", trim: true)
        |> Enum.at(1)
        |> String.split("\n", trim: true)
        |> Enum.map(&String.to_integer/1)
        |> Enum.reverse()
        |> Stack.from_list()
      end)
      |> List.to_tuple()

    play(s1, s2)
    |> Stack.to_list()
    |> Enum.reduce({1, 0}, fn e, {i, score} ->
      {i + 1, score + e * i}
    end)
    |> elem(1)
  end

  def play(s1, s2) do
    cond do
      Stack.size(s1) == 0 ->
        s2

      Stack.size(s2) == 0 ->
        s1

      Stack.size(s2) != 0 && Stack.size(s1) != 0 ->
        {s1, p1} = Stack.pop!(s1)
        {s2, p2} = Stack.pop!(s2)

        cond do
          p1 > p2 ->
            play(
              Stack.to_list(s1)
              |> then(fn l -> [p2, p1] ++ l end)
              |> Stack.from_list(),
              s2
            )

          p2 > p1 ->
            play(s1, Stack.to_list(s2) |> then(fn l -> [p1, p2] ++ l end) |> Stack.from_list())

          p2 == p1 ->
            raise "no"
        end
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(_input) do
    0
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day22.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day22.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
