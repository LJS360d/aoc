# Advent of Code 2020 - Day 11
# https://adventofcode.com/2020/day/11
defmodule Aoc2020.Day11 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    board =
      input
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.flat_map(fn {row, i} ->
        row
        |> String.graphemes()
        |> Enum.with_index()
        |> Enum.map(fn {char, ii} -> {"#{i},#{ii}", char} end)
      end)
      |> Map.new()

    solve(board)
  end

  def get_adj(board, id) when is_bitstring(id) do
    String.split(id, ",")
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
    |> (&get_adj(board, &1)).()
  end

  def get_adj(board, {i, j}) when is_integer(i) and is_integer(j) do
    neighbors =
      [
        {i - 1, j - 1},
        {i - 1, j},
        {i - 1, j + 1},
        {i, j - 1},
        {i, j + 1},
        {i + 1, j - 1},
        {i + 1, j},
        {i + 1, j + 1}
      ]
      |> Enum.map(fn {i, j} -> "#{i},#{j}" end)

    Enum.filter(neighbors, fn n -> Map.has_key?(board, n) end)
    |> Enum.map(fn n -> Map.get(board, n) end)
  end

  def solve(state, last_state \\ %{}) do
    if Map.equal?(state, last_state) do
      Map.values(state)
      |> Enum.count(fn seat ->
        seat == "#"
      end)
    else
      new_state =
        state
        |> Enum.map(fn {id, seat} ->
          case seat do
            "." ->
              {id, seat}

            "L" ->
              all_empty_neighbors =
                get_adj(state, id)
                |> Enum.all?(fn n ->
                  n == "L" or n == "."
                end)

              new_seat_state = if all_empty_neighbors, do: "#", else: seat
              {id, new_seat_state}

            "#" ->
              occupied_neighbors_count = get_adj(state, id) |> Enum.count(fn n -> n == "#" end)
              new_seat_state = if occupied_neighbors_count >= 4, do: "L", else: seat
              {id, new_seat_state}
          end
        end)
        |> Map.new()

      solve(new_state, state)
    end
  end

  # def view(state) do
  #   # visualize the board as a string
  #   state
  #   |> Enum.map(fn {id, seat} -> {id, seat} end)
  #   |> MapSet.new()
  #   |> MapSet.to_list()
  #   |> Enum.sort()
  #   |> Enum.map(fn {_id, seat} -> seat end)
  #   |> Enum.chunk_every(10)
  #   |> Enum.map(fn row -> Enum.join(row, "") end)
  #   |> Enum.join("\n")
  #   |> IO.puts()

  #   IO.puts("\n")
  # end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    board =
      input
      |> String.split("\n", trim: true)
      |> Enum.with_index()
      |> Enum.flat_map(fn {row, i} ->
        row
        |> String.graphemes()
        |> Enum.with_index()
        |> Enum.map(fn {char, ii} -> {"#{i},#{ii}", char} end)
      end)
      |> Map.new()

    solve2(board)
  end

  def solve2(state, last_state \\ %{}) do
    if Map.equal?(state, last_state) do
      Map.values(state)
      |> Enum.count(fn seat ->
        seat == "#"
      end)
    else
      new_state =
        state
        |> Enum.map(fn {id, seat} ->
          case seat do
            "." ->
              {id, seat}

            "L" ->
              all_empty_visible =
                get_visible(state, id)
                |> Enum.map(&Map.get(state, &1))
                |> Enum.count(fn seat ->
                  seat == "#"
                end) == 0

              new_seat_state = if all_empty_visible, do: "#", else: seat
              {id, new_seat_state}

            "#" ->
              occupied_visible_count =
                get_visible(state, id)
                |> Enum.map(&Map.get(state, &1))
                |> Enum.count(fn seat ->
                  seat == "#"
                end)

              new_seat_state = if occupied_visible_count >= 5, do: "L", else: seat
              {id, new_seat_state}
          end
        end)
        |> Map.new()

      solve2(new_state, state)
    end
  end

  def get_visible(board, id) when is_bitstring(id) do
    String.split(id, ",")
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
    |> (&get_visible(board, &1)).()
  end

  def get_visible(board, {i, j}) when is_integer(i) and is_integer(j) do
    dirs = [
      {1, 0},
      {0, 1},
      {-1, 0},
      {0, -1},
      {1, 1},
      {-1, 1},
      {1, -1},
      {-1, -1}
    ]

    dirs
    |> Enum.map(fn dir -> get_seat_in_direction(board, {i, j}, dir) end)
    |> Enum.filter(fn n -> n != nil end)
  end

  def get_seat_in_direction(board, {i, j}, {di, dj}) do
    move_to = "#{i + di},#{j + dj}"

    case Map.get(board, move_to) do
      nil ->
        nil

      seat ->
        if seat == ".",
          do: get_seat_in_direction(board, {i + di, j + dj}, {di, dj}),
          else: move_to
    end
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day11.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day11.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
