# Advent of Code 2020 - Day 9
# https://adventofcode.com/2020/day/9
defmodule Aoc2020.Day9 do
  @preamble_len 25

  @spec part1(String.t()) :: integer()
  def part1(input) do
    list =
      input
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)

    preamble =
      list
      |> Enum.slice(0..(@preamble_len - 1))

    list
    |> Enum.slice(@preamble_len..-1//1)
    |> Enum.with_index()
    |> Enum.reduce_while(preamble, fn {el, index}, acc ->
      sums =
        acc
        |> Enum.with_index()
        |> Enum.flat_map(fn {val1, i} ->
          acc
          |> Enum.slice((i + 1)..-1//1)
          |> Enum.map(fn val2 -> val1 + val2 end)
        end)
        |> MapSet.new()

      if MapSet.member?(sums, el) do
        pstart = @preamble_len - (@preamble_len - 1) + index

        {:cont,
         list
         |> Enum.slice(pstart..(pstart + @preamble_len))}
      else
        {:halt, el}
      end
    end)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    list =
      input
      |> String.split("\n")
      |> Enum.map(&String.to_integer/1)

    p2_solve(part1(input), list)
  end

  def p2_solve(target, list, st \\ 0, en \\ 1) do
    range = Enum.slice(list, st..en)
    sum = Enum.sum(range)

    if sum == target do
      min = Enum.min(range)
      max = Enum.max(range)
      min + max
    else
      cond do
        sum > target ->
          p2_solve(target, list, st + 1, st + 2)

        true ->
          p2_solve(target, list, st, en + 1)
      end
    end
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day9.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day9.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
