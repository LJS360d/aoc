use std::num;

use regex::Regex;

/// Advent of Code 2025 - Day 6
/// https://adventofcode.com/2025/day/6
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let mut nums: Vec<Vec<u64>> = vec![];
    let mut ops = Vec::<char>::new();
    let re = Regex::new(r"(?x)(\d+)").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    for (i, line) in lines.iter().take(lines.len() - 1).enumerate() {
        nums.push(vec![]);
        for cap in re.find_iter(line) {
            let num = cap.as_str().parse::<u64>().unwrap();
            nums[i].push(num);
        }
    }
    let re_ops = Regex::new(r"(?x)([\+|\*])").unwrap();
    let last_line = lines.last().unwrap();
    for cap in re_ops.find_iter(last_line) {
        let op = cap.as_str().parse::<char>().unwrap();
        ops.push(op);
    }

    let mut sum = 0;
    for (i, operation) in ops.iter().enumerate() {
        let op1 = nums[0].get(i).unwrap();
        let op2 = nums[1].get(i).unwrap();
        let op3 = nums[2].get(i).unwrap();
        let op4 = nums[3].get(i).unwrap();
        match operation {
            '+' => {
                let res = (op1 + op2 + op3 + op4);
                sum += res;
            }
            '*' => {
                let res = (op1 * op2 * op3 * op4);
                sum += res;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let mut ops = Vec::<char>::new();
    let mut last_line = input.lines().last().unwrap().to_string();
    let mut num_lengths = vec![];
    let mut curr_num_len = 0;
    last_line.push(' ');
    last_line.push('\n');
    for ch in last_line.chars() {
        match ch {
            ' ' => {
                curr_num_len += 1;
            }
            ch => {
                if ch != '\n' {
                    ops.push(ch);
                }
                if curr_num_len != 0 {
                    num_lengths.push(curr_num_len.clone());
                    curr_num_len = 0;
                }
            }
        }
    }
    let mut num_builds: Vec<Vec<String>> = vec![];
    for n in num_lengths.clone() {
        let mut inner = vec![];
        for _ in 0..n {
            inner.push(String::new());
        }
        num_builds.push(inner);
    }
    // 3 for test input, 4 for real input
    let sig_lines: u32 = 4;
    for (line_idx, line) in input.lines().enumerate().take(sig_lines as usize) {
        let mut chars = line.chars().rev();
        for (num_idx, &num_len) in num_lengths.iter().enumerate() {
            for i in (0..num_len).rev() {
                let ch = chars.next_back().unwrap();
                match ch {
                    ' ' => {}
                    ch => {
                        num_builds[num_idx][i as usize].push(ch);
                    }
                }
            }
            let _ = chars.next_back();
        }
    }

    let mut sum: u64 = 0;
    for (i, op) in ops.iter().enumerate() {
        let operands: Vec<u64> = num_builds[i]
            .iter()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        match op {
            '+' => sum += operands.iter().fold(0, |acc, x| acc + x),
            '*' => {
                sum += operands.iter().fold(1, |acc, x| acc * x);
            }
            _ => {}
        }
    }

    println!("{}", sum);
}
