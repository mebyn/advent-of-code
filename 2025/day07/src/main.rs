use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, hash_map::Entry},
    fs,
    rc::Rc,
};

fn main() {
    let input = fs::read_to_string("day07/input.txt").expect("Failed to read input file");

    println!("Day 7: Advent of Code 2025");
    println!("=========================");

    println!("Part 01");
    println!("Result: {}", solution_1(&input));

    println!("\n=========================\n");

    println!("Part 02");
    println!("Result: {}", solution_2(&input));
}

const BEAM_ENTRANCE: char = 'S';

type NodeId = (usize, usize);

#[derive(Debug, Clone)]
struct Node {
    id: NodeId,
    edges: Vec<NodeId>,
    has_split: bool,
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<NodeId, Node>,
}

impl Tree {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: NodeId) -> &mut Node {
        match self.nodes.entry(id) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let new_node = Node {
                    id: id,
                    edges: Vec::new(),
                    has_split: false,
                };
                v.insert(new_node)
            }
        }
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.add_node(to);
        let parent = self.add_node(from);
        parent.edges.push(to);
    }

    fn count_split_occurrence(&self, root: NodeId) -> u64 {
        fn count_split(tree: &Tree, id: NodeId, visited_nodes: &mut HashSet<NodeId>) -> u64 {
            let node = match tree.nodes.get(&id) {
                Some(n) => n,
                None => return 0,
            };
            if visited_nodes.contains(&id) {
                return 0;
            }
            visited_nodes.insert(id);
            let mut split_count = node.has_split as u64;
            let child_ids = node
                .edges
                .iter()
                .map(|c| c)
                .collect::<Vec<&(usize, usize)>>();
            for child_id in child_ids {
                split_count += count_split(tree, *child_id, visited_nodes);
            }
            split_count
        }
        count_split(self, root, &mut HashSet::new())
    }

    fn count_timelines(&self, root: NodeId) -> u64 {
        fn count_timeline(
            tree: &Tree,
            id: NodeId,
            already_traversed: &mut HashMap<NodeId, u64>,
        ) -> u64 {
            if let Some(&node_traversed) = already_traversed.get(&id) {
                return node_traversed;
            }
            let node = match tree.nodes.get(&id) {
                Some(n) => n,
                None => panic!("Unable to find node!"),
            };
            let node_id = node.id;
            let mut timeline_count = 0;
            let child_ids = node
                .edges
                .iter()
                .map(|c| c)
                .collect::<Vec<&(usize, usize)>>();
            if child_ids.is_empty() {
                return 1;
            }
            for child_id in child_ids {
                timeline_count += count_timeline(tree, *child_id, already_traversed);
            }
            already_traversed.insert(node_id, timeline_count);
            timeline_count
        }
        count_timeline(self, root, &mut HashMap::new())
    }
}

fn build_tree(grid: &[Vec<char>], start: NodeId) -> Tree {
    let mut tree = Tree::new();
    tree.add_node(start);

    let mut current_positions = vec![start];

    for y in (start.0 + 1)..grid.len() {
        let mut next_beams = Vec::new();
        for &(_, x) in &current_positions {
            let node_in_scope = (y - 1, x);
            let ch = grid[y][x];
            match ch {
                '^' => {
                    let nis = tree
                        .nodes
                        .get_mut(&node_in_scope)
                        .expect("Unable to find node!");
                    nis.has_split = true;
                    if x > 0 {
                        let split_left = (y, x - 1);
                        tree.add_edge(node_in_scope, split_left);
                        next_beams.push(split_left);
                    }
                    if x + 1 < grid[y].len() {
                        let split_right = (y, x + 1);
                        tree.add_edge(node_in_scope, split_right);
                        next_beams.push(split_right);
                    }
                }
                _ => {
                    let position = (y, x);
                    tree.add_edge(node_in_scope, position);
                    next_beams.push(position);
                }
            }
        }
        next_beams.sort();
        next_beams.dedup();
        current_positions = next_beams;
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
        .position(|row| row.contains(&BEAM_ENTRANCE))
        .expect("missing S");
    let start_col = grid[start_row]
        .iter()
        .position(|c| *c == BEAM_ENTRANCE)
        .unwrap();

    (grid, (start_row, start_col))
}

fn solution_1(input: &str) -> u64 {
    let (grid, start_position) = parse_grid(input);
    let tree = build_tree(&grid, start_position);
    tree.count_split_occurrence(start_position)
}

fn solution_2(input: &str) -> u64 {
    let (grid, start_position) = parse_grid(input);
    let tree = build_tree(&grid, start_position);
    tree.count_timelines(start_position)
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
        assert_eq!(solution_1(input), 21);
    }

    #[test]
    fn test_timelines() {
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
        assert_eq!(solution_2(input), 40);
    }
}
