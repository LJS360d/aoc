#!/usr/bin/env -S uv run
# /// script
# dependencies = []
# ///

# https://adventofcode.com/2015/day/5
import itertools
import re


def part1(input: str):
    def is_nice(s: str) -> bool:
      has_vowels = sum(s.count(v) for v in "aeiou") >= 3
      has_double = any(a == b for a, b in itertools.pairwise(s))
      no_blacklist = not any(bad in s for bad in ("ab", "cd", "pq", "xy"))

      return has_vowels and has_double and no_blacklist

    return sum(1 for s in input.splitlines() if is_nice(s))

def part2(input: str):
  def is_nice(s: str) -> bool:
    has_twins = len(list(re.finditer(r"([a-z][a-z]).*\1{1}", s, re.DOTALL))) >= 1
    has_skip = len(list(re.finditer(r"([a-z])[a-z]\1{1}", s, re.DOTALL))) >= 1
    return has_twins and has_skip

  return sum(1 for s in input.splitlines() if is_nice(s))

if __name__ == "__main__":
    from pathlib import Path
    from time import perf_counter_ns

    input_file = Path(__file__).parent / "../../../inputs/2015/5.txt"
    input = input_file.read_text().rstrip()

    print("\n")
    part1_start = perf_counter_ns()
    res = part1(input)
    part1_elapsed = (perf_counter_ns() - part1_start)
    print(f"part1 result: {res}")
    print(f"part1 run in {part1_elapsed}ns")

    print("\n")
    part2_start = perf_counter_ns()
    res = part2(input)
    part2_elapsed = (perf_counter_ns() - part2_start)
    print(f"part2 result: {res}")
    print(f"part2 run in {part2_elapsed}ns")
