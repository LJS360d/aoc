use std::collections::VecDeque;

/// Advent of Code 2022 - Day 20
/// https://adventofcode.com/2022/day/20
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

fn mixing(nums: VecDeque<i32>) -> VecDeque<i32> {
    let mut result = nums.clone();
    let wrap = nums.len() as i32 - 1;
    for num in nums.into_iter() {
        if num == 0 {
            continue;
        }
        let i = result.iter().position(|n| *n == num).unwrap();
        let mut new_index = (i as i32 + num) % wrap;
        if new_index < 0 {
            new_index += wrap;
        }
        if new_index == 0 {
            new_index = wrap;
        }
        let new_index = new_index as usize;
        result.remove(i);
        result.insert(new_index, num);
        // println!("{:?}", result);
    }
    result
}

#[allow(unused)]
fn part1(input: &str) {
    let mut nums: VecDeque<i32> = VecDeque::new();
    for line in input.lines() {
        nums.push_back(line.parse::<i32>().unwrap());
    }
    let wrap = nums.len();
    // println!("{:?}", nums);
    let mixed = mixing(nums.clone());
    let (zero_index, _) = mixed.iter().enumerate().find(|(i, n)| **n == 0).unwrap();
    let i1k = (zero_index + 1000) % wrap;
    let i2k = (zero_index + 2000) % wrap;
    let i3k = (zero_index + 3000) % wrap;
    let sum = mixed[i1k] + mixed[i2k] + mixed[i3k];
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
