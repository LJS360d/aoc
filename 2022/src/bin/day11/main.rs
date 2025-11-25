use std::{
    collections::{BTreeMap, VecDeque},
    str::FromStr,
};

use regex::Regex;

/// Advent of Code 2022 - Day 11
/// https://adventofcode.com/2022/day/11
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: i32,
    items: VecDeque<i32>,

    operator: char,
    operand: String,

    test_divisor: i32,
    false_throw_to: i32,
    true_throw_to: i32,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_re = Regex::new(r"Monkey (\d+):\n").unwrap();
        let (_, [id_str]) = id_re.captures(s).unwrap().extract();
        let id = id_str.parse::<i32>().unwrap();

        let mut items = VecDeque::<i32>::new();
        let items_re = Regex::new(r"Starting items: (.+)\n").unwrap();
        if let Some(captures) = items_re.captures(s) {
            let (_, [items_str]) = captures.extract();
            for item in items_str.split(", ") {
                items.push_back(item.parse::<i32>().unwrap());
            }
        }
        let mut operator: Option<char> = None;
        let mut operand: Option<String> = None;

        let operation_re = Regex::new(r"Operation: new = old (.+)\n").unwrap();
        if let Some(captures) = operation_re.captures(s) {
            let (_, [operation_str]) = captures.extract();
            let (operator_str, operand_str) = operation_str.split_once(" ").unwrap();
            operator = Some(operator_str.chars().next().unwrap());
            operand = Some(operand_str.to_string());
        }

        let mut test_divisor = 0;
        let test_re = Regex::new(r"Test: divisible by (\d+)\n").unwrap();
        if let Some(captures) = test_re.captures(s) {
            let (_, [test_divisor_str]) = captures.extract();
            test_divisor = test_divisor_str.parse::<i32>().unwrap();
        }

        let mut false_throw_to = 0;
        let mut true_throw_to = 0;

        let true_throw_re = Regex::new(r"If true: throw to monkey (\d+)\n").unwrap();
        if let Some(captures) = true_throw_re.captures(s) {
            let (_, [true_throw_to_str]) = captures.extract();
            true_throw_to = true_throw_to_str.parse::<i32>().unwrap();
        }
        let false_throw_re = Regex::new(r"If false: throw to monkey (\d+)\n").unwrap();
        if let Some(captures) = false_throw_re.captures(s) {
            let (_, [false_throw_to_str]) = captures.extract();
            false_throw_to = false_throw_to_str.parse::<i32>().unwrap();
        }

        Ok(Monkey {
            id,
            items,
            operator: operator.unwrap(),
            operand: operand.unwrap(),
            test_divisor,
            false_throw_to,
            true_throw_to,
        })
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut all_monkeys: BTreeMap<i32, Monkey> = BTreeMap::new();
    for parsable_monkey in input.trim().split("\n\n") {
        let monkey = Monkey::from_str(parsable_monkey).unwrap();
        all_monkeys.insert(monkey.id, monkey);
    }
    println!("{:?}", all_monkeys);

    for _ in 0..20 {
        // play a round
        let monkey_ids: Vec<i32> = all_monkeys.keys().copied().collect();

        for monkey_id in monkey_ids {
            // Process all items for this monkey
            let mut throws: Vec<(i32, i32)> = Vec::new(); // (target_monkey, item)

            {
                let monkey = all_monkeys.get_mut(&monkey_id).unwrap();

                while let Some(item) = monkey.items.pop_front() {
                    let operand = if monkey.operand == "old" {
                        item
                    } else {
                        monkey.operand.parse::<i32>().unwrap()
                    };
                    let inspection_res = match monkey.operator {
                        '+' => item + operand,
                        '-' => item - operand,
                        '*' => item * operand,
                        _ => panic!("Invalid operator"),
                    };
                    let worry_level = inspection_res / 3;

                    // perform monkey test
                    let throw_to = if worry_level % monkey.test_divisor == 0 {
                        monkey.true_throw_to
                    } else {
                        monkey.false_throw_to
                    };

                    throws.push((throw_to, worry_level));
                }
            } // Release borrow of monkey here

            for (target_monkey, item) in throws {
                all_monkeys
                    .get_mut(&target_monkey)
                    .unwrap()
                    .items
                    .push_back(item);
            }
        }
    }

    println!("{:?}", all_monkeys);
}
#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
