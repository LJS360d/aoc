# Advent of Code 2020 - Day 20
# https://adventofcode.com/2020/day/20
defmodule Aoc2020.Day20 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    nodes =
      input
      |> String.split("\n\n", trim: true)
      |> Enum.map(fn sec ->
        [title, map_sec] = String.split(sec, ":\n", trim: true)
        id = String.split(title, " ") |> Enum.at(1) |> String.to_integer()

        matrix =
          String.split(map_sec, "\n", trim: true)
          |> Enum.map(&String.split(&1, "", trim: true))

        {id, matrix}
      end)

    handles =
      nodes
      |> Enum.flat_map(fn {id, matrix} ->
        [
          {"#{id}T", top(matrix)},
          {"#{id}B", bottom(matrix)},
          {"#{id}L", left(matrix)},
          {"#{id}R", right(matrix)}
        ]
      end)

    nodes
    |> Enum.filter(fn {id, matrix} ->
      edges = find_matching(handles, id, matrix) |> length()
      IO.inspect(edges)
      edges == 2
    end)
    |> Enum.map(fn {id, _} -> id end)
    |> IO.inspect()
    |> Enum.reduce(1, fn id, acc -> id * acc end)
  end

  def find_matching(handles, id, matrix) when is_list(matrix) do
    t = {"#{id}T", top(matrix)}
    b = {"#{id}B", bottom(matrix)}
    l = {"#{id}L", left(matrix)}
    r = {"#{id}R", right(matrix)}

    top_match = find_matching_handle(handles, t)
    bottom_match = find_matching_handle(handles, b)
    left_match = find_matching_handle(handles, l)
    right_match = find_matching_handle(handles, r)

    [top_match, bottom_match, left_match, right_match]
    |> Enum.filter(fn m -> not is_nil(m) end)
  end

  def find_matching_handle(handles, {hid, hstr}) do
    Enum.find(handles, fn {ohid, oh} ->
      hstr == oh && ohid |> String.slice(0..-1//1) != hid |> String.slice(0..-1//1)
    end)
  end

  def top(matrix) when is_list(matrix) do
    matrix |> List.first() |> Enum.join()
  end

  def left(matrix) when is_list(matrix) do
    matrix
    # rotates counter clockwise
    |> Enum.map(&Enum.reverse/1)
    |> Stream.zip()
    |> Enum.map(&Tuple.to_list/1)
    |> List.first()
    |> Enum.join()
  end

  def right(matrix) when is_list(matrix) do
    matrix
    # rotates clockwise
    |> Stream.zip()
    |> Enum.map(&Tuple.to_list/1)
    |> Enum.map(&Enum.reverse/1)
    |> List.first()
    |> Enum.join()
  end

  def bottom(matrix) when is_list(matrix) do
    matrix |> List.last() |> Enum.join()
  end

  @spec part2(String.t()) :: integer()
  def part2(_input) do
    0
  end
end

# runner
input = Path.join([__DIR__, "test_input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day20.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day20.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
