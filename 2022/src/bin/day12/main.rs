use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
    iter,
};

/// Advent of Code 2022 - Day 12
/// https://adventofcode.com/2022/day/12
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}
#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}
type AdjacencyList = HashMap<usize, Vec<(usize, u8)>>;

impl Graph {
    fn build_adj_list(&self) -> AdjacencyList {
        let mut adj = HashMap::new();
        for node in &self.nodes {
            adj.insert(node.id, Vec::new());
        }
        // Populate the entries with edges
        for edge in &self.edges {
            if let Some(neighbors) = adj.get_mut(&edge.from) {
                neighbors.push((edge.to, edge.weight));
            }
        }
        adj
    }

    fn shortest_path(&self, start: usize, end: usize) -> Vec<usize> {
        let adj_list = self.build_adj_list();

        // Deque for 0-1 BFS
        let mut deque: VecDeque<usize> = VecDeque::new();

        // Distance map: Node ID -> Minimum distance from start
        let mut distance: HashMap<usize, usize> = HashMap::new();

        // Predecessor map: Child -> Parent for path reconstruction
        let mut predecessors: HashMap<usize, usize> = HashMap::new();

        // Initialize all nodes to max distance, or 0 for the start node
        for node in &self.nodes {
            distance.insert(node.id, usize::MAX);
        }

        distance.insert(start, 0);
        deque.push_back(start);

        // 0-1 BFS Traversal
        while let Some(current) = deque.pop_front() {
            if current == end {
                break; // Path found
            }

            let current_dist = *distance.get(&current).unwrap_or(&usize::MAX);

            if let Some(neighbors) = adj_list.get(&current) {
                for (neighbor, weight) in neighbors {
                    let weight = *weight as usize;

                    let new_dist = current_dist + weight;
                    if new_dist < *distance.get(neighbor).unwrap_or(&usize::MAX) {
                        distance.insert(*neighbor, new_dist);
                        predecessors.insert(*neighbor, current);

                        if weight == 0 {
                            deque.push_front(*neighbor);
                        } else if weight == 1 {
                            deque.push_back(*neighbor);
                        }
                    }
                }
            }
        }

        let mut path = Vec::new();
        let mut current_step = end;

        if *distance.get(&end).unwrap_or(&usize::MAX) == usize::MAX {
            return Vec::new();
        }

        while let Some(&prev) = predecessors.get(&current_step) {
            path.push(current_step);
            current_step = prev;
            if current_step == start {
                path.push(start);
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

    fn visualize_path(&self, path: Vec<usize>) {
        for (i, node) in self.nodes.iter().enumerate() {
            if path.contains(&node.id) {
                print!("{}", of_height(node.height));
            } else {
                print!(".");
            }

            if i % 143 == 0 {
                println!();
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    id: usize,
    height: u8,
}

impl Node {
    fn new(id: usize, height: char) -> Self {
        Node {
            id,
            height: height_of(height),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    weight: u8,
}

fn height_of(c: char) -> u8 {
    match c {
        'E' => 27,
        'S' => 0,
        letter => letter as u8 - b'a' + 1,
    }
}

fn of_height(c: u8) -> char {
    match c {
        27 => 'E',
        0 => 'S',
        letter => ((letter + b'a' - 1) as char).to_ascii_lowercase(),
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

    for (row, line) in lines_vec.iter().enumerate() {
        let line_chars: Vec<char> = line.chars().collect();
        for (col, c) in line_chars.iter().enumerate() {
            let node_id: usize = (row + 1) * (col + 1);
            let node_id_str = format!("{row};{col}").as_str();

            let node_height = height_of(*c) as i8;

            graph.nodes.push(Node::new(node_id, *c));
            if col != 0 {
                // left
                let to_height = height_of(line_chars[col - 1]) as i8;
                graph.edges.push(Edge {
                    from: node_id,
                    to: (col) * (row + 1),
                    weight: (to_height - node_height).abs() as u8,
                });
            }
            if row != 0 {
                // up
                let to_height = height_of(lines_vec[row - 1].chars().nth(col).unwrap()) as i8;

                graph.edges.push(Edge {
                    from: node_id,
                    to: (col + 1) * (row),
                    weight: (to_height - node_height).abs() as u8,
                });
            }
            if col != line.len() - 1 {
                // right
                let to_height = height_of(line_chars[col + 1]) as i8;

                graph.edges.push(Edge {
                    from: node_id,
                    to: (col + 2) * (row + 1),
                    weight: (to_height - node_height).abs() as u8,
                });
            }
            if row != lines.clone().count() - 1 {
                // down
                let to_height = height_of(lines_vec[row + 1].chars().nth(col).unwrap()) as i8;
                graph.edges.push(Edge {
                    from: node_id,
                    to: (col + 1) * (row + 2),
                    weight: (to_height - node_height).abs() as u8,
                });
            }
            // no diagonal edges
        }
    }
    let start_node = graph.nodes.iter().find(|n| n.height == 0).unwrap();
    let end_node = graph.nodes.iter().find(|n| n.height == 27).unwrap();
    println!("{}", graph.shortest_path(start_node.id, end_node.id).len());

    graph.visualize_path(graph.shortest_path(start_node.id, end_node.id));
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
