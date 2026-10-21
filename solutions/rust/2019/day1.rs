#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2024"
[profile.dev]
opt-level = 3
---
/// https://adventofcode.com/2019/day/1

#[allow(unused)]
fn part1(input: &str) {
    let n = input
        .lines()
        .map(|line| {
            let num = line.parse::<f32>().unwrap();
            let f: f32 = num / 3_f32;
            let g: i32 = f.floor() as i32;
            return g - 2;
        })
        .sum::<i32>();
    println!("{}", n)
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}

fn main() {
    let input = include_str!("../../../inputs/2019/1.txt");
    part1(&input);
    part2(&input);
}
