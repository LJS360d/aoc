# Advent of Code 2020 - Day 18
# https://adventofcode.com/2020/day/18
defmodule Aoc2020.Day18 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&eval/1)
    |> Enum.sum()
  end

  def parse(line) do
    line
    |> String.graphemes()
    |> Enum.filter(&(&1 != " "))
    |> Enum.map(fn
      "+" -> :+
      "*" -> :*
      "(" -> :"("
      ")" -> :")"
      x -> String.to_integer(x)
    end)
  end

  def eval(expr) do
    tokens = parse(expr)
    {result, []} = eval_expr(tokens, 0, nil)
    result
  end

  def eval_expr([:")" | rest], acc, _), do: {acc, rest}
  def eval_expr([], acc, _), do: {acc, []}

  def eval_expr([num | rest], _acc, nil) when is_integer(num) do
    eval_expr(rest, num, nil)
  end

  def eval_expr([op | rest], acc, nil) when op in [:+, :*] do
    eval_expr(rest, acc, op)
  end

  def eval_expr([num | rest], acc, op) when is_integer(num) do
    new_acc = apply_op(op, acc, num)
    eval_expr(rest, new_acc, nil)
  end

  def eval_expr([:"(" | rest], acc, op) do
    {sub, rest} = eval_expr(rest, 0, nil)
    new_acc = if op, do: apply_op(op, acc, sub), else: sub
    eval_expr(rest, new_acc, nil)
  end

  defp apply_op(:+, a, b), do: a + b
  defp apply_op(:*, a, b), do: a * b

  @spec part2(String.t()) :: integer()
  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&eval2/1)
    |> Enum.sum()
  end

  def eval2(expr) do
    tokens = parse(expr)
    {result, []} = eval_mult(tokens)
    result
  end

  def eval_mult(tokens) do
    {acc, rest} = eval_add(tokens)
    eval_mult_rest(acc, rest)
  end

  def eval_mult_rest(acc, [:* | rest]) do
    {val, rest} = eval_add(rest)
    eval_mult_rest(acc * val, rest)
  end

  def eval_mult_rest(acc, rest) do
    {acc, rest}
  end

  def eval_add(tokens) do
    {acc, rest} = eval_primary(tokens)
    eval_add_rest(acc, rest)
  end

  def eval_add_rest(acc, [:+ | rest]) do
    {val, rest} = eval_primary(rest)
    eval_add_rest(acc + val, rest)
  end

  def eval_add_rest(acc, rest) do
    {acc, rest}
  end

  def eval_primary([:"(" | rest]) do
    {val, [:")" | rest]} = eval_mult(rest)
    {val, rest}
  end

  def eval_primary([num | rest]) when is_integer(num) do
    {num, rest}
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day18.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day18.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
