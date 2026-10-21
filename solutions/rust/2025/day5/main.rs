use std::cmp;

/// Advent of Code 2025 - Day 5
/// https://adventofcode.com/2025/day/5
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let (ranges, inventory) = input.split_once("\n\n").unwrap();
    let mut fresh_ranges = Vec::<(u64, u64)>::new();
    for line in ranges.lines() {
        let range = line
            .split_once("-")
            .map(|(f, t)| (f.parse::<u64>().unwrap(), t.parse::<u64>().unwrap()))
            .unwrap();
        fresh_ranges.push(range);
    }
    let mut sum = 0;
    for id in inventory.lines().map(|l| l.parse::<u64>().unwrap()) {
        let is_fresh = fresh_ranges
            .iter()
            .any(|(from, to)| id >= *from && id <= *to);
        if is_fresh {
            sum += 1;
        }
    }
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut intervals = Vec::<(u64, u64)>::new();
    for line in ranges.lines() {
        let (left, right) = line
            .split_once("-")
            .map(|(f, t)| (f.parse::<u64>().unwrap(), t.parse::<u64>().unwrap()))
            .unwrap();
        intervals.push((left, right));
    }
    intervals.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut merged = Vec::<(u64, u64)>::new();
    merged.push(intervals[0]);
    for curr in intervals.iter().skip(1) {
        let last_merged = merged.last_mut().unwrap();
        let curr_start = curr.0;
        let curr_end = curr.1;
        let last_merged_end = last_merged.1;
        if curr_start <= last_merged_end.checked_add(1).unwrap_or(u64::MAX) {
            last_merged.1 = cmp::max(last_merged_end, curr_end);
        } else {
            merged.push(*curr);
        }
    }

    let mut sum = 0;
    for interval in merged {
        sum += interval.1 - interval.0 + 1;
    }
    println!("{}", sum);
}
