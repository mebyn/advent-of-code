use std::fs;

fn main() {
    let input = fs::read_to_string("day06/input.txt").expect("Failed to read input file");

    println!("Day 6: Advent of Code 2025");
    println!("=========================");

    println!("Result Solution 1: {}", solution_1(&input)); //5784380717354
    println!("Result Solution 2: {}", solution_2(&input)); //7996218225744
}

struct Problem {
    numbers: Vec<u64>,
    operation: fn(u64, u64) -> u64,
}

impl Problem {
    const ADD: fn(u64, u64) -> u64 = |a: u64, b: u64| a + b;
    const MULTIPLY: fn(u64, u64) -> u64 = |a: u64, b: u64| a * b;

    fn map_problem<'a>(worksheet: &'a Worksheet<'a>) -> Vec<(Vec<&'a str>, fn(u64, u64) -> u64)> {
        worksheet
            .pivot_grid()
            .iter()
            .map(|problem| {
                let numbers = problem[..problem.len() - 1].iter().copied().collect();
                let operation = problem.last().or_else(|| panic!("Invalid operation!"));
                (
                    numbers,
                    match operation {
                        Some(o) if o.trim() == "*" => Self::MULTIPLY,
                        Some(o) if o.trim() == "+" => Self::ADD,
                        _ => panic!("Invalid operation"),
                    },
                )
            })
            .collect()
    }

    fn map_problem_1(worksheet: &Worksheet) -> Vec<Problem> {
        let problems = Problem::map_problem(worksheet);
        problems
            .iter()
            .map(|(numbers, operation)| Problem {
                numbers: numbers.iter().map(|s| s.parse::<u64>().unwrap()).collect(),
                operation: *operation,
            })
            .collect()
    }

    fn map_problem_2(worksheet: &Worksheet) -> Vec<Problem> {
        let problems = Problem::map_problem(worksheet);
        problems
            .iter()
            .map(|p| {
                let (numbers, operation) = p;
                let max_len = numbers
                    .iter()
                    .max_by(|n1, n2| n1.len().cmp(&n2.len()))
                    .unwrap_or_else(|| panic!("No max len"))
                    .len();
                let mut column_numbers: Vec<u64> = Vec::new();
                (0..max_len).rev().for_each(|length| {
                    let mut numbers = numbers.clone();
                    let mut column_chars: Vec<char> = Vec::new();
                    while let Some(number) = numbers.pop() {
                        let digit = number.chars().nth(length).unwrap_or_else(|| {
                            panic!("No character! This should not be reachable! 😡")
                        });
                        if digit.is_whitespace() {
                            continue;
                        }
                        column_chars.push(digit);
                    }
                    column_numbers.push(
                        column_chars
                            .iter()
                            .rev()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                    );
                });
                Problem {
                    numbers: column_numbers,
                    operation: *operation,
                }
            })
            .collect()
    }
}

struct Worksheet<'a> {
    grid: Vec<Vec<&'a str>>,
}

impl<'a> Worksheet<'a> {
    fn new<F>(input: &'a str, line_parser: F) -> Self
    where
        F: Fn(&'a str) -> Vec<&'a str>,
    {
        let lines = input.lines().collect::<Vec<&str>>();
        let grid: Vec<Vec<&str>> = lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| line_parser(line))
            .collect();
        Self { grid }
    }

    fn pivot_grid(&self) -> Vec<Vec<&str>> {
        let mut numbers: Vec<Vec<&str>> = Vec::new();
        for x in 0..self.grid[0].len() {
            let mut row = Vec::new();
            for y in 0..self.grid.len() {
                row.push(self.grid[y][x]);
            }
            numbers.push(row);
        }
        numbers
    }
}

fn solution_1(input: &str) -> u64 {
    let worksheet = Worksheet::new(input.trim(), |line| {
        line.trim().split_whitespace().collect()
    });
    let math_problems = Problem::map_problem_1(&worksheet);
    math_problems
        .iter()
        .map(|problem| {
            problem
                .numbers
                .iter()
                .copied()
                .reduce(problem.operation)
                .unwrap_or(0)
        })
        .sum()
}

fn solution_2(input: &str) -> u64 {
    let operations = input
        .lines()
        .last()
        .unwrap_or_else(|| panic!("Invalid input!"));
    let o_indices = operations
        .char_indices()
        .filter(|(_, c)| !c.is_whitespace())
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();
    let worksheet = Worksheet::new(input, |row| {
        let mut o_indices = o_indices.clone();
        o_indices.push(row.len() + 1);
        let mut digits: Vec<&str> = Vec::new();
        while let Some(split_at) = o_indices.pop()
            && split_at > 0
        {
            let next_op = o_indices.last().unwrap_or(&0);
            let digit_string = &row[*next_op..split_at - 1];
            digits.push(digit_string);
        }
        digits
    });
    let math_problems = Problem::map_problem_2(&worksheet);
    math_problems
        .iter()
        .map(|problem| {
            problem
                .numbers
                .iter()
                .copied()
                .reduce(problem.operation)
                .unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_1() {
        let input = "
        123 328  51 64 
        45 64  387 23 
        6 98  215 314
        *   +   *   + 
        ";
        assert_eq!(solution_1(input), 4277556);
    }

    #[test]
    fn test_solution_2() {
        let input = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        assert_eq!(solution_2(input), 3263827);
    }
}
