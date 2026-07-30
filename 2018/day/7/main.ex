# Advent of Code 2018 - Day 7
# https://adventofcode.com/2018/day/7
defmodule Aoc2018.Day7 do
  def run do
    input = Path.join([__DIR__, "input.txt"]) |> File.read!()

    # part 1
    {time, res1} = :timer.tc(fn -> Aoc2018.Day7.part1(input) end)
    IO.inspect(res1)
    IO.puts("Part 1 solved in: #{time}µs\n")

    # part 2
    {time, res2} = :timer.tc(fn -> Aoc2018.Day7.part2(input) end)
    IO.inspect(res2)
    IO.puts("Part 2 solved in: #{time}µs")
  end

  @spec part1(String.t()) :: integer()
  def part1(input) do
    edges =
      input
      |> String.split("\n", trim: true)
      |> Enum.reduce(%{}, fn str, acc ->
        [_, from, to] = Regex.run(~r"Step (.) must be finished before step (.) can begin.", str)
        Map.put(acc, to, ((Map.get(acc, to) || []) ++ [from]) |> Enum.sort())
      end)

    {at, no_deps} =
      MapSet.difference(
        Map.values(edges) |> Enum.flat_map(& &1) |> MapSet.new(),
        Map.keys(edges) |> MapSet.new()
      )
      |> MapSet.to_list()
      |> Enum.sort()
      |> List.pop_at(0)

    edges =
      Enum.reduce(no_deps, edges, fn node, acc ->
        Map.put(acc, node, [])
      end)

    explore(edges, [at])
  end

  def explore(edges, memo) do
    at = memo |> List.last()

    edges =
      Map.new(edges |> Map.keys(), fn k ->
        {k, Map.get(edges, k) |> List.delete(at)}
      end)

    empty_map = %{}

    case edges do
      ^empty_map ->
        memo |> List.to_string()

      edges ->
        next =
          Map.filter(edges, fn {_k, v} -> v == [] end)
          |> Map.keys()
          |> Enum.sort()
          |> List.first()

        explore(edges |> Map.delete(next), memo ++ [next])
    end
  end

  @workers 5
  @base_duration 60

  @spec part2(String.t()) :: integer()
  def part2(input) do
    tasks =
      input
      |> String.split("\n", trim: true)
      |> Enum.reduce(%{}, fn str, acc ->
        [_, from, to] = Regex.run(~r"Step (.) must be finished before step (.) can begin.", str)
        Map.put(acc, to, ((Map.get(acc, to) || []) ++ [from]) |> Enum.sort())
      end)

    no_deps =
      MapSet.difference(
        Map.values(tasks) |> Enum.flat_map(& &1) |> MapSet.new(),
        Map.keys(tasks) |> MapSet.new()
      )
      |> MapSet.to_list()
      |> Enum.sort()

    tasks =
      Enum.reduce(no_deps, tasks, fn node, acc ->
        Map.put(acc, node, [])
      end)

    resolve(
      tasks,
      Enum.map(1..@workers, fn _ ->
        %{task: nil, done_in: 0}
      end)
    )
  end


  def resolve(tasks, workers, done \\ [], s \\ 0)
  
  def resolve(%{} = tasks, workers, _done, s) when map_size(tasks) == 0 do
    s + Enum.reduce(workers, 0, fn w, acc -> max(w.done_in, acc) end)
  end

  def resolve(tasks, workers, done, s) do
    todo =
      tasks
      |> Map.filter(fn {_k, v} ->
        v == []
      end)
      |> Map.keys()

    # assign to workers
    workers =
      Enum.reduce_while(todo, workers, fn task, acc ->
        # get first available worker
        i = Enum.find_index(acc, fn w -> w.done_in <= 0 end)

        case i do
          nil ->
            # no available workers, cannot assign task
            {:halt, acc}

          i ->
            {:cont,
             acc
             |> List.replace_at(i, %{
               task: task,
               done_in: task_time(task)
             })}
        end
      end)

    # remove claimed tasks from pool
    tasks =
      Map.reject(tasks, fn {k, _v} ->
        Enum.any?(workers, fn w ->
          w.task == k
        end)
      end)

    # add-up worktime
    {workers, finished_tasks} =
      Enum.map_reduce(workers, [], fn w, acc ->
        if !is_nil(w.task) && w.done_in == 1 do
          {%{task: nil, done_in: 0}, acc ++ [w.task]}
        else
          {%{task: w.task, done_in: w.done_in - 1}, acc}
        end
      end)

    # remove finished tasks from dependencies
    {_, tasks} = Enum.map_reduce(finished_tasks, tasks, fn task, tasks -> 
      {task, tasks 
        |> Enum.map(fn {k, v} -> 
          {k, v |> Enum.filter(& (&1 != task))}
      end) 
        |> Map.new()} 
    end)

    resolve(tasks, workers, done ++ finished_tasks, s + 1)
  end



  def task_time(task) do
    [char] = String.to_charlist(task |> String.upcase())
    index = char - ?A + 1
    @base_duration + index
  end
end

unless Code.ensure_loaded?(IEx) and IEx.started?() do
  if length(System.argv()) >= 0 do
    Aoc2018.Day7.run()
  end
end
