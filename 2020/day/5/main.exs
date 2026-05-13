# Advent of Code 2020 - Day 5
# https://adventofcode.com/2020/day/5
defmodule Aoc2020.Day1 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(fn str ->
      # split at 7th char
      {row_inst, col_inst} = str |> String.split_at(7)

      row =
        String.graphemes(row_inst)
        |> Enum.reduce({0, 127}, fn el, {low, high} ->
          case el do
            "F" -> {low, div(high + low, 2) |> trunc()}
            "B" -> {(low + div(high - low, 2) + 1) |> trunc(), high}
          end
        end)
        |> elem(0)

      col =
        String.graphemes(col_inst)
        |> Enum.reduce({0, 7}, fn el, {low, high} ->
          case el do
            "L" -> {low, div(high + low, 2) |> trunc()}
            "R" -> {(low + div(high - low, 2) + 1) |> trunc(), high}
          end
        end)
        |> elem(0)

      row * 8 + col
    end)
    |> Enum.max()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    seats =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn str ->
        # split at 7th char
        {row_inst, col_inst} = str |> String.split_at(7)

        row =
          String.graphemes(row_inst)
          |> Enum.reduce({0, 127}, fn el, {low, high} ->
            case el do
              "F" -> {low, div(high + low, 2) |> trunc()}
              "B" -> {(low + div(high - low, 2) + 1) |> trunc(), high}
            end
          end)
          |> elem(0)

        col =
          String.graphemes(col_inst)
          |> Enum.reduce({0, 7}, fn el, {low, high} ->
            case el do
              "L" -> {low, div(high + low, 2) |> trunc()}
              "R" -> {(low + div(high - low, 2) + 1) |> trunc(), high}
            end
          end)
          |> elem(0)

        row * 8 + col
      end)

    min = Enum.min(seats)
    max = Enum.max(seats)

    Enum.find((min + 1)..(max - 1), fn seat ->
      !Enum.member?(seats, seat)
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
