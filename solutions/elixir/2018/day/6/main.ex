# Advent of Code 2018 - Day 6
# https://adventofcode.com/2018/day/6
defmodule Aoc2018.Day6 do
  def run do
    input = Path.join([__DIR__, "input.txt"]) |> File.read!()

    # part 1
    {time, res1} = :timer.tc(fn -> Aoc2018.Day6.part1(input) end)
    IO.inspect(res1)
    IO.puts("Part 1 solved in: #{time}µs\n")

    # part 2
    {time, res2} = :timer.tc(fn -> Aoc2018.Day6.part2(input) end)
    IO.inspect(res2)
    IO.puts("Part 2 solved in: #{time}µs")
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    points =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn coords ->
        coords |> String.split(", ") |> Enum.map(&String.to_integer/1) |> List.to_tuple()
      end)

    {bx, by} =
      points
      |> Enum.reduce({0, 0}, fn {x, y}, {ax, ay} ->
        {max(x, ax), max(y, ay)}
      end)

    inf_scalers =
      (Enum.flat_map(-1..(bx + 1), fn x ->
         [{x, -1}, {x, by + 1}]
       end) ++
         Enum.flat_map(0..by, fn y ->
           [{bx + 1, y}, {-1, y}]
         end))
      |> Enum.map(fn ap ->
        {_dist, closest_point} =
          Enum.reduce(points, {:infinity, nil}, fn pm, {acc, acc_p} ->
            dist = distance(ap, pm)

            cond do
              acc == 0 or dist == acc -> {dist, 0}
              dist < acc -> {dist, pm}
              true -> {acc, acc_p}
            end
          end)

        {ap, closest_point}
      end)
      |> Enum.frequencies_by(&elem(&1, 1))
      |> Map.keys()

    area =
      Enum.flat_map(0..bx, fn x ->
        Enum.map(0..by, fn y ->
          {x, y}
        end)
      end)
      |> Enum.filter(fn p ->
        not Enum.member?(points, p)
      end)
      # find closest point
      |> Enum.map(fn ap ->
        {_dist, closest_point} =
          Enum.reduce(points, {:infinity, nil}, fn pm, {acc, acc_p} ->
            dist = distance(ap, pm)

            cond do
              acc == 0 or dist == acc -> {dist, 0}
              dist < acc -> {dist, pm}
              true -> {acc, acc_p}
            end
          end)

        {ap, closest_point}
      end)
      # eliminate areas that would go infinite without bounds
      |> Enum.filter(fn {_, cp} ->
        not Enum.member?(inf_scalers, cp)
      end)
      |> Enum.frequencies_by(&elem(&1, 1))
      |> Map.values()
      |> Enum.max()

    area + 1
  end

  def distance({x1, y1}, {x2, y2}) do
    abs(x1 - x2) + abs(y1 - y2)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    points =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn coords ->
        coords |> String.split(", ") |> Enum.map(&String.to_integer/1) |> List.to_tuple()
      end)

    {bx, by} =
      points
      |> Enum.reduce({0, 0}, fn {x, y}, {ax, ay} ->
        {max(x, ax), max(y, ay)}
      end)

    Enum.flat_map(0..bx, fn x ->
      Enum.map(0..by, fn y ->
        {x, y}
      end)
    end)
    |> Enum.filter(fn ap ->
      tdist = points |> Enum.map(&distance(ap, &1)) |> Enum.sum()
      tdist < 10000
    end)
    |> length()
  end
end

unless Code.ensure_loaded?(IEx) and IEx.started?() do
  if length(System.argv()) >= 0 do
    Aoc2018.Day6.run()
  end
end
