/// Advent of Code 2022 - Day 4
/// https://adventofcode.com/2022/day/4
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut count = 0;
    for (i, line) in lines.enumerate() {
        let (e1, e2) = line.split_once(',').unwrap();
        let (e1s1_str, e1s2_str) = e1.split_once('-').unwrap();
        let (e2s1_str, e2s2_str) = e2.split_once('-').unwrap();

        let e1s1 = e1s1_str.parse::<u32>().unwrap();
        let e1s2 = e1s2_str.parse::<u32>().unwrap();
        let e2s1 = e2s1_str.parse::<u32>().unwrap();
        let e2s2 = e2s2_str.parse::<u32>().unwrap();

        if e1s1 <= e2s1 && e1s2 >= e2s2 {
            // range 1 fully contains range 2
            count += 1;
        } else if e2s1 <= e1s1 && e2s2 >= e1s2 {
            // range 2 fully contains range 1
            count += 1;
        }
    }
    println!("{}", count)
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    let mut count = 0;
    for (i, line) in lines.enumerate() {
        let (e1, e2) = line.split_once(',').unwrap();
        let (e1s1_str, e1s2_str) = e1.split_once('-').unwrap();
        let (e2s1_str, e2s2_str) = e2.split_once('-').unwrap();

        // +1 for end because ex: 2..5 -> [2,3,4]
        let r1 = (e1s1_str.parse::<u32>().unwrap()..e1s2_str.parse::<u32>().unwrap() + 1);
        let r2 = (e2s1_str.parse::<u32>().unwrap()..e2s2_str.parse::<u32>().unwrap() + 1);

        for num in r1.into_iter() {
            if r2.contains(&num) {
                count += 1;
                break;
            }
        }
    }

    println!("{}", count)
}
