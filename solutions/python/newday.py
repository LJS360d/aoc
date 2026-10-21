#!/usr/bin/env -S uv run
import re
from pathlib import Path

base_dir = Path(__file__).parent

year_dirs = [
    int(p.name)
    for p in base_dir.iterdir()
    if p.is_dir() and p.name.isdigit() and int(p.name) >= 2015
]
year = max(year_dirs) if year_dirs else 2015
year_dir = base_dir / str(year)
year_dir.mkdir(parents=True, exist_ok=True)

day_files = [
    int(m.group(1))
    for p in year_dir.glob("day*.py")
    if (m := re.match(r"^day(\d+)\.py$", p.name))
]

if not day_files:
    day = 1
else:
    max_day = max(day_files)
    if max_day >= 25:
        year += 1
        day = 1
        year_dir = base_dir / str(year)
        year_dir.mkdir(parents=True, exist_ok=True)
    else:
        day = max_day + 1

template = f'''#!/usr/bin/env -S uv run
# /// script
# dependencies = []
# ///
 
# https://adventofcode.com/{year}/day/{day}

def part1(input: str):
    return 0

def part2(input: str):
    return 0

if __name__ == "__main__":
    from pathlib import Path
    from time import perf_counter_ns

    input_file = Path(__file__).parent / "../../../inputs/{year}/{day}.txt"
    input = input_file.read_text().rstrip()

    print("\\n")
    part1_start = perf_counter_ns()
    res = part1(input)
    part1_elapsed = (perf_counter_ns() - part1_start)
    print(f"part1 result: {{res}}")
    print(f"part1 run in {{part1_elapsed}}ns")

    print("\\n")
    part2_start = perf_counter_ns()
    res = part2(input)
    part2_elapsed = (perf_counter_ns() - part2_start)
    print(f"part2 result: {{res}}")
    print(f"part2 run in {{part2_elapsed}}ns")
'''

target_file = year_dir / f"day{day}.py"
target_file.write_text(template, encoding="utf-8")
print(f"Created: {target_file}")