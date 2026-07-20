# Advent of Code 2018 - Day 2
# https://adventofcode.com/2018/day/2
defmodule Aoc2018.Day2 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.reduce({0, 0}, fn val, {twos, threes} ->
      dups =
        String.split(val, "", trim: true)
        |> Enum.group_by(&String.first/1)
        |> Enum.map(fn {_key, val} ->
          length(val)
        end)

      has_two =
        dups
        |> Enum.any?(fn val ->
          val == 2
        end)

      has_three = dups |> Enum.any?(fn val -> val == 3 end)

      {twos + ((has_two && 1) || 0), threes + ((has_three && 1) || 0)}
    end)
    |> Tuple.product()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input =
      input
      |> String.split("\n", trim: true)

    [id1, id2] =
      input
      |> Enum.filter(fn id ->
        iid =
          Enum.find(input, fn iid ->
            jaro = String.jaro_distance(id, iid)
            jaro >= 0.97 && jaro != 1.0
          end)

        iid != nil
      end)
      |> Enum.map(&String.to_charlist/1)

    Enum.filter(id1 |> Enum.with_index(), fn {char, i} ->
      Enum.at(id2, i) == char
    end)
    |> Enum.map(fn {ch, _} -> ch end)
    |> to_string()
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2018.Day2.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2018.Day2.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
