use regex::Regex;
use std::collections::{HashMap, HashSet};

/// Advent of Code 2022 - Day 21
/// https://adventofcode.com/2022/day/21
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let re_num = Regex::new(r"^(\w+):\s(\d+)$").unwrap();
    let re_op = Regex::new(r"^(\w+):\s(\w+)\s*(.)\s*(\w+)$").unwrap();
    let mut nums = HashMap::<&str, i64>::new();
    let mut ops: HashSet<(&str, &str, &str, &str)> = HashSet::new();
    for line in lines {
        if re_num.is_match(line) {
            let (_, [monkey, num_str]) = re_num.captures(line).unwrap().extract();
            let num = num_str.parse::<i64>().unwrap();
            nums.insert(monkey, num);
            continue;
        }
        let (_, [monkey, var_1, op, var_2]) = re_op.captures(line).unwrap().extract();
        if nums.contains_key(var_1) && nums.contains_key(var_2) {
            // it can be resolved immediatly
            let n1 = nums.get(var_1).unwrap();
            let n2 = nums.get(var_2).unwrap();
            let res = process_op(*n1, op, *n2);
            nums.insert(monkey, res);
        }
        ops.insert((monkey, var_1, op, var_2));
    }

    while !nums.contains_key("root") {
        for operation in ops.clone().iter() {
            let &(monkey, var_1, op, var_2) = operation;
            if nums.contains_key(var_1) && nums.contains_key(var_2) {
                // it can be resolved immediatly
                let n1 = nums.get(var_1).unwrap();
                let n2 = nums.get(var_2).unwrap();
                let res = process_op(*n1, op, *n2);
                nums.insert(monkey, res);
                ops.remove(operation);
            }
        }
    }

    print!("{}", nums.get("root").unwrap())
}

fn process_op(v1: i64, op: &str, v2: i64) -> i64 {
    match op {
        "-" => v1 - v2,
        "+" => v1 + v2,
        "*" => v1 * v2,
        "/" => v1 / v2,
        _ => unreachable!(),
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let re_num = Regex::new(r"^(\w+):\s(\d+)$").unwrap();
    let re_op = Regex::new(r"^(\w+):\s(\w+)\s*(.)\s*(\w+)$").unwrap();
    let mut nums = HashMap::<&str, i64>::new();
    let mut ops: HashSet<(&str, &str, &str, &str)> = HashSet::new();
    for line in lines {
        if re_num.is_match(line) {
            let (_, [monkey, num_str]) = re_num.captures(line).unwrap().extract();
            let num = num_str.parse::<i64>().unwrap();
            if monkey == "humn" {
                continue;
            }
            nums.insert(monkey, num);
            continue;
        }
        let (_, [monkey, var_1, op, var_2]) = re_op.captures(line).unwrap().extract();
        if nums.contains_key(var_1) && nums.contains_key(var_2) {
            // it can be resolved immediatly
            let n1 = nums.get(var_1).unwrap();
            let n2 = nums.get(var_2).unwrap();
            let res = process_op(*n1, op, *n2);
            nums.insert(monkey, res);
        }
        ops.insert((monkey, var_1, op, var_2));
    }
    let mut humn: i64 = 0; // 3 887 609 741 189
    let mut inc: i64 = 1;
    loop {
        let mut nums_clone = nums.clone();
        nums_clone.insert("humn", humn);
        let mut ops_clone = ops.clone();
        while !nums_clone.contains_key("root") {
            for operation in ops_clone.clone().iter() {
                let &(monkey, var_1, op, var_2) = operation;
                if nums_clone.contains_key(var_1) && nums_clone.contains_key(var_2) {
                    // it can be resolved immediatly
                    let n1 = nums_clone.get(var_1).unwrap();
                    let n2 = nums_clone.get(var_2).unwrap();
                    if monkey == "root" {
                        if n1 == n2 {
                            println!("{humn}");
                            return;
                        }
                        inc = (n1.abs_diff(*n2) as i64) / 2;
                        if inc == 0 {
                            inc = 1;
                        }
                        println!("humn: {humn}, inc: {inc}, {n1} {n2} {}", n1 > n2)
                    }
                    let res = process_op(*n1, op, *n2);
                    nums_clone.insert(monkey, res);
                    ops_clone.remove(operation);
                }
            }
        }
        humn = humn + inc;
    }

    print!("{}", nums.get("root").unwrap())
}
