# Advent of Code 2020 - Day 17
# https://adventofcode.com/2020/day/17
defmodule Aoc2020.Day17 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    state =
      input
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.map(fn {row, y} ->
        row
        |> String.split("", trim: true)
        |> Enum.with_index()
        |> Enum.filter(fn {cell, _} -> cell == "#" end)
        |> Enum.map(fn {_, x} ->
          {x, y, 0}
        end)
      end)
      |> List.flatten()
      |> MapSet.new()

    Enum.reduce(1..6, state, fn _, acc -> step(acc) end)
    |> MapSet.size()
  end

  def neighbors({x, y, z}) do
    for dx <- -1..1,
        dy <- -1..1,
        dz <- -1..1,
        not (dx == 0 and dy == 0 and dz == 0),
        do: {x + dx, y + dy, z + dz}
  end

  def neighbors({x, y, z, w}) do
    for dx <- -1..1,
        dy <- -1..1,
        dz <- -1..1,
        dw <- -1..1,
        not (dx == 0 and dy == 0 and dz == 0 and dw == 0),
        do: {x + dx, y + dy, z + dz, w + dw}
  end

  def active_neighbors(state, point) do
    neighbors(point)
    |> Enum.filter(fn point -> MapSet.member?(state, point) end)
  end

  def step(state) do
    cubes_to_check =
      state
      |> Enum.flat_map(&neighbors/1)
      |> Enum.concat(state)
      |> MapSet.new()

    cubes_to_check
    |> Enum.filter(fn cube ->
      active_count = active_neighbors(state, cube) |> length()

      if MapSet.member?(state, cube) do
        active_count in 2..3
      else
        active_count == 3
      end
    end)
    |> MapSet.new()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    state =
      input
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.map(fn {row, y} ->
        row
        |> String.split("", trim: true)
        |> Enum.with_index()
        |> Enum.filter(fn {cell, _} -> cell == "#" end)
        |> Enum.map(fn {_, x} ->
          {x, y, 0, 0}
        end)
      end)
      |> List.flatten()
      |> MapSet.new()

    Enum.reduce(1..6, state, fn _, acc -> step(acc) end)
    |> MapSet.size()
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day17.part1(input) end)
IO.inspect(res1, limit: :infinity)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day17.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
