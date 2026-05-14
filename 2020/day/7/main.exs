# Advent of Code 2020 - Day 7
# https://adventofcode.com/2020/day/7
defmodule Aoc2020.Day1 do
  @target "shiny gold"

  @spec part1(String.t()) :: integer()
  def part1(input) do
    find_containers(@target, input) |> Enum.uniq() |> length()
  end

  def find_containers(target, input) do
    re = ~r/^(.*)\sbags contain.*\d\s#{target} bags?/m
    colors = Regex.scan(re, input) |> Enum.map(&Enum.at(&1, 1))
    colors ++ Enum.flat_map(colors, &find_containers(&1, input))
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    count_children(input, @target)
  end

  def count_children(input, target) do
    re = ~r/^#{target} bags contain (.*)\./m

    [_, children] = Regex.run(re, input)

    children
    |> String.split(",", trim: true)
    |> Enum.map(fn bag ->
      if bag == "no other bags" do
        nil
      else
        [_, n, color] = Regex.run(~r/(\d) (.*) bags?/, bag)
        {String.to_integer(n), color}
      end
    end)
    |> Enum.map(fn i ->
      case i do
        {n, color} ->
          n * (count_children(input, color) + 1)

        nil ->
          0
      end
    end)
    |> Enum.sum()
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
# 184295 too high
