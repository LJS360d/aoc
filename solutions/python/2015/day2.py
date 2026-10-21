#!/usr/bin/env -S uv run
# /// script
# dependencies = []
# ///
 
# https://adventofcode.com/2015/day/2

def part1(input: str):
    area = 0
    for line in input.splitlines(keepends=False):
        [l, w, h] = map(int,line.split("x"))
        f1 = l*w
        f2 = w*h
        f3 = h*l
        area += 2*f1 + 2*f2 + 2*f3 + min(f1, f2, f3)
    return area

def part2(input: str):
    ribbon = 0
    for line in input.splitlines(keepends=False):
        [l, w, h] = map(int,line.split("x"))
        [l1, l2, _] = sorted([l, w, h])
        ribbon += (l1+l1+l2+l2) + (l*w*h)
    return ribbon
    
if __name__ == "__main__":
    from pathlib import Path
    from time import perf_counter_ns

    input_file = Path(__file__).parent / "../../../inputs/2015/2.txt"
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
