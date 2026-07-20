# Advent of Code 2020 - Day 19
# https://adventofcode.com/2020/day/19
defmodule Aoc2020.Day19 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    [rules_sec, messages_sec] = input |> String.split("\n\n", trim: true)

    rules =
      rules_sec
      |> String.split("\n", trim: true)
      |> Enum.map(fn rule ->
        [id, corpus] = String.split(rule, ": ", trim: true)

        {id,
         corpus
         |> String.replace("\"", "")
         |> String.split("|", trim: true)
         |> Enum.map(&String.trim/1)
         |> Enum.map(&String.split(&1, " ", trim: true))}
      end)
      |> Enum.sort()

    comb = eval_rule(rules, "0") |> elem(0)

    IO.inspect(comb)

    messages_sec
    |> String.split("\n", trim: true)
    |> Enum.count(fn message ->
      MapSet.member?(comb, message)
    end)
  end

  def eval_rule(rules, rule_id, memo \\ Map.new()) do
    if memo |> Map.has_key?(rule_id) do
      {Map.get(memo, rule_id), memo}
    else
      rule_body =
        case rules |> Enum.find(fn {id, _} -> id == rule_id end) do
          nil -> nil
          tup -> elem(tup, 1)
        end

      {result, new_memo} =
        case rule_body do
          [list] when is_list(list) ->
            case list do
              [bef, aft] ->
                {res1, m1} = eval_rule(rules, bef, memo)
                {res2, m2} = eval_rule(rules, aft, m1)
                {MapSet.union(res1, res2), m2}

              [en] ->
                eval_rule(rules, en, memo)
            end

          [l1, l2] when is_list(l1) and is_list(l2) ->
            {res1, m1} =
              case l1 do
                [bef, aft] ->
                  {res1, m1} = eval_rule(rules, bef, memo)
                  {res2, m2} = eval_rule(rules, aft, m1)
                  {MapSet.union(res1, res2), m2}

                [en] ->
                  eval_rule(rules, en, memo)
              end

            {res2, m2} =
              case l2 do
                [bef, aft] ->
                  {res1, m3} = eval_rule(rules, bef, m1)
                  {res2, m4} = eval_rule(rules, aft, m3)
                  {MapSet.union(res1, res2), m4}

                [en] ->
                  eval_rule(rules, en, memo)
              end

            {MapSet.union(res1, res2), m2}

          [s] when is_bitstring(s) ->
            {MapSet.new([s]), memo}

          nil ->
            {MapSet.new([rule_id]), memo}
        end

      {result, Map.put(new_memo, rule_id, result)}
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(_input) do
    0
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day19.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day19.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
