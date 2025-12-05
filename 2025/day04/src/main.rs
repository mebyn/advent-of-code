use std::fs;

fn main() {
    let input = fs::read_to_string("day04/input.txt").expect("Failed to read input file");

    println!("Day 4: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> i64 {
    // TODO: Implement solution
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = ""; // Add test input
        assert_eq!(solution(input), 0);
    }
}
