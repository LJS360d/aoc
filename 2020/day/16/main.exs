# Advent of Code 2020 - Day 16
# https://adventofcode.com/2020/day/16
defmodule Aoc2020.Day16 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    [clauses_section, _, tickets_section] =
      input
      |> String.split("\n\n")

    ranges =
      clauses_section
      |> String.split("\n", trim: true)
      |> Enum.map(fn clause ->
        clause
        |> String.split(": ")
        |> Enum.at(1)
        |> String.split(" or ")
        |> Enum.map(fn r -> String.split(r, "-") |> Enum.map(&String.to_integer/1) end)
        |> Enum.map(fn [min, max] -> min..max end)
      end)

    tickets =
      String.split(tickets_section, "\n", trim: true)
      |> Enum.drop(1)
      |> Enum.map(fn ticket -> String.split(ticket, ",") |> Enum.map(&String.to_integer/1) end)

    Enum.reduce(tickets, [], fn ticket, acc ->
      acc ++
        Enum.filter(ticket, fn num ->
          not Enum.any?(ranges, fn [lo, hi] -> num in lo or num in hi end)
        end)
    end)
    |> Enum.sum()
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    [clauses_section, your_ticket_section, other_tickets_section] =
      input
      |> String.split("\n\n")

    ranges =
      clauses_section
      |> String.split("\n", trim: true)
      |> Enum.map(fn clause ->
        [name, ranges_sec] =
          clause
          |> String.split(": ")

        {name,
         ranges_sec
         |> String.split(" or ")
         |> Enum.map(fn r -> String.split(r, "-") |> Enum.map(&String.to_integer/1) end)
         |> Enum.map(fn [min, max] -> min..max end)
         |> List.to_tuple()}
      end)
      |> Map.new()

    your_ticket =
      String.split(your_ticket_section, "\n", trim: true)
      |> Enum.at(1)
      |> String.split(",")
      |> Enum.map(&String.to_integer/1)

    fields =
      String.split(other_tickets_section, "\n", trim: true)
      |> Enum.drop(1)
      |> Enum.map(fn ticket -> String.split(ticket, ",") |> Enum.map(&String.to_integer/1) end)
      |> Enum.filter(fn ticket ->
        Enum.filter(ticket, fn num ->
          not Enum.any?(ranges, fn {_, {lo, hi}} -> num in lo or num in hi end)
        end)
        |> Enum.empty?()
      end)
      |> Enum.zip()
      |> Enum.map(&Tuple.to_list/1)
      |> build_field_map(ranges)
      |> eliminate_all()

    Enum.zip(fields, your_ticket)
    |> Enum.filter(fn {k, _} -> String.starts_with?(k, "departure") end)
    |> Enum.reduce(1, fn {_, v}, acc -> v * acc end)
  end

  defp build_field_map(columns, ranges) do
    columns
    |> Enum.with_index()
    |> Enum.map(fn {col, idx} ->
      possible =
        Enum.map(col, fn num ->
          Enum.filter(ranges, fn {_, {lo, hi}} -> num in lo or num in hi end)
          |> Enum.map(&elem(&1, 0))
          |> MapSet.new()
        end)
        |> Enum.reduce(&MapSet.intersection/2)

      {idx, possible}
    end)
    |> Map.new()
  end

  defp eliminate_all(col_map, assignments \\ %{}) do
    case Enum.find(col_map, fn {_, ranges} -> MapSet.size(ranges) == 1 end) do
      nil ->
        Enum.sort_by(assignments, &elem(&1, 0))
        |> Enum.map(&elem(&1, 1))

      {col_idx, range_set} ->
        [range_name] = MapSet.to_list(range_set)
        new_assignments = Map.put(assignments, col_idx, range_name)

        new_col_map =
          col_map
          |> Enum.map(fn {idx, ranges} ->
            {idx, MapSet.delete(ranges, range_name)}
          end)
          |> Map.new()

        eliminate_all(new_col_map, new_assignments)
    end
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day16.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day16.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
