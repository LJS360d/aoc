# Advent of Code 2020 - Day 13
# https://adventofcode.com/2020/day/13
defmodule Aoc2020.Day13 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    [timestamp, buses] = String.split(input, "\n")
    timestamp = String.to_integer(timestamp)

    {id, offset} =
      String.split(buses, ",")
      |> Enum.map(&Integer.parse/1)
      |> Enum.reject(fn b -> b == :error end)
      |> Enum.map(&elem(&1, 0))
      |> Enum.map(fn b -> {b, b - rem(timestamp, b)} end)
      |> Enum.min_by(fn {_, r} -> r end)

    id * offset
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    [_, buses] = String.split(input, "\n")

    buses =
      String.split(buses, ",")
      |> Enum.with_index()
      |> Enum.map(fn {b, i} -> {i, Integer.parse(b)} end)
      |> Enum.reject(fn {_, b} -> b == :error end)
      |> Enum.map(fn {i, {n, _}} -> {i, n} end)

    solve(buses)
  end

  defp solve(buses) do
    [{_, first_bus} | rest] = buses

    Enum.reduce(rest, {0, first_bus}, fn {offset, bus}, {t, step} ->
      t = find_t(t, step, bus, offset)
      {t, lcm(step, bus)}
    end)
    |> elem(0)
  end

  def find_t(t, step, bus, offset) do
    if rem(t + offset, bus) == 0 do
      t
    else
      find_t(t + step, step, bus, offset)
    end
  end

  def lcm(a, b) do
    div(a * b, Integer.gcd(a, b))
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day13.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day13.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
