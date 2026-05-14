# Advent of Code 2020 - Day 3
# https://adventofcode.com/2020/day/3
defmodule Aoc2020.Day3 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    lines = String.split(input, "\n", trim: true)
    line_length = lines |> Enum.at(0) |> String.length()
    total_lines = lines |> Enum.count()

    map =
      lines
      |> Enum.map(fn line ->
        String.graphemes(line) |> List.to_tuple()
      end)
      |> List.to_tuple()

    Enum.reduce(1..(total_lines - 1), {0, 0}, fn new_y, {old_x, old_c} ->
      new_x = (old_x + 3) |> rem(line_length)

      c =
        old_c +
          case elem(elem(map, new_y), new_x) do
            "#" -> 1
            _ -> 0
          end

      {new_x, c}
    end)
    |> elem(1)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    lines = String.split(input, "\n", trim: true)
    line_length = lines |> Enum.at(0) |> String.length()
    total_lines = lines |> Enum.count()

    map =
      lines
      |> Enum.map(fn line ->
        String.graphemes(line) |> List.to_tuple()
      end)
      |> List.to_tuple()

    slopes = [
      {1, 1},
      {3, 1},
      {5, 1},
      {7, 1},
      {1, 2}
    ]

    Enum.map(slopes, fn {right, down} ->
      Enum.reduce(down..(total_lines - 1)//down, {0, 0}, fn new_y, {old_x, old_c} ->
        new_x = (old_x + right) |> rem(line_length)

        c =
          old_c +
            case elem(elem(map, new_y), new_x) do
              "#" -> 1
              _ -> 0
            end

        {new_x, c}
      end)
      |> elem(1)
    end)
    |> Enum.reduce(1, fn c, acc -> c * acc end)
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day3.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day3.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
