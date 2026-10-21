/// Advent of Code 2022 - Day 1
/// https://adventofcode.com/2022/day/1
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut index = 0;
    let mut nums: Vec<i32> = vec![];
    for line in lines {
        if line == "" {
            index += 1;
            continue;
        }
        let num = line.parse::<i32>().unwrap();

        if nums.len() <= index {
            nums.push(num);
        } else {
            nums[index] += num;
        }
    }
    // biggest in nums
    let largest = nums
        .iter()
        .reduce(|a, b| if a > b { a } else { b })
        .unwrap();
    println!("{}", largest);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let mut index = 0;
    let mut nums: Vec<i32> = vec![];
    for line in lines {
        if line == "" {
            index += 1;
            continue;
        }
        let num = line.parse::<i32>().unwrap();

        if nums.len() <= index {
            nums.push(num);
        } else {
            nums[index] += num;
        }
    }
    // biggest in nums
    nums.sort();
    nums.reverse();
    let top3: Vec<i32> = nums.iter().take(3).copied().collect();
    let sum = top3.iter().sum::<i32>();
    println!("{}", sum);
}
