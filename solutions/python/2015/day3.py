#!/usr/bin/env -S uv run
# /// script
# dependencies = []
# ///

# https://adventofcode.com/2015/day/3


def part1(input: str):
    visited = set()
    x, y = 0, 0
    for n in input:
        match n:
            case ">":
                x += 1
            case "<":
                x -= 1
            case "v":
                y += 1
            case "^":
                y -= 1
        visited.add(f"{x},{y}")
    return len(visited)

def part2(input: str):
    visited = set()
    sx, sy, rx, ry = 0, 0, 0, 0
    for i, n in enumerate(input):
        if i % 2 == 0:
            match n:
                case ">":
                    sx += 1
                case "<":
                    sx -= 1
                case "v":
                    sy += 1
                case "^":
                    sy -= 1
            visited.add(f"{sx},{sy}")
        else:
            match n:
                case ">":
                    rx += 1
                case "<":
                    rx -= 1
                case "v":
                    ry += 1
                case "^":
                    ry -= 1
            visited.add(f"{rx},{ry}")
    return len(visited)


if __name__ == "__main__":
    from pathlib import Path
    from time import perf_counter_ns

    input_file = Path(__file__).parent / "../../../inputs/2015/3.txt"
    input = input_file.read_text().rstrip()

    print("\n")
    part1_start = perf_counter_ns()
    res = part1(input)
    part1_elapsed = perf_counter_ns() - part1_start
    print(f"part1 result: {res}")
    print(f"part1 run in {part1_elapsed}ns")

    print("\n")
    part2_start = perf_counter_ns()
    res = part2(input)
    part2_elapsed = perf_counter_ns() - part2_start
    print(f"part2 result: {res}")
    print(f"part2 run in {part2_elapsed}ns")
