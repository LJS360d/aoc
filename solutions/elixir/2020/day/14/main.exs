# Advent of Code 2020 - Day 14
# https://adventofcode.com/2020/day/14
defmodule Aoc2020.Day14 do
  import Bitwise

  @spec part1(String.t()) :: integer()
  def part1(input) do
    program = input |> String.split("\n", trim: true)
    exec(program) |> Map.values() |> Enum.sum()
  end

  def exec(program, mask \\ nil, mem \\ %{}, pc \\ 0) do
    instr = Enum.at(program, pc)
    pc = pc + 1

    if is_nil(instr) do
      mem
    else
      if String.starts_with?(instr, "mask") do
        mask = String.trim_leading(instr, "mask = ")
        exec(program, mask, mem, pc)
      else
        [addr, val] = String.trim_leading(instr, "mem[") |> String.split("] = ", trim: true)
        val = mask(mask, String.to_integer(val))
        exec(program, mask, Map.put(mem, String.to_integer(addr), val), pc)
      end
    end
  end

  def mask(mask, val) do
    bits = Integer.to_string(val, 2) |> String.pad_leading(36, "0") |> String.to_charlist()
    mask = String.to_charlist(mask)

    Enum.zip(mask, bits)
    |> Enum.map(fn
      {?X, bit} -> bit
      {mask_bit, _} -> mask_bit
    end)
    |> List.to_string()
    |> String.to_integer(2)
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    program = input |> String.split("\n", trim: true)
    execv2(program) |> Map.values() |> Enum.sum()
  end

  def execv2(program, mask \\ nil, mem \\ %{}, pc \\ 0) do
    instr = Enum.at(program, pc)
    pc = pc + 1

    if is_nil(instr) do
      mem
    else
      if String.starts_with?(instr, "mask") do
        mask = String.trim_leading(instr, "mask = ")
        execv2(program, mask, mem, pc)
      else
        [addr, val] = String.trim_leading(instr, "mem[") |> String.split("] = ", trim: true)
        addresses = mask_floating(mask, String.to_integer(addr))

        execv2(
          program,
          mask,
          Enum.reduce(addresses, mem, fn address, acc ->
            Map.put(acc, address, String.to_integer(val))
          end),
          pc
        )
      end
    end
  end

  def mask_floating(mask, addr) do
    bits = Integer.to_string(addr, 2) |> String.pad_leading(36, "0") |> String.to_charlist()
    mask = String.to_charlist(mask)

    Enum.zip(mask, bits)
    |> Enum.map(fn
      {?X, _} -> "X"
      {mask_bit, bit} -> bit ||| mask_bit
    end)
    |> List.to_string()
    |> String.split("X")
    # Right-fold to reconstruct the string from back to front
    |> List.foldr([""], fn segment, acc ->
      for tail <- acc, choice <- ["0", "1"], do: segment <> choice <> tail
    end)
    # The fold leaves an extra trailing choice at the very end, trim it off
    |> Enum.map(&String.slice(&1, 0..-2//1))
    |> Enum.map(&String.to_integer(&1, 2))
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day14.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day14.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
