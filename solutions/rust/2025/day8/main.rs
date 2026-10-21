use regex::Regex;
use std::collections::{BTreeMap, HashMap};

/// Advent of Code 2025 - Day 8
/// https://adventofcode.com/2025/day/8
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
}

type Node = (u32, u32, u32);
type Edge = (Node, Node);
type Graph = (Vec<Node>, Vec<Edge>);

struct DSU {
    // parent[i] is the parent of element i. If parent[i] == i, i is the representative.
    parent: Vec<usize>,
    // size[i] stores the size of the set rooted at i. Used for Union by Size optimization.
    size: Vec<usize>,
    // The number of disjoint sets (components).
    num_components: usize,
}

impl DSU {
    /// Creates a new DSU structure with N elements, each in its own set.
    pub fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_components: n,
        }
    }

    /// Finds the representative (root) of the set containing element 'i'.
    /// Implements Path Compression for optimization.
    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            // i is the root
            return i;
        }
        // Path Compression: Set the parent directly to the root
        let root = self.find(self.parent[i]);
        self.parent[i] = root;
        root
    }

    /// Unites the sets containing elements 'i' and 'j'.
    /// Returns true if a union occurred (i.e., they were in different sets).
    /// Implements Union by Size for optimization.
    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            // They are in different sets, so merge them.
            // Union by Size: Attach smaller tree to the root of the larger tree.
            if self.size[root_i] < self.size[root_j] {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            self.num_components -= 1;
            return true;
        }
        // Already in the same set, no union necessary.
        false
    }
}

/// Finds all connected components (sub-graphs) within the given set of nodes and edges.
fn find_connected_components(nodes: &Vec<Node>, edges: &Vec<Edge>) -> Vec<Graph> {
    let mut node_to_id: HashMap<&Node, usize> = HashMap::new();
    let mut id_to_node: Vec<&Node> = Vec::with_capacity(nodes.len());

    for node in nodes.into_iter() {
        if !node_to_id.contains_key(&node) {
            let id = id_to_node.len();
            node_to_id.insert(node, id);
            id_to_node.push(node);
        }
    }

    let num_unique_nodes = id_to_node.len();
    if num_unique_nodes == 0 {
        return Vec::new(); // Empty graph
    }

    let mut dsu = DSU::new(num_unique_nodes);

    for (u, v) in edges.iter() {
        if let (Some(&id_u), Some(&id_v)) = (node_to_id.get(u), node_to_id.get(v)) {
            dsu.union(id_u, id_v);
        }
        // Edges with non-existent nodes are ignored here
    }

    // Group Nodes by Component Root
    // BTreeMap mapping Root ID to a list of Node IDs.
    let mut component_nodes: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for id in 0..num_unique_nodes {
        let leader_id = dsu.find(id);
        component_nodes.entry(leader_id).or_default().push(id);
    }

    // Build Final Component Graphs (Vec<Graph>)
    let mut result_graphs: Vec<Graph> = Vec::new();

    // Map component node IDs back to their coordinate tuples for the final output
    for (_leader_id, node_ids_in_component) in component_nodes.into_iter() {
        let component_coords: Vec<Node> = node_ids_in_component
            .iter()
            .map(|&id| *id_to_node[id])
            .collect();

        let component_node_set: HashMap<Node, ()> =
            component_coords.iter().map(|&coord| (coord, ())).collect();

        let mut component_edges: Vec<Edge> = Vec::new();

        // Filter the original edges to find only those belonging to this component
        for (u, v) in edges.iter() {
            // An edge belongs to a component if both its endpoints are in that component.
            if component_node_set.contains_key(u) && component_node_set.contains_key(v) {
                component_edges.push((*u, *v));
            }
        }

        // Push the complete component graph
        result_graphs.push((component_coords, component_edges));
    }

    result_graphs
}

fn euclidean_distance(p1: Node, p2: Node) -> u32 {
    let (x1, y1, z1) = p1;
    let (x2, y2, z2) = p2;
    let dx = (x1 as i64 - x2 as i64).pow(2);
    let dy = (y1 as i64 - y2 as i64).pow(2);
    let dz = (z1 as i64 - z2 as i64).pow(2);
    (dx + dy + dz).isqrt() as u32
}

#[allow(unused)]
fn part1(input: &str) {
    let mut edges: Vec<Edge> = Vec::new();
    let mut nodes = Vec::<Node>::new();
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    for line in input.lines() {
        let (_, [x_str, y_str, z_str]) = re.captures(line).unwrap().extract::<3>();
        let x = x_str.parse::<u32>().unwrap();
        let y = y_str.parse::<u32>().unwrap();
        let z = z_str.parse::<u32>().unwrap();
        nodes.push((x, y, z));
    }
    let mut adj_map: HashMap<Edge, u32> = HashMap::new();
    for &n1 in nodes.iter() {
        for &n2 in nodes.iter() {
            if n1 == n2 {
                continue;
            }
            if adj_map.contains_key(&(n2, n1)) {
                continue;
            }
            let _ = adj_map.insert((n1, n2), euclidean_distance(n1, n2));
        }
    }

    let mut adj_list = adj_map.clone().into_iter().collect::<Vec<(Edge, u32)>>();
    adj_list.sort_unstable_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b));
    const PAIRS: usize = 1000;
    for i in 0..PAIRS {
        let Some((pair, _)) = adj_list.get(i) else {
            break;
        };
        edges.push(*pair);
    }

    let mut circuits = find_connected_components(&nodes, &edges);
    circuits.sort_unstable_by(|(a, _), (b, _)| a.len().cmp(&b.len()));
    println!(
        "{:?}",
        circuits
            .iter()
            .rev()
            .take(3)
            .map(|(n, _)| n.len())
            .fold(1, |acc, n| acc * n)
    )
}

#[allow(unused)]
fn part2(input: &str) {
    let mut edges: Vec<Edge> = Vec::new();
    let mut nodes = Vec::<Node>::new();
    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();
    for line in input.lines() {
        let (_, [x_str, y_str, z_str]) = re.captures(line).unwrap().extract::<3>();
        let x = x_str.parse::<u32>().unwrap();
        let y = y_str.parse::<u32>().unwrap();
        let z = z_str.parse::<u32>().unwrap();
        nodes.push((x, y, z));
    }
    let mut adj_map: HashMap<Edge, u32> = HashMap::new();
    for &n1 in nodes.iter() {
        for &n2 in nodes.iter() {
            if n1 == n2 {
                continue;
            }
            if adj_map.contains_key(&(n2, n1)) {
                continue;
            }
            let _ = adj_map.insert((n1, n2), euclidean_distance(n1, n2));
        }
    }

    let mut adj_list = adj_map.clone().into_iter().collect::<Vec<(Edge, u32)>>();
    adj_list.sort_unstable_by(|(_, dist_a), (_, dist_b)| dist_a.cmp(dist_b).reverse());
    let mut i = 2000;
    loop {
        i -= 1;
        let Some((pair, _)) = adj_list.pop() else {
            break;
        };
        edges.push(pair);
        if i != 0 {
            continue;
        }
        let circuits = find_connected_components(&nodes, &edges).len();
        if circuits == 1 {
            // we did it
            let ((x1, _, _), (x2, _, _)) = pair;
            println!("{:?}", x1 * x2);
            return;
        }
        i += circuits;
    }
}
