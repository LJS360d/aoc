#!/usr/bin/env -S uv run
# /// script
# dependencies = []
# ///

# https://adventofcode.com/2015/day/6
import re


def part1(input: str):
    def bounds(p1: tuple[int, int], p2: tuple[int, int]):
      coords = set()
      for x in range(p1[0], p2[0]):
        for y in range(p1[1], p2[1]):
          coords.add((x,y))
      return coords
    lights_on=set()
    for instr in input.splitlines():
      [match] = re.findall(r"(.+)? (\d+),(\d+) through (\d+),(\d+)", instr)
      type, [x1, y1, x2, y2] = match[0], [int(i) for i in match[1:]]
      b = bounds((x1, y1), (x2, y2))
      match type:
        case "turn off":
          for o in b:
            lights_on.discard(o)
        case "turn on":
          for o in b:
            lights_on.add(o)
        case "toggle":
          for o in b:
            if o in lights_on:
              lights_on.discard(o)
            else:
              lights_on.add(o)

    return len(lights_on)

def part2(input: str):
    return 0

if __name__ == "__main__":
    from pathlib import Path
    from time import perf_counter_ns

    input_file = Path(__file__).parent / "../../../inputs/2015/6.txt"
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
