use std::collections::VecDeque;

/// Advent of Code 2022 - Day 20
/// https://adventofcode.com/2022/day/20
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

fn mixing(initial_nums_with_indices: VecDeque<(i64, usize)>) -> VecDeque<(i64, usize)> {
    let mut current_state = initial_nums_with_indices.clone();

    for (original_val, original_idx) in initial_nums_with_indices.into_iter() {
        // Find the current position of the number (original_val, original_idx)
        let (current_pos, _) = current_state
            .iter()
            .enumerate()
            .find(|(_, (val, idx))| *val == original_val && *idx == original_idx)
            .unwrap();

        let moved_num_data = current_state.remove(current_pos).unwrap();
        let move_value = moved_num_data.0;
        let current_len_after_removal = current_state.len() as i64;

        if move_value == 0 {
            current_state.insert(current_pos, moved_num_data);
            continue;
        }

        let mut new_pos = (current_pos as i64 + move_value) % current_len_after_removal;

        // Ensure new_pos is non-negative
        if new_pos < 0 {
            new_pos += current_len_after_removal;
        }

        current_state.insert(new_pos as usize, moved_num_data);
    }
    current_state
}

#[allow(unused)]
fn part1(input: &str) {
    let mut nums_with_indices: VecDeque<(i64, usize)> = VecDeque::new();
    for (i, line) in input.lines().enumerate() {
        nums_with_indices.push_back((line.parse::<i64>().unwrap(), i));
    }

    let num_elements_total = nums_with_indices.len();
    let mixed_with_indices = mixing(nums_with_indices.clone());

    let (zero_index_in_mixed, _) = mixed_with_indices
        .iter()
        .enumerate()
        .find(|(_, (n_val, _))| *n_val == 0)
        .unwrap();

    let i1k = (zero_index_in_mixed + 1000) % num_elements_total;
    let i2k = (zero_index_in_mixed + 2000) % num_elements_total;
    let i3k = (zero_index_in_mixed + 3000) % num_elements_total;

    let sum = mixed_with_indices[i1k].0 + mixed_with_indices[i2k].0 + mixed_with_indices[i3k].0;
    println!("{}", sum);
}

#[allow(unused)]
fn part2(input: &str) {
    const ENC_KEY: i64 = 811589153;

    let mut nums_with_indices: VecDeque<(i64, usize)> = VecDeque::new();
    for (i, line) in input.lines().enumerate() {
        nums_with_indices.push_back((line.parse::<i64>().unwrap(), i));
    }

    let num_elements_total = nums_with_indices.len();
    let mixed_with_indices = mixing(nums_with_indices.clone());

    let (zero_index_in_mixed, _) = mixed_with_indices
        .iter()
        .enumerate()
        .find(|(_, (n_val, _))| *n_val == 0)
        .unwrap();

    let i1k = (zero_index_in_mixed + 1000) % num_elements_total;
    let i2k = (zero_index_in_mixed + 2000) % num_elements_total;
    let i3k = (zero_index_in_mixed + 3000) % num_elements_total;

    let sum = mixed_with_indices[i1k].0 + mixed_with_indices[i2k].0 + mixed_with_indices[i3k].0;
    println!("{}", sum);
}
