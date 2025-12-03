/// Advent of Code 2025 - Day 3
/// https://adventofcode.com/2025/day/3
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let mut batteries = line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let mut max_dec_index: usize = 0;
        let mut max_unit = 0;

        for (i, battery) in batteries[..batteries.len() - 1].iter().enumerate() {
            if *battery > batteries[max_dec_index] {
                max_dec_index = i;
            }
        }
        for battery in batteries[(max_dec_index + 1)..].iter() {
            if *battery > max_unit {
                max_unit = *battery;
            }
        }

        let joltage = batteries[max_dec_index] * 10 + max_unit;
        println!("{line} -> {joltage}");
        sum += joltage;
    }
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let mut sum: u128 = 0;
    for line in input.lines() {
        let mut batteries = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect::<Vec<i32>>();

        let mut start_index = 0;
        let mut joltage_vec: Vec<usize> = vec![];
        const BATTERIES_AMOUNT: usize = 12;
        for i in (1..=BATTERIES_AMOUNT).rev() {
            let skip_amount: usize = i;
            let mut max: i32 = -1;
            let max_index: usize = 0;
            let to = (batteries.len() + 1 - start_index) - (BATTERIES_AMOUNT - joltage_vec.len());
            let search_vec = batteries[start_index..(start_index + to)].to_vec();
            println!(
                "{line} , searching for max in {:?}, from: {start_index}, to: {to}",
                search_vec
            );
            let mut used_index = 0;
            for (ii, battery) in search_vec.into_iter().enumerate() {
                if max < battery {
                    max = battery;
                    used_index = ii;
                }
            }
            start_index += used_index;
            joltage_vec.push(max as usize);
            println!(
                "max ({max}) is at {start_index}, {} numbers available left, {} more numbers are necessary",
                batteries.len() + 1 - start_index,
                BATTERIES_AMOUNT - joltage_vec.len()
            );
            start_index += 1;
        }
        println!("{joltage_vec:?}");

        let joltage: u64 = joltage_vec
            .iter()
            .enumerate()
            .map(|(i, n)| {
                let mul: u64 = 10_u64.pow((joltage_vec.len() - i - 1) as u32);
                return (*n as u64) * mul;
            })
            .sum::<u64>();

        println!("{line} -> {joltage}");
        sum += joltage as u128;
    }
    println!("{}", sum);
}
