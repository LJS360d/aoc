# Advent of Code 2020 - Day 10
# https://adventofcode.com/2020/day/10
defmodule Aoc2020.Day10 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    m =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&String.to_integer/1)
      |> Enum.sort()
      |> Enum.chunk_every(2, 1, :discard)
      |> Enum.map(fn [a, b] -> b - a end)
      |> Enum.frequencies()

    # +1 to each for the 0 outlet and the max+3 device
    (Map.get(m, 1, 0) + 1) * (Map.get(m, 3, 0) + 1)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    adapters =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(&String.to_integer/1)
      |> Enum.sort()
      |> MapSet.new()

    # %{ <prev> -> count}
    p2dp(adapters, 0)
    |> Map.get(0)
  end

  def p2dp(adapters, curr, memo \\ %{}) do
    case Map.get(memo, curr) do
      nil ->
        intr =
          MapSet.new([curr + 1, curr + 2, curr + 3])
          |> MapSet.intersection(adapters)

        count = MapSet.size(intr)

        case count do
          0 ->
            Map.put(memo, curr, 1)

          _ ->
            memo = Enum.reduce(intr, memo, fn n, memo -> p2dp(adapters, n, memo) end)
            Map.put(memo, curr, Enum.reduce(intr, 0, fn n, acc -> acc + Map.get(memo, n) end))
        end

      _ ->
        memo
    end
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day10.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day10.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
