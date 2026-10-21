# Advent of Code 2018 - Day 3
# https://adventofcode.com/2018/day/3
defmodule Aoc2018.Day3 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    all_tiles =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn str ->
        # [_, id, left, top, width, height]
        Regex.run(~r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)", str)
        |> Enum.drop(2)
        |> Enum.map(&String.to_integer/1)
        |> List.to_tuple()
        |> tiles_of()
      end)
      |> List.flatten()

    all_tiles
    |> Enum.group_by(& &1)
    |> Enum.count(fn {_, v} -> length(v) >= 2 end)
  end

  def tiles_of({left, top, width, height}) do
    Enum.flat_map(0..(width - 1), fn dx ->
      Enum.map(0..(height - 1), fn dy ->
        "#{left + dx},#{top + dy}"
      end)
    end)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    all_squares =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn str ->
        # [_, id, left, top, width, height]
        Regex.run(~r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)", str)
        |> Enum.drop(1)
        |> Enum.map(&String.to_integer/1)
        |> List.to_tuple()
      end)

    Enum.find(all_squares, fn sq1 ->
      !Enum.any?(all_squares, fn sq2 ->
        elem(sq1, 0) != elem(sq2, 0) && overlaps?(sq1, sq2)
      end)
    end)
    |> elem(0)
  end

  def overlaps?(
        {_id1, left1, top1, width1, height1},
        {_id2, left2, top2, width2, height2}
      ) do
    !(left1 >= left2 + width2 ||
        left2 >= left1 + width1 ||
        top1 >= top2 + height2 ||
        top2 >= top1 + height1)
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2018.Day3.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2018.Day3.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
