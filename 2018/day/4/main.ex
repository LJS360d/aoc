# Advent of Code 2018 - Day 4
# https://adventofcode.com/2018/day/4
defmodule Aoc2018.Day4 do
  def run do
    input = Path.join([__DIR__, "input.txt"]) |> File.read!()

    # part 1
    {time, res1} = :timer.tc(fn -> Aoc2018.Day4.part1(input) end)
    IO.inspect(res1)
    IO.puts("Part 1 solved in: #{time}µs\n")

    # part 2
    {time, res2} = :timer.tc(fn -> Aoc2018.Day4.part2(input) end)
    IO.inspect(res2)
    IO.puts("Part 2 solved in: #{time}µs")
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    {id, _, ranges} =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn str ->
        [_, year, month, day, hour, minute, action] =
          Regex.run(~r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)", str)

        year = String.to_integer(year)
        month = String.to_integer(month)
        day = String.to_integer(day)
        hour = String.to_integer(hour)
        minute = String.to_integer(minute)

        {DateTime.new!(Date.new!(year, month, day), Time.new!(hour, minute, 0)), action}
      end)
      |> Enum.sort_by(fn {dt, _} ->
        dt |> DateTime.to_gregorian_seconds()
      end)
      |> reduce()
      |> Enum.reduce({0, 0, []}, fn {id, ranges}, {acc_id, acc_total, acc_ranges} ->
        r_total = Enum.reduce(ranges, 0, fn r, acc -> acc + Range.size(r) end)

        if r_total > acc_total do
          {id, r_total, ranges}
        else
          {acc_id, acc_total, acc_ranges}
        end
      end)

    most_asleep_minute =
      ranges
      |> Enum.flat_map(&Range.to_list/1)
      |> Enum.group_by(& &1)
      |> Enum.map(fn {m, t} -> {m, length(t)} end)
      |> Enum.reduce({-1, 0}, fn {min, am}, {acc_min, acc_am} ->
        if acc_am > am do
          {acc_min, acc_am}
        else
          {min, am}
        end
      end)
      |> elem(0)

    id * most_asleep_minute
  end

  def reduce(log, index \\ 0, guard \\ nil, memo \\ %{}) do
    case Enum.at(log, index) do
      {dt, action} ->
        case action do
          "falls asleep" ->
            reduce(
              log,
              index + 1,
              guard,
              Map.put(memo, guard, Map.get(memo, guard) ++ [dt.minute])
            )

          "wakes up" ->
            fall_asleep_min = Map.get(memo, guard) |> List.last()

            popped =
              Map.get(memo, guard)
              |> List.pop_at(-1)
              |> elem(1)

            reduce(
              log,
              index + 1,
              guard,
              Map.put(
                memo,
                guard,
                popped ++ [fall_asleep_min..(dt.minute - 1)]
              )
            )

          shift_start ->
            [_, guard_id_str] = Regex.run(~r"Guard #(\d+) begins shift", shift_start)
            guard_id = String.to_integer(guard_id_str)

            reduce(
              log,
              index + 1,
              guard_id,
              Map.put(memo, guard_id, Map.get(memo, guard_id) || [])
            )
        end

      nil ->
        memo
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    {id, most_asleep_minute, _} =
      input
      |> String.split("\n", trim: true)
      |> Enum.map(fn str ->
        [_, year, month, day, hour, minute, action] =
          Regex.run(~r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)\] (.+)", str)

        year = String.to_integer(year)
        month = String.to_integer(month)
        day = String.to_integer(day)
        hour = String.to_integer(hour)
        minute = String.to_integer(minute)

        {DateTime.new!(Date.new!(year, month, day), Time.new!(hour, minute, 0)), action}
      end)
      |> Enum.sort_by(fn {dt, _} ->
        dt |> DateTime.to_gregorian_seconds()
      end)
      |> reduce()
      |> Enum.map(fn {id, ranges} ->
        {most_asleep_minute, times} =
          ranges
          |> Enum.flat_map(&Range.to_list/1)
          |> Enum.group_by(& &1)
          |> Enum.map(fn {m, t} -> {m, length(t)} end)
          |> Enum.reduce({-1, 0}, fn {min, am}, {acc_min, acc_am} ->
            if acc_am > am do
              {acc_min, acc_am}
            else
              {min, am}
            end
          end)

        {id, most_asleep_minute, times}
      end)
      |> Enum.reduce({-1, 0, 0}, fn {min, am, times}, {acc_min, acc_am, acc_times} ->
        if times < acc_times do
          {acc_min, acc_am, acc_times}
        else
          {min, am, times}
        end
      end)

    id * most_asleep_minute
  end
end

unless Code.ensure_loaded?(IEx) and IEx.started?() do
  if length(System.argv()) >= 0 do
    Aoc2018.Day4.run()
  end
end
