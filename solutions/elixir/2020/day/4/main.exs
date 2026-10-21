# Advent of Code 2020 - Day 4
# https://adventofcode.com/2020/day/4
defmodule Aoc2020.Day4 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split("\n\n")
    |> Enum.count(fn doc ->
      ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]
      |> Enum.all?(&String.contains?(doc, &1))
    end)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input
    |> String.split("\n\n")
    |> Enum.filter(fn doc ->
      ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]
      |> Enum.all?(&String.contains?(doc, &1))
    end)
    |> Enum.count(fn doc ->
      doc
      |> String.split("\n", trim: true)
      |> Enum.flat_map(fn line -> String.split(line, " ", trim: true) end)
      |> Enum.all?(fn prop ->
        [name, val] = String.split(prop, ":")

        case name do
          "byr" ->
            year = val |> String.to_integer()
            year in 1920..2002

          "iyr" ->
            year = val |> String.to_integer()
            year in 2010..2020

          "eyr" ->
            year = val |> String.to_integer()
            year in 2020..2030

          "hgt" ->
            case Integer.parse(val) do
              {n, "in"} -> n in 59..76
              {n, "cm"} -> n in 150..193
              _ -> false
            end

          "hcl" ->
            val |> String.match?(~r/^#[0-9a-f]{6}$/)

          "ecl" ->
            val in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

          "pid" ->
            val |> String.match?(~r/^[0-9]{9}$/)

          _ ->
            true
        end
      end)
    end)
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day4.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day4.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
