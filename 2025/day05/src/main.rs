use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("day05/input.txt").expect("Failed to read input file");

    println!("Day 5: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
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

    fn find_fresh_ingredients(&self) -> Vec<u64> {
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
}

fn solution(input: &str) -> u64 {
    let inventory = Inventory::new(input);
    let fresh_ingredients = inventory.find_fresh_ingredients();
    println!("Fresh ingredients: {fresh_ingredients:?}");
    fresh_ingredients.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
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
        assert_eq!(solution(input), 3);
    }
}
