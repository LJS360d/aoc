/// Advent of Code 2022 - Day 7
/// https://adventofcode.com/2022/day/7
fn main() {
    let input = include_str!("./input.txt");
    part1(&input);
    // part2(&input);
}

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct ArenaTree {
    pub nodes: Vec<Node>,
}

impl ArenaTree {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn add(&mut self, name: String, data: FileOrDir, parent_id: Option<NodeId>) -> NodeId {
        let new_id = self.nodes.len();

        let new_node = Node {
            name,
            data,
            parent: parent_id,
            children: Vec::new(),
        };

        self.nodes.push(new_node);

        if let Some(p_id) = parent_id {
            if let Some(parent_node) = self.nodes.get_mut(p_id) {
                parent_node.children.push(new_id);
            }
        }

        new_id
    }

    pub fn dirs_under_100000(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| node.size(self) < 100000)
            .collect()
    }

    pub fn print(&self) {
        self.print_node(0, 0);
    }

    fn print_node(&self, id: NodeId, depth: usize) {
        let node = &self.nodes[id];
        if node.size(self) <= 100000 {
            println!(
                "{:indent$}{} ({})",
                "",
                node.name,
                node.size(self),
                indent = depth * 2
            );
        }

        for &child_id in &node.children {
            self.print_node(child_id, depth + 1);
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileOrDir {
    Directory,
    File { size: u64 },
}
#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    data: FileOrDir,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl Node {
    pub fn size(&self, tree: &ArenaTree) -> u64 {
        return match self.data {
            FileOrDir::File { size } => size,
            FileOrDir::Directory => self
                .children
                .iter()
                .map(|&child_id| {
                    if let Some(child_node) = tree.nodes.get(child_id) {
                        child_node.size(tree)
                    } else {
                        0
                    }
                })
                .sum(),
        };
    }
}

#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut tree = ArenaTree::new();
    let mut cwd = tree.add(String::from("/"), FileOrDir::Directory, None);
    for line in lines.skip(1) {
        if line.starts_with("$ cd") {
            let dir = line.chars().skip(5).collect::<String>();
            let cwd_node = tree.nodes.get(cwd).unwrap();
            if dir == ".." {
                match cwd_node.parent {
                    Some(parent) => cwd = parent,
                    None => {}
                }
                continue;
            }
            let children_dirs = tree
                .nodes
                .get(cwd)
                .unwrap()
                .children
                .clone()
                .iter()
                .filter_map(|node| {
                    if matches!(tree.nodes.get(*node).unwrap().data, FileOrDir::Directory) {
                        Some(*node)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for child in children_dirs.into_iter() {
                let node = tree.nodes.get(child).unwrap();
                if node.name == dir {
                    cwd = child;
                    break;
                }
            }
        } else if line.starts_with("$ ls") {
            // well... we need to process the next lines here
            continue;
        } else if line.starts_with("dir ") {
            let dir = line.chars().skip(4).collect::<String>();
            match tree.nodes.iter().find(|node| node.name == dir) {
                Some(folder) => {}
                None => {
                    tree.add(dir, FileOrDir::Directory, Some(cwd));
                }
            }
        } else {
            // line has file entry
            let (size_str, name) = line.split_once(" ").unwrap();
            let size = size_str.parse::<u64>().unwrap();
            tree.add(
                String::from(name),
                FileOrDir::File { size: size },
                Some(cwd),
            );
        }
    }
    tree.print();
    println!(
        "{:?}",
        tree.dirs_under_100000()
            .iter()
            .map(|d| d.size(&tree))
            .collect::<Vec<u64>>()
            .iter()
            .sum::<u64>()
    );
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    for line in lines {
        let mut chars = line.chars();
    }
}
