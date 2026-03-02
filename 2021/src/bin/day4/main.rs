use regex::Match;

/// Advent of Code 2021 - Day 4
/// https://adventofcode.com/2021/day/4
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Cell {
    value: usize,
    marked: bool,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Board {
    grid: Vec<Vec<Cell>>,
    winner: bool,
}

impl Board {
    fn row(&self, y: usize) -> Vec<Cell> {
        self.grid[y].clone()
    }

    fn col(&self, x: usize) -> Vec<Cell> {
        self.grid
            .iter()
            .map(|row| row[x])
            .collect::<Vec<Cell>>()
            .clone()
    }

    fn get_unmarked(&self) -> Vec<Cell> {
        self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|c| !c.marked)
            .cloned()
            .collect::<Vec<Cell>>()
    }

    fn mark(&mut self, n: usize) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value == n {
                    cell.marked = true;
                }
            }
        }
    }

    fn has_winner(&self) -> Option<Vec<Cell>> {
        for n in 0..5 {
            let row = self.row(n);
            if row.iter().all(|c| c.marked) {
                return Some(row);
            }
            let col = self.col(n);
            if col.iter().all(|c| c.marked) {
                return Some(col);
            }
        }
        None
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut lines = input.lines();
    let seq_str = lines.next().unwrap();
    let mut board_idx: isize = -1;
    let mut y: usize = 0;
    let mut boards: Vec<Board> = vec![];
    let re = regex::Regex::new(r"(\s?\d+)").unwrap();
    for line in lines {
        if line == "" {
            y = 0;
            board_idx += 1;
            boards.push(Board {
                grid: vec![
                    vec![
                        Cell {
                            value: 0,
                            marked: false
                        };
                        5
                    ];
                    5
                ],
                winner: false,
            });
            continue;
        }
        let mut board = &mut boards[board_idx as usize];
        let row: Vec<Match> = re.find_iter(line).collect();
        for (i, s) in row.iter().enumerate() {
            let mut n = s.as_str().trim().parse::<usize>().unwrap();
            board.grid[y][i].value = n;
        }
        y += 1;
    }

    let seq = seq_str
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for num in seq {
        for board in boards.iter_mut() {
            board.mark(num);
            if let Some(_) = board.has_winner() {
                let sum: usize = board.get_unmarked().iter().map(|c| c.value).sum();
                println!("{:?}", sum * num);
                return;
            } else {
                continue;
            }
        }
    }
}

#[allow(unused)]
fn part2(input: &str) {
    let mut lines = input.lines();
    let seq_str = lines.next().unwrap();
    let mut board_idx: isize = -1;
    let mut y: usize = 0;
    let mut boards: Vec<Board> = vec![];
    let re = regex::Regex::new(r"(\s?\d+)").unwrap();
    for line in lines {
        if line == "" {
            y = 0;
            board_idx += 1;
            boards.push(Board {
                grid: vec![
                    vec![
                        Cell {
                            value: 0,
                            marked: false
                        };
                        5
                    ];
                    5
                ],
                winner: false,
            });
            continue;
        }
        let mut board = &mut boards[board_idx as usize];
        let row: Vec<Match> = re.find_iter(line).collect();
        for (i, s) in row.iter().enumerate() {
            let mut n = s.as_str().trim().parse::<usize>().unwrap();
            board.grid[y][i].value = n;
        }
        y += 1;
    }

    let seq = seq_str
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for num in seq {
        let mut boards_clone = boards.clone();
        for board in boards.iter_mut() {
            if board.winner {
                continue;
            }
            board.mark(num);
            if let Some(_) = board.has_winner() {
                board.winner = true;
                let sum: usize = board.get_unmarked().iter().map(|c| c.value).sum();
                println!("{:?}", sum * num);
            } else {
                continue;
            }
        }
    }
}
