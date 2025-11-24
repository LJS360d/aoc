use std::collections::{HashSet, VecDeque};

/// Advent of Code 2022 - Day 6
/// https://adventofcode.com/2022/day/6
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let line = input.lines().next().unwrap();

    let mut buffer: VecDeque<char> = VecDeque::new();
    for (i, char) in line.chars().enumerate() {
        if buffer.len() < 4 {
            buffer.push_back(char);
            continue;
        }
        buffer.pop_front();
        buffer.push_back(char);
        let set: HashSet<&char> = buffer.iter().clone().collect();
        if set.len() != 4 {
            continue;
        } else {
            println!("{}", i + 1);
            break;
        }
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let line = input.lines().next().unwrap();

    let mut buffer: VecDeque<char> = VecDeque::new();
    for (i, char) in line.chars().enumerate() {
        if buffer.len() < 14 {
            buffer.push_back(char);
            continue;
        }
        buffer.pop_front();
        buffer.push_back(char);
        let set: HashSet<&char> = buffer.iter().clone().collect();
        if set.len() != 14 {
            continue;
        } else {
            println!("{}", i + 1);
            break;
        }
    }
}
