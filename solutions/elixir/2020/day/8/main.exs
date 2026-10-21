# Advent of Code 2020 - Day 8
# https://adventofcode.com/2020/day/8
defmodule Aoc2020.Day8 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    program =
      String.split(input, "\n")

    exec(program, 0, 0, MapSet.new()) |> elem(1)
  end

  @re ~r/(.+) (.+)/m
  def exec(program, acc, pc, past_instr) do
    if MapSet.member?(past_instr, pc) do
      {:loop, acc}
    else
      case Enum.at(program, pc, nil) do
        nil ->
          {:ok, acc}

        instr ->
          n_past_instr = MapSet.put(past_instr, pc)
          [_, op, arg] = Regex.run(@re, instr)

          case op do
            "nop" ->
              exec(program, acc, pc + 1, n_past_instr)

            "acc" ->
              exec(program, acc + String.to_integer(arg), pc + 1, n_past_instr)

            "jmp" ->
              exec(program, acc, pc + String.to_integer(arg), n_past_instr)
          end
      end
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    program =
      String.split(input, "\n")

    part2_solve(program, 0)
  end

  def part2_solve(program, swap_pc) do
    instr =
      program
      |> Enum.at(swap_pc)

    new_program =
      program
      |> List.replace_at(
        swap_pc,
        instr
        |> String.replace("nop", "jmp")
        |> String.replace("jmp", "nop")
      )

    case exec(new_program, 0, 0, MapSet.new()) do
      {:ok, acc} ->
        acc

      {:loop, _} ->
        last_rel_pc =
          program
          |> Enum.slice((swap_pc + 1)..length(program))
          |> Enum.find_index(fn instr ->
            [_, op, _] = Regex.run(@re, instr)
            op == "nop" or op == "jmp"
          end)

        part2_solve(program, swap_pc + last_rel_pc + 1)
    end
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day8.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day8.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
