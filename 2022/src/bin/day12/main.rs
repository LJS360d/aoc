use std::collections::{HashMap, VecDeque};

/// Advent of Code 2022 - Day 12
/// https://adventofcode.com/2022/day/12
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}
#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
type AdjacencyList = HashMap<String, Vec<(String, u8)>>;

impl Graph {
    fn build_adj_list(&self) -> AdjacencyList {
        let mut adj = HashMap::new();
        for node in &self.nodes {
            adj.insert(node.id.clone(), Vec::new());
        }
        // Populate the entries with edges
        for edge in &self.edges {
            if let Some(neighbors) = adj.get_mut(&edge.from) {
                neighbors.push((edge.to.clone(), edge.weight));
            }
        }
        adj
    }

    fn shortest_path(&self, start: String, end: String) -> Vec<String> {
        let adj_list = self.build_adj_list();

        // Deque for 0-1 BFS
        let mut deque: VecDeque<String> = VecDeque::new();

        // Distance map: Node ID -> Minimum distance from start
        let mut distance: HashMap<String, usize> = HashMap::new();

        // Predecessor map: Child -> Parent for path reconstruction
        let mut predecessors: HashMap<String, String> = HashMap::new();

        // Initialize all nodes to max distance, or 0 for the start node
        for node in &self.nodes {
            distance.insert(node.id.clone(), usize::MAX);
        }

        distance.insert(start.clone(), 0);
        deque.push_back(start.clone());

        // 0-1 BFS Traversal
        while let Some(current) = deque.pop_front() {
            if current == end {
                break; // Path found
            }

            let current_dist = *distance.get(&current).unwrap_or(&usize::MAX);

            if let Some(neighbors) = adj_list.get(&current) {
                for (neighbor, weight) in neighbors.clone() {
                    let weight = weight as usize;

                    let new_dist = current_dist + weight;
                    if new_dist < *distance.get(&neighbor.clone()).unwrap_or(&usize::MAX) {
                        distance.insert(neighbor.clone(), new_dist);
                        predecessors.insert(neighbor.clone(), current.clone());
                        deque.push_back(neighbor.clone());
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current_step = end.clone();

        if *distance.get(&end).unwrap_or(&usize::MAX) == usize::MAX {
            return Vec::new();
        }

        while let Some(prev) = predecessors.get(&current_step) {
            path.push(current_step);
            current_step = prev.clone();
            if current_step == start {
                path.push(start.clone());
                break;
            }
        }

        // Handle case where start == end
        if path.is_empty() && start == end {
            return vec![start];
        }

        path.reverse();
        path
    }

    fn visualize_path(&self, path: Vec<String>) {
        for (i, node) in self.nodes.iter().enumerate() {
            if path.contains(&node.id) {
                print!("{}", node.label);
            } else {
                print!(".");
            }

            if (i + 1) % 143 == 0 {
                println!();
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    label: char,
}

impl Node {
    fn new(id: String, height: char) -> Self {
        Node { id, label: height }
    }
}
#[derive(Debug, Clone)]
struct Edge {
    from: String,
    to: String,
    weight: u8,
}

fn height_of(c: char) -> u8 {
    match c {
        'E' => 25,
        'S' => 0,
        letter => letter as u8 - b'a',
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let mut lines = input.lines();
    let lines_vec: Vec<_> = lines.clone().collect();

    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let num_rows = lines_vec.len();
    let num_cols = lines_vec[0].len();

    for (row, line) in lines_vec.iter().enumerate() {
        let line_chars: Vec<char> = line.chars().collect();
        for (col, c) in line_chars.iter().enumerate() {
            let node_id: String = format!("{};{}", row, col);

            let current_height = height_of(*c) as i8;

            graph.nodes.push(Node::new(node_id.clone(), *c));

            // Define all potential neighbors (dr, dc, target_char, target_id)
            let neighbors: [Option<(isize, isize, char)>; 4] = [
                // (dr, dc, target_char)
                if col > 0 {
                    Some((0, -1, line_chars[col - 1]))
                } else {
                    None
                }, // Left
                if row > 0 {
                    Some((-1, 0, lines_vec[row - 1].chars().nth(col).unwrap()))
                } else {
                    None
                }, // Up
                if col < num_cols - 1 {
                    Some((0, 1, line_chars[col + 1]))
                } else {
                    None
                }, // Right
                if row < num_rows - 1 {
                    Some((1, 0, lines_vec[row + 1].chars().nth(col).unwrap()))
                } else {
                    None
                }, // Down
            ];

            for neighbor in neighbors.iter().filter_map(|&n| n) {
                let (dr, dc, target_char) = neighbor;

                let to_row = (row as isize + dr) as usize;
                let to_col = (col as isize + dc) as usize;

                let to_id = format!("{};{}", to_row, to_col);
                let to_height = height_of(target_char) as i8;

                // CRITICAL MOVEMENT RULE: Can only move if neighbor height is AT MOST 1 higher.
                let diff = to_height - current_height;

                if diff <= 1 {
                    // Weight is 1 for every step taken
                    graph.edges.push(Edge {
                        from: node_id.clone(),
                        to: to_id,
                        weight: 1,
                    });
                }
            }
        }
    }

    let start_node = graph.nodes.iter().find(|n| n.label == 'S').unwrap();
    let end_node = graph.nodes.iter().find(|n| n.label == 'E').unwrap();
    println!(
        "{}",
        graph
            .shortest_path(start_node.id.clone(), end_node.id.clone())
            .len()
            - 1 // -1 because the path includes the start node, which is not a step
    );

    graph.visualize_path(graph.shortest_path(start_node.id.clone(), end_node.id.clone()));
}

#[allow(unused)]
fn part2(input: &str) {
    let mut lines = input.lines();
    let lines_vec: Vec<_> = lines.clone().collect();

    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };

    let num_rows = lines_vec.len();
    let num_cols = lines_vec[0].len();

    for (row, line) in lines_vec.iter().enumerate() {
        let line_chars: Vec<char> = line.chars().collect();
        for (col, c) in line_chars.iter().enumerate() {
            let node_id: String = format!("{};{}", row, col);

            let current_height = height_of(*c) as i8;

            graph.nodes.push(Node::new(node_id.clone(), *c));

            // Define all potential neighbors (dr, dc, target_char, target_id)
            let neighbors: [Option<(isize, isize, char)>; 4] = [
                // (dr, dc, target_char)
                if col > 0 {
                    Some((0, -1, line_chars[col - 1]))
                } else {
                    None
                }, // Left
                if row > 0 {
                    Some((-1, 0, lines_vec[row - 1].chars().nth(col).unwrap()))
                } else {
                    None
                }, // Up
                if col < num_cols - 1 {
                    Some((0, 1, line_chars[col + 1]))
                } else {
                    None
                }, // Right
                if row < num_rows - 1 {
                    Some((1, 0, lines_vec[row + 1].chars().nth(col).unwrap()))
                } else {
                    None
                }, // Down
            ];

            for neighbor in neighbors.iter().filter_map(|&n| n) {
                let (dr, dc, target_char) = neighbor;

                let to_row = (row as isize + dr) as usize;
                let to_col = (col as isize + dc) as usize;

                let to_id = format!("{};{}", to_row, to_col);
                let to_height = height_of(target_char) as i8;

                // CRITICAL MOVEMENT RULE: Can only move if neighbor height is AT MOST 1 higher.
                let diff = to_height - current_height;

                if diff <= 1 {
                    // Weight is 1 for every step taken
                    graph.edges.push(Edge {
                        from: node_id.clone(),
                        to: to_id,
                        weight: 1,
                    });
                }
            }
        }
    }

    let start_nodes = graph
        .nodes
        .iter()
        .filter(|n| n.label == 'b')
        .collect::<Vec<_>>();
    // so i noticed in the input theres only 1 full column of b's,
    // i could compute all the starting point from a'a sure but we can just compute the paths from the b's and add 1 to the result
    // since you can ONLY move from a to b
    // this saves a lot of compute time and avoids requiring optimizations

    let end_node = graph.nodes.iter().find(|n| n.label == 'E').unwrap();

    let mut shortest_path = usize::MAX;
    for start_node in start_nodes {
        let path = graph.shortest_path(start_node.id.clone(), end_node.id.clone());
        if path.len() > 0 && path.len() < shortest_path {
            shortest_path = path.len();
        }
    }
    println!("{}", shortest_path);
}
