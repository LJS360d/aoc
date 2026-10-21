#!/usr/bin/env -S uv run
# /// script
# dependencies = []
# ///

# https://adventofcode.com/2015/day/1

def part1(input: str):
    return input.count("(") - input.count(")")
    
def part2(input: str):
    start = 0
    for i, s in enumerate(input):
        if s == "(":
            start += 1
        elif s == ")":
            start -= 1
        if start == -1:
            return i +1
    
if __name__ == "__main__":
    from pathlib import Path
    from time import perf_counter_ns
    
    input_file = Path(__file__).parent / "../../../inputs/2015/1.txt"
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
