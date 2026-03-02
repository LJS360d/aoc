use std::collections::{HashMap, HashSet};

/// Advent of Code 2021 - Day 3
/// https://adventofcode.com/2021/day/3
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let bins: Vec<&str> = input.lines().collect();
    let width = bins[0].len();
    let total = bins.len();

    let gamma_str: String = (0..width)
        .map(|i| {
            let ones = bins.iter().filter(|s| s.as_bytes()[i] == b'1').count();
            if ones > total / 2 { '1' } else { '0' }
        })
        .collect();

    let gamma = i32::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = !gamma & ((1 << width) - 1);

    println!("{}", gamma * epsilon);
}

#[allow(unused)]
fn part2(input: &str) {
    let bins: Vec<&str> = input.lines().map(|s| s).collect();
    let bins_set: HashSet<&str> = bins.iter().cloned().collect();
    let width = bins[0].len();
    let mut counts: HashMap<usize, i32> = HashMap::new();
    for i in 0..width {
        for bin in bins.clone() {
            let ch = bin.chars().nth(i).unwrap();
            if ch == '1' {
                let entry = counts.entry(i).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut oxygen_bins_set = bins_set.clone();
    println!("{:?}", oxygen_bins_set);
    for bin in bins_set {
        if oxygen_bins_set.len() == 1 {
            break;
        }
        for i in 0..width {
            let count = counts.get(&i).unwrap();
            let ch = if *count >= (bins.len() as i32 / 2) {
                '1'
            } else {
                '0'
            };
            if bin.chars().nth(i).unwrap() != ch {
                oxygen_bins_set.remove(bin);
                break;
            }
        }
    }
    println!("{:?}", oxygen_bins_set);
}
