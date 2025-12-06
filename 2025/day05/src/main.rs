use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("day05/input.txt").expect("Failed to read input file");

    println!("Day 5: Advent of Code 2025");
    println!("=========================");
    let result_1 = solution_part_1(&input);
    println!("Result Part 1: {}", result_1); //733

    let result_2 = solution_part_2(&input);
    println!("Result Part 2: {}", result_2); //345821388687084
}

#[derive(Debug)]
struct Inventory {
    ranges: Vec<(u64, u64)>,
    ingredients: HashSet<u64>,
}

impl Inventory {
    fn new(input: &str) -> Self {
        let (ranges, ingredients) = input.split_once("\n\n").unwrap();
        let range_pairs = ranges
            .lines()
            .map(|line| {
                let (start, end) = line.trim().split_once('-').unwrap();
                let start = start.parse::<u64>().unwrap();
                let end = end.parse::<u64>().unwrap();
                (start, end)
            })
            .collect::<Vec<(u64, u64)>>();
        let ingredients = ingredients
            .lines()
            .map(|line| line.trim().parse::<u64>().unwrap())
            .collect::<HashSet<u64>>();
        Self {
            ranges: range_pairs,
            ingredients,
        }
    }

    fn find_fresh_ingredients_from_stash(&self) -> Vec<u64> {
        self.ingredients
            .iter()
            .filter(|ingredient| {
                self.ranges
                    .iter()
                    .any(|(start, end)| ingredient >= &start && ingredient <= &end)
            })
            .cloned()
            .collect()
    }

    fn find_all_fresh_ingredients(&self) -> Vec<(u64, u64)> {
        let mut sorted_range_stack = self.ranges.clone();
        sorted_range_stack.sort_by_key(|range| std::cmp::Reverse(range.0));
        let mut bucket: Vec<(u64, u64)> = Vec::new();
        while let Some((next_start, next_end)) = sorted_range_stack.pop() {
            match bucket.last_mut() {
                Some((_, last_end)) if *last_end >= next_start => {
                    *last_end = (*last_end).max(next_end);
                }
                _ => {
                    bucket.push((next_start, next_end));
                }
            }
        }
        bucket
    }
}

fn solution_part_1(input: &str) -> u64 {
    let inventory = Inventory::new(input);
    let fresh_ingredients = inventory.find_fresh_ingredients_from_stash();
    fresh_ingredients.len() as u64
}

fn solution_part_2(input: &str) -> u64 {
    let inventory = Inventory::new(input);
    let fresh_ingredients = inventory.find_all_fresh_ingredients();
    fresh_ingredients
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part_1() {
        let input = "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";
        assert_eq!(solution_part_1(input), 3);
    }

    #[test]
    fn test_solution_part_2() {
        let input = "3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32";
        assert_eq!(solution_part_2(input), 14);
    }
}
