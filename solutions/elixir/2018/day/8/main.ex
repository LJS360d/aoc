# Advent of Code 2018 - Day 8
# https://adventofcode.com/2018/day/8
defmodule Aoc2018.Day8 do
  def run do
    input = Path.join([__DIR__, "test_input.txt"]) |> File.read!()

    # part 1
    {time, res1} = :timer.tc(fn -> Aoc2018.Day8.part1(input) end)
    IO.inspect(res1)
    IO.puts("Part 1 solved in: #{time}µs\n")

    # part 2
    {time, res2} = :timer.tc(fn -> Aoc2018.Day8.part2(input) end)
    IO.inspect(res2)
    IO.puts("Part 2 solved in: #{time}µs")
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.trim()
    |> String.split(" ")
    |> Enum.map(&String.to_integer/1)
    |> treeify()
    |> Enum.sum()
  end

  def treeify(list, head \\ 0, tail \\ 0, meta \\ []) do
    [n_child, n_metas] = Enum.drop(list, head) |> Enum.take(2)

    meta =
      meta ++ (Enum.reverse(list) |> Enum.drop(-tail) |> Enum.take(n_metas) |> Enum.reverse())

    dbg()

    meta ++
      Enum.flat_map(1..n_child//1, fn n ->
        head = head + 2 + floor((length(list) - n_metas - 2) / n_child * (n - 1))
        tail = -length(list) + rem(n_metas * n_child * n, length(list)) + 1
        treeify(list, head, tail, meta)
      end)
  end

  @spec part2(String.t()) :: integer()
  def part2(_input) do
    0
  end
end

unless Code.ensure_loaded?(IEx) and IEx.started?() do
  if length(System.argv()) >= 0 do
    Aoc2018.Day8.run()
  end
end
