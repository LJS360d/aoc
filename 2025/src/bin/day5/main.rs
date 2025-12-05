use std::cmp::Ordering;

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

#[derive(Clone, Copy, Debug)]
enum Bound {
    Left(u64),
    Right(u64),
}

#[allow(unused)]
fn part2(input: &str) {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut min = u64::MAX;
    let mut max = 0_u64;
    let mut fresh_ids_bounds = Vec::<Bound>::new();
    for line in ranges.lines() {
        let (left, right) = line
            .split_once("-")
            .map(|(f, t)| (f.parse::<u64>().unwrap(), t.parse::<u64>().unwrap()))
            .unwrap();
        fresh_ids_bounds.push(Bound::Left(left));
        fresh_ids_bounds.push(Bound::Right(right));
        if right > max {
            max = right
        }
        if left < min {
            min = left
        }
    }
    fresh_ids_bounds.sort_unstable_by(|a_bnd, b_bnd| {
        let a = match a_bnd {
            Bound::Left(val) => val,
            Bound::Right(val) => val,
        };
        let b = match b_bnd {
            Bound::Left(val) => val,
            Bound::Right(val) => val,
        };

        if a > b {
            return Ordering::Greater;
        } else if a < b {
            return Ordering::Less;
        }
        return Ordering::Equal;
    });
    // identify holes
    let mut holes = Vec::<(u64, u64)>::new();
    for (i, curr) in fresh_ids_bounds.iter().enumerate() {
        if i == 0 || i == fresh_ids_bounds.len() - 1 {
            continue;
        }
        let prev = fresh_ids_bounds[i - 1];
        let next = fresh_ids_bounds[i + 1];

        if let Bound::Left(curr_value) = curr {
            if let Bound::Left(_) = next {
                // theres a hole that goes from "prev" to "curr"
                let Bound::Right(prev_value) = prev else {
                    unreachable!()
                };

                holes.push((prev_value, *curr_value));
            }
        }
    }

    let mut res = max - min;
    println!("{:?}", fresh_ids_bounds);
    println!("{:?}", holes);
    for hole in holes.iter() {
        res -= (hole.1 - hole.0)
    }

    println!("{:?}", res);
}
