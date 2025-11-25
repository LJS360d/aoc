/// Advent of Code 2022 - Day 10
/// https://adventofcode.com/2022/day/10
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut cycles = 0;
    let mut reg = 1;
    let mut sum = 0;

    fn add_cycle(cycles: &mut i32, reg: i32, sum: &mut i32) {
        *cycles += 1;
        if (*cycles + 20) % 40 == 0 {
            let signal_strength = reg * *cycles;
            *sum += signal_strength;
        }
    }

    for line in lines {
        if line == "noop" {
            add_cycle(&mut cycles, reg, &mut sum);
            continue;
        }

        let (_, arg) = line.split_once(" ").unwrap();
        let num = arg.parse::<i32>().unwrap();
        add_cycle(&mut cycles, reg, &mut sum);
        add_cycle(&mut cycles, reg, &mut sum);
        reg += num;
    }
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    let mut cycles = 0;
    let mut reg = 1;
    let mut output = Vec::<char>::new();

    fn add_cycle(cycles: &mut i32, reg: i32, output: &mut Vec<char>) {
        if (*cycles % 40).abs_diff(reg) <= 1 {
            output.push('#')
        } else {
            output.push('.')
        }
        *cycles += 1;


        if *cycles % 40 == 0 {
            output.push('\n');
        }
    }

    for line in lines {
        if line == "noop" {
            add_cycle(&mut cycles, reg, &mut output);
            continue;
        }

        let (_, arg) = line.split_once(" ").unwrap();
        let num = arg.parse::<i32>().unwrap();
        add_cycle(&mut cycles, reg, &mut output);
        add_cycle(&mut cycles, reg, &mut output);
        reg += num;

    }

    println!("{}", output.iter().collect::<String>());
}
