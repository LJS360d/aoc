# Advent of Code 2020 - Day 1
# https://adventofcode.com/2020/day/1
defmodule Aoc2020.Day1 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    arr = String.split(input, "\n") |> Enum.map(&String.to_integer/1)
    set = arr |> MapSet.new()

    Enum.find_value(arr, nil, fn n ->
      diff = 2020 - n

      if MapSet.member?(set, diff) do
        diff * n
      end
    end)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    arr = String.split(input, "\n") |> Enum.map(&String.to_integer/1)
    set = arr |> MapSet.new()

    Enum.find_value(arr, nil, fn x ->
      r =
        Enum.find_value(arr, nil, fn n ->
          diff = 2020 - n - x

          if MapSet.member?(set, diff) do
            [n, diff]
          end
        end)

      case r do
        nil -> nil
        [n1, n2] -> n1 * n2 * x
      end
    end)
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
