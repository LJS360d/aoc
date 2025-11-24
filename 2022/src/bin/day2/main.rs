/// Advent of Code 2022 - Day 2
/// https://adventofcode.com/2022/day/2
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();
    let mut rounds: Vec<[char; 2]> = vec![];

    for line in lines {
        let mut round = [line.chars().next().unwrap(), line.chars().nth(2).unwrap()];
        rounds.push(round);
    }

    let mut score = 0;
    // A -> X(1) Rock
    // B -> Y(2) Paper
    // C -> Z(3) Scissors
    for round in &rounds {
        match round {
            ['A', 'X'] => score += (3 + 1), // draw and 1
            ['A', 'Y'] => score += (6 + 2), // win and 2
            ['A', 'Z'] => score += (0 + 3), // loss and 3
            ['B', 'X'] => score += (0 + 1), // loss and 1
            ['B', 'Y'] => score += (3 + 2), // draw and 2
            ['B', 'Z'] => score += (6 + 3), // win and 3
            ['C', 'X'] => score += (6 + 1), // win and 1
            ['C', 'Y'] => score += (0 + 2), // loss and 2
            ['C', 'Z'] => score += (3 + 3), // draw and 3
            _ => {}
        }
    }
    println!("{:?}", score);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();
    let mut rounds: Vec<[char; 2]> = vec![];

    for line in lines {
        let mut round = [line.chars().next().unwrap(), line.chars().nth(2).unwrap()];
        rounds.push(round);
    }

    let mut score = 0;
    // X lose
    // Y draw
    // Z win
    for round in &rounds {
        match round {
            // #### Outcome | Hand Played
            ['A', 'X'] => score += (0 + 3), // they rock, need loss     -> scissors (0+3)
            ['A', 'Y'] => score += (3 + 1), // they rock, need draw     -> rock (3+1)
            ['A', 'Z'] => score += (6 + 2), // they rock, need win      -> paper (6+2)

            ['B', 'X'] => score += (0 + 1), // they paper, need loss    -> rock (0+1)
            ['B', 'Y'] => score += (3 + 2), // they paper, need draw    -> paper (3+2)
            ['B', 'Z'] => score += (6 + 3), // they paper, need win     -> scissors (6+3)

            ['C', 'X'] => score += (0 + 2), // they scissors, need loss -> paper (0+2)
            ['C', 'Y'] => score += (3 + 3), // they scissors, need draw -> scissors (3+3)
            ['C', 'Z'] => score += (6 + 1), // they scissors, need win  -> rock (6+1)
            _ => {}
        }
    }
    println!("{:?}", score);
}
