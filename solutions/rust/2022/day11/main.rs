use std::{
    collections::{BTreeMap, VecDeque},
    str::FromStr,
};

use regex::Regex;

/// Advent of Code 2022 - Day 11
/// https://adventofcode.com/2022/day/11
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: u64,
    items: VecDeque<u64>,

    operator: char,
    operand: String,

    test_divisor: u64,
    false_throw_to: u64,
    true_throw_to: u64,
    inspections: u64,
}

impl FromStr for Monkey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_re = Regex::new(r"Monkey (\d+):\n").unwrap();
        let (_, [id_str]) = id_re.captures(s).unwrap().extract();
        let id = id_str.parse::<u64>().unwrap();

        let mut items = VecDeque::<u64>::new();
        let items_re = Regex::new(r"Starting items: (.+)\n").unwrap();
        if let Some(captures) = items_re.captures(s) {
            let (_, [items_str]) = captures.extract();
            for item in items_str.split(", ") {
                items.push_back(item.parse::<u64>().unwrap());
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
            test_divisor = test_divisor_str.parse::<u64>().unwrap();
        }

        let mut false_throw_to = 0;
        let mut true_throw_to = 0;

        let true_throw_re = Regex::new(r"If true: throw to monkey (\d+)\n").unwrap();
        if let Some(captures) = true_throw_re.captures(s) {
            let (_, [true_throw_to_str]) = captures.extract();
            true_throw_to = true_throw_to_str.parse::<u64>().unwrap();
        }
        let false_throw_re = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
        if let Some(captures) = false_throw_re.captures(s) {
            let (_, [false_throw_to_str]) = captures.extract();
            false_throw_to = false_throw_to_str.parse::<u64>().unwrap();
        }

        Ok(Monkey {
            id,
            items,
            operator: operator.unwrap(),
            operand: operand.unwrap(),
            test_divisor,
            false_throw_to,
            true_throw_to,
            inspections: 0,
        })
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut all_monkeys: BTreeMap<u64, Monkey> = BTreeMap::new();
    for parsable_monkey in input.trim().split("\n\n") {
        let monkey = Monkey::from_str(parsable_monkey).unwrap();
        all_monkeys.insert(monkey.id, monkey);
    }
    println!("Starting state");
    all_monkeys.values().into_iter().for_each(|monkey| {
        println!("Monkey {}: {:?}", monkey.id, monkey.items);
    });
    for round in 1..21 {
        // play a round
        let monkey_ids: Vec<u64> = all_monkeys.keys().copied().collect();

        for monkey_id in monkey_ids {
            let mut throws: Vec<(u64, u64)> = Vec::new();
            println!("Monkey {monkey_id}:");
            {
                let monkey = all_monkeys.get_mut(&monkey_id).unwrap();

                while let Some(item) = monkey.items.pop_front() {
                    println!("\tMonkey inspects an item with a worry level of {item}.");

                    let operand = if monkey.operand == "old" {
                        item
                    } else {
                        monkey.operand.parse::<u64>().unwrap()
                    };
                    let inspection_res: u64 = match monkey.operator {
                        '+' => {
                            let res = item + operand;
                            println!("\t\tWorry level increases by {operand} to {res}");
                            res
                        }
                        '-' => {
                            let res = item - operand;
                            println!("\t\tWorry level decreases by {operand} to {res}");
                            res
                        }
                        '*' => {
                            let res = item * operand;
                            println!("\t\tWorry level is multiplied by by {operand} to {res}");
                            res
                        }
                        _ => panic!("Invalid operator"),
                    };
                    let worry_level = inspection_res / 3;
                    println!(
                        "\t\tMonkey gets bored with item. Worry level is divided by 3 to {worry_level}."
                    );
                    monkey.inspections += 1;

                    let throw_to = if (worry_level % monkey.test_divisor == 0) {
                        println!(
                            "\t\tCurrent Current worry level is divisible by {}",
                            monkey.test_divisor
                        );
                        monkey.true_throw_to
                    } else {
                        println!(
                            "\t\tCurrent Current worry level is not divisible by {}",
                            monkey.test_divisor
                        );
                        monkey.false_throw_to
                    };
                    println!(
                        "\t\tItem with worry level {worry_level} is thrown to monkey {throw_to}"
                    );
                    throws.push((throw_to, worry_level));
                }
            }

            for (target_monkey, wl) in throws {
                all_monkeys
                    .get_mut(&target_monkey)
                    .unwrap()
                    .items
                    .push_back(wl);
            }
        }
        println!("State after round {round}");
        all_monkeys.values().into_iter().for_each(|monkey| {
            println!("Monkey {}: {:?}", monkey.id, monkey.items);
        });
    }

    let mut sorted_inspections = all_monkeys
        .values()
        .map(|m| m.inspections)
        .collect::<Vec<u64>>();
    sorted_inspections.sort();
    sorted_inspections.reverse();

    println!(
        "{:?}, {}",
        sorted_inspections,
        sorted_inspections.iter().take(2).fold(1, |acc, x| acc * x)
    );
}
#[allow(unused)]
fn part2(input: &str) {
    let mut all_monkeys: BTreeMap<u64, Monkey> = BTreeMap::new();
    for parsable_monkey in input.trim().split("\n\n") {
        let monkey = Monkey::from_str(parsable_monkey).unwrap();
        all_monkeys.insert(monkey.id, monkey);
    }

    // CRT - Sunzi's Theorem
    let super_mod: u64 = all_monkeys.values().map(|m| m.test_divisor).product();

    for round in 1..10001 {
        // play a round
        let monkey_ids: Vec<u64> = all_monkeys.keys().copied().collect();

        for monkey_id in monkey_ids {
            let mut throws: Vec<(u64, u64)> = Vec::new();
            {
                let monkey = all_monkeys.get_mut(&monkey_id).unwrap();

                while let Some(item) = monkey.items.pop_front() {
                    let operand = if monkey.operand == "old" {
                        item
                    } else {
                        monkey.operand.parse::<u64>().unwrap()
                    };
                    let inspection_res: u64 = match monkey.operator {
                        '+' => {
                            let res = item + operand;
                            res
                        }
                        '-' => {
                            let res = item - operand;
                            res
                        }
                        '*' => {
                            let res = item * operand;
                            res
                        }
                        _ => panic!("Invalid operator"),
                    };
                    let worry_level = inspection_res % super_mod;
                    monkey.inspections += 1;

                    let throw_to = if (worry_level % monkey.test_divisor == 0) {
                        monkey.true_throw_to
                    } else {
                        monkey.false_throw_to
                    };
                    throws.push((throw_to, worry_level));
                }
            }

            for (target_monkey, wl) in throws {
                all_monkeys
                    .get_mut(&target_monkey)
                    .unwrap()
                    .items
                    .push_back(wl);
            }
        }
    }

    let mut sorted_inspections = all_monkeys
        .values()
        .map(|m| m.inspections)
        .collect::<Vec<u64>>();
    sorted_inspections.sort();
    sorted_inspections.reverse();

    println!(
        "{:?}, {}",
        sorted_inspections,
        sorted_inspections.iter().take(2).fold(1, |acc, x| acc * x)
    );
}
