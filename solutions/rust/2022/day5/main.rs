use regex::Regex;
use std::collections::VecDeque;

/// Advent of Code 2022 - Day 5
/// https://adventofcode.com/2022/day/5
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let mut state: [VecDeque<char>; 9] = [
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
    ];
    for (i, line) in input.lines().take(8).enumerate() {
        for (ii, char) in line.chars().enumerate() {
            if ii % 4 == 1 && char != ' ' {
                let state_i = (ii).div_ceil(4) - 1;
                state[state_i].push_front(char);
            }
        }
    }
    let re = Regex::new(r"move ([0-9]+) from ([1-9]) to ([1-9])").unwrap();
    for line in input.lines().skip(10) {
        if let Some(captures) = re.captures(line) {
            let (_, [amount_str, from_str, to_str]) = captures.extract();
            let amount = amount_str.parse::<u32>().unwrap();
            let from = from_str.parse::<usize>().unwrap() - 1;
            let to = to_str.parse::<usize>().unwrap() - 1;

            for _ in (0..amount) {
                match state[from].pop_back() {
                    Some(item) => {
                        state[to].push_back(item);
                    }
                    None => continue,
                };
            }
        }
    }

    let mut res: Vec<char> = vec![];
    for mut stack in state.into_iter() {
        res.push(stack.pop_back().unwrap())
    }
    println!("{}", res.into_iter().collect::<String>())
}

#[allow(unused)]
fn part2(input: &str) {
    let mut state: [VecDeque<char>; 9] = [
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
        VecDeque::from(vec![]),
    ];
    for (i, line) in input.lines().take(8).enumerate() {
        for (ii, char) in line.chars().enumerate() {
            if ii % 4 == 1 && char != ' ' {
                let state_i = (ii).div_ceil(4) - 1;
                state[state_i].push_front(char);
            }
        }
    }
    let re = Regex::new(r"move ([0-9]+) from ([1-9]) to ([1-9])").unwrap();
    for line in input.lines().skip(10) {
        if let Some(captures) = re.captures(line) {
            let (_, [amount_str, from_str, to_str]) = captures.extract();
            let amount = amount_str.parse::<usize>().unwrap();
            let from = from_str.parse::<usize>().unwrap() - 1;
            let to = to_str.parse::<usize>().unwrap() - 1;

            let mut sub: Vec<char> = vec![];
            for _ in (0..amount) {
                match state[from].pop_back() {
                    Some(item) => {
                        sub.push(item);
                    }
                    None => continue,
                };
            }
            sub.reverse();
            for item in sub.into_iter() {
                state[to].push_back(item);
            }
        }
    }
    println!("{:?}", state);

    let mut res: Vec<char> = vec![];
    for mut stack in state.into_iter() {
        res.push(stack.pop_back().unwrap())
    }
    println!("{}", res.into_iter().collect::<String>())
}
