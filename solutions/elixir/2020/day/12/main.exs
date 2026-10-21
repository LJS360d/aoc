# Advent of Code 2020 - Day 12
# https://adventofcode.com/2020/day/12
defmodule Aoc2020.Day12 do
  @spec part1(String.t()) :: integer()
  def part1(input) do
    f =
      exec(
        input
        |> String.split("\n", trim: true)
        |> Enum.map(fn inst ->
          {op, arg} = String.split_at(inst, 1)
          {op, String.to_integer(arg)}
        end)
      )

    abs(f.n) + abs(f.e)
  end

  def exec(instructions, state \\ %{pc: 0, n: 0, e: 0, angle: 90}) do
    if state.pc >= length(instructions) do
      state
    else
      inst = Enum.at(instructions, state.pc)
      state = Map.put(state, :pc, state.pc + 1)

      case inst do
        {"E", arg} ->
          exec(instructions, Map.put(state, :e, state.e + arg))

        {"W", arg} ->
          exec(instructions, Map.put(state, :e, state.e - arg))

        {"N", arg} ->
          exec(instructions, Map.put(state, :n, state.n + arg))

        {"S", arg} ->
          exec(instructions, Map.put(state, :n, state.n - arg))

        {"L", arg} ->
          exec(instructions, Map.put(state, :angle, normalize(state.angle - arg)))

        {"R", arg} ->
          exec(instructions, Map.put(state, :angle, normalize(state.angle + arg)))

        {"F", arg} ->
          case state.angle do
            angle when angle in [0, 360] -> exec(instructions, Map.put(state, :n, state.n + arg))
            180 -> exec(instructions, Map.put(state, :n, state.n - arg))
            90 -> exec(instructions, Map.put(state, :e, state.e + arg))
            270 -> exec(instructions, Map.put(state, :e, state.e - arg))
            _ -> raise "Invalid angle: #{state.angle}"
          end
      end
    end
  end

  @spec part2(String.t()) :: integer()
  def part2(input) do
    f =
      exec2(
        input
        |> String.split("\n", trim: true)
        |> Enum.map(fn inst ->
          {op, arg} = String.split_at(inst, 1)
          {op, String.to_integer(arg)}
        end)
      )

    abs(f.n) + abs(f.e)
  end

  def exec2(instructions, state \\ %{pc: 0, n: 0, e: 0, wp: %{n: 1, e: 10}}) do
    if state.pc >= length(instructions) do
      state
    else
      inst = Enum.at(instructions, state.pc)
      state = Map.put(state, :pc, state.pc + 1)

      case inst do
        {"E", arg} ->
          exec2(instructions, Map.put(state, :wp, Map.put(state.wp, :e, state.wp.e + arg)))

        {"W", arg} ->
          exec2(instructions, Map.put(state, :wp, Map.put(state.wp, :e, state.wp.e - arg)))

        {"N", arg} ->
          exec2(instructions, Map.put(state, :wp, Map.put(state.wp, :n, state.wp.n + arg)))

        {"S", arg} ->
          exec2(instructions, Map.put(state, :wp, Map.put(state.wp, :n, state.wp.n - arg)))

        {"L", arg} ->
          exec2(
            instructions,
            Map.put(
              state,
              :wp,
              rotate(state.wp, arg)
            )
          )

        {"R", arg} ->
          exec2(
            instructions,
            Map.put(
              state,
              :wp,
              rotate(state.wp, -arg)
            )
          )

        {"F", arg} ->
          e = state.wp.e * arg
          n = state.wp.n * arg
          exec2(instructions, Map.put(state, :e, state.e + e) |> Map.put(:n, state.n + n))
      end
    end
  end

  def normalize(deg), do: rem(rem(deg, 360) + 360, 360)

  def rotate(%{e: x, n: y}, degrees) do
    radians = degrees * (:math.pi() / 180)

    cos = :math.cos(radians)
    sin = :math.sin(radians)

    new_x = round(x * cos - y * sin)
    new_y = round(x * sin + y * cos)

    %{e: new_x, n: new_y}
  end
end

# runner
input = Path.join([__DIR__, "input.txt"]) |> File.read!()

# part 1
{time, res1} = :timer.tc(fn -> Aoc2020.Day12.part1(input) end)
IO.inspect(res1)
IO.puts("Part 1 solved in: #{time}µs\n")

# part 2
{time, res2} = :timer.tc(fn -> Aoc2020.Day12.part2(input) end)
IO.inspect(res2)
IO.puts("Part 2 solved in: #{time}µs")
