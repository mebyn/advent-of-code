use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    fs,
    rc::Rc,
};

fn main() {
    let input = fs::read_to_string("day07/input.txt").expect("Failed to read input file");

    println!("Day 7: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

const BEAM_ENTRACE: char = 'S';

type NodeId = (usize, usize);
type NodeRef = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    id: NodeId,
    edges: Vec<NodeRef>,
    has_splitten: bool,
    is_visited: bool,
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<NodeId, NodeRef>,
}

impl Tree {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: NodeId) -> NodeRef {
        let new_node = || {
            Rc::new(RefCell::new(Node {
                id,
                edges: vec![],
                has_splitten: false,
                is_visited: false,
            }))
        };
        match self.nodes.entry(id) {
            Entry::Occupied(o) => o.get().clone(),
            Entry::Vacant(v) => v.insert(new_node()).clone(),
        }
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        let parent = self.add_node(from);
        let child = self.add_node(to);
        parent.borrow_mut().edges.push(child);
    }

    fn count_split_occurrence(&self, root: NodeId) -> u64 {
        fn count_split(tree: &Tree, id: NodeId) -> u64 {
            let node_ref = match tree.nodes.get(&id) {
                Some(n) => n,
                None => return 0,
            };
            let mut node = node_ref.borrow_mut();
            if node.is_visited {
                return 0;
            }
            node.is_visited = true;
            let mut split_count = node.has_splitten as u64;
            let child_ids = node
                .edges
                .iter()
                .map(|c| c.borrow().id)
                .collect::<Vec<(usize, usize)>>();
            for child_id in child_ids {
                split_count += count_split(tree, child_id);
            }
            split_count
        }
        count_split(self, root)
    }
}

fn build_tree(grid: &[Vec<char>], start: NodeId) -> Tree {
    let mut tree = Tree::new();
    tree.add_node(start);

    let mut current_positions = vec![start]; // positions in current row

    for y in (start.0 + 1)..grid.len() {
        let mut next = Vec::new();
        for &(_, x) in &current_positions {
            let node_in_scope = (y - 1, x);
            let ch = grid[y][x];
            match ch {
                '^' => {
                    let nis = tree
                        .nodes
                        .get(&node_in_scope)
                        .expect("Unable to find node!");
                    nis.borrow_mut().has_splitten = true;
                    if x > 0 {
                        tree.add_edge(node_in_scope, (y, x - 1));
                        next.push((y, x - 1));
                    }
                    if x + 1 < grid[y].len() {
                        tree.add_edge(node_in_scope, (y, x + 1));
                        next.push((y, x + 1));
                    }
                }
                _ => {
                    tree.add_edge(node_in_scope, (y, x));
                    next.push((y, x));
                }
            }
        }
        next.sort();
        next.dedup();
        current_positions = next;
    }
    tree
}

fn parse_grid(input: &str) -> (Vec<Vec<char>>, NodeId) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let start_row = grid
        .iter()
        .position(|row| row.contains(&BEAM_ENTRACE))
        .expect("missing S");
    let start_col = grid[start_row]
        .iter()
        .position(|c| *c == BEAM_ENTRACE)
        .unwrap();

    (grid, (start_row, start_col))
}

fn solution(input: &str) -> u64 {
    let (grid, start_position) = parse_grid(input);
    let tree = build_tree(&grid, start_position);
    tree.count_split_occurrence(start_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
        ";
        assert_eq!(solution(input), 21);
    }
}
