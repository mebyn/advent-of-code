use std::fs;

fn main() {
    let input = fs::read_to_string("day06/input.txt").expect("Failed to read input file");

    println!("Day 6: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> u64 {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let grid: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.trim().split_whitespace().collect())
        .collect();

    let mut numbers: Vec<Vec<&str>> = Vec::new();
    for x in 0..grid[0].len() {
        let mut row = Vec::new();
        for y in 0..grid.len() {
            row.push(grid[y][x]);
        }
        numbers.push(row);
    }
    println!("Numbers = {:?}", numbers);
    numbers
        .iter()
        .map(|problem| {
            let operation = problem.last().unwrap();
            let numbers = problem[..problem.len() - 1]
                .iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            match *operation {
                "*" => numbers.iter().product(),
                "+" => numbers.iter().sum(),
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "
        123 328  51 64 
        45 64  387 23 
        6 98  215 314
        *   +   *   + 
        ";
        assert_eq!(solution(input), 4277556);
    }
}
