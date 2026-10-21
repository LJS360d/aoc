/// Advent of Code 2022 - Day 7
/// https://adventofcode.com/2022/day/7
fn main() {
    let input = include_str!("./input.txt");
    // part1(&input);
    part2(&input);
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
        println!(
            "{:indent$}{} ({})",
            "",
            node.name,
            node.size(self),
            indent = depth * 2
        );

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
#[allow(unused)]
fn part1(input: &str) {
    let lines = input.lines();

    let mut tree = ArenaTree::new();
    let mut cwd: NodeId = tree.add(String::from("/"), FileOrDir::Directory, None);

    for line in lines.skip(1) {
        if line.starts_with("$ cd") {
            let dir = line.chars().skip(5).collect::<String>();
            let cwd_node = tree.nodes.get(cwd).unwrap();
            if dir == ".." {
                match cwd_node.parent {
                    Some(parent) => cwd = parent,
                    None => {}
                }
            } else {
                let mut found = false;
                for &child_id in &cwd_node.children {
                    if tree.nodes[child_id].name == dir {
                        cwd = child_id;
                        found = true;
                        break;
                    }
                }
                if !found {
                    // NTD
                }
            }
        } else if line.starts_with("$ ls") {
            // well... we need to process the next lines here
            continue;
        } else if line.starts_with("dir ") {
            let dir_name = line.chars().skip(4).collect::<String>();

            let cwd_children = &tree.nodes[cwd].children;
            let already_exists = cwd_children.iter().any(|&child_id| {
                tree.nodes[child_id].name == dir_name
                    && matches!(tree.nodes[child_id].data, FileOrDir::Directory)
            });

            if !already_exists {
                tree.add(dir_name, FileOrDir::Directory, Some(cwd));
            }
        } else {
            let (size_str, name) = line.split_once(" ").unwrap();
            let size = size_str.parse::<u64>().unwrap();

            let file_name = String::from(name);
            let cwd_children = &tree.nodes[cwd].children;
            let already_exists = cwd_children.iter().any(|&child_id| {
                tree.nodes[child_id].name == file_name
                    && matches!(tree.nodes[child_id].data, FileOrDir::File { .. })
            });

            if !already_exists {
                tree.add(file_name, FileOrDir::File { size }, Some(cwd));
            }
        }
    }

    let result: u64 = tree
        .nodes
        .iter()
        .filter_map(|node| {
            if matches!(node.data, FileOrDir::Directory) {
                let size = node.size(&tree);
                if size <= 100000 { Some(size) } else { None }
            } else {
                None
            }
        })
        .sum();

    println!("{}", result);
}

#[allow(unused)]
fn part2(input: &str) {
    let lines = input.lines();

    let mut tree = ArenaTree::new();
    let mut cwd: NodeId = tree.add(String::from("/"), FileOrDir::Directory, None);

    for line in lines.skip(1) {
        if line.starts_with("$ cd") {
            let dir = line.chars().skip(5).collect::<String>();
            let cwd_node = tree.nodes.get(cwd).unwrap();
            if dir == ".." {
                match cwd_node.parent {
                    Some(parent) => cwd = parent,
                    None => {}
                }
            } else {
                let mut found = false;
                for &child_id in &cwd_node.children {
                    if tree.nodes[child_id].name == dir {
                        cwd = child_id;
                        found = true;
                        break;
                    }
                }
                if !found {
                    // NTD
                }
            }
        } else if line.starts_with("$ ls") {
            // well... we need to process the next lines here
            continue;
        } else if line.starts_with("dir ") {
            let dir_name = line.chars().skip(4).collect::<String>();

            let cwd_children = &tree.nodes[cwd].children;
            let already_exists = cwd_children.iter().any(|&child_id| {
                tree.nodes[child_id].name == dir_name
                    && matches!(tree.nodes[child_id].data, FileOrDir::Directory)
            });

            if !already_exists {
                tree.add(dir_name, FileOrDir::Directory, Some(cwd));
            }
        } else {
            let (size_str, name) = line.split_once(" ").unwrap();
            let size = size_str.parse::<u64>().unwrap();

            let file_name = String::from(name);
            let cwd_children = &tree.nodes[cwd].children;
            let already_exists = cwd_children.iter().any(|&child_id| {
                tree.nodes[child_id].name == file_name
                    && matches!(tree.nodes[child_id].data, FileOrDir::File { .. })
            });

            if !already_exists {
                tree.add(file_name, FileOrDir::File { size }, Some(cwd));
            }
        }
    }

    let total_space: u64 = 70000000;
    let required_space: u64 = 30000000;
    let unused_space = total_space - tree.nodes.get(0).unwrap().size(&tree);
    let free_target = required_space - unused_space;

    let mut result = total_space;
    for node in &tree.nodes {
        if matches!(node.data, FileOrDir::Directory) {
            let size = node.size(&tree);
            if free_target <= size && result > size {
                result = size;
            }
        }
    }
    println!("{}", result);
}
