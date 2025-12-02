use std::fs;

fn main() {
    let input = fs::read_to_string("day10/input.txt").expect("Failed to read input file");

    println!("day10: Advent of Code 2025");
    println!("=========================");

    // Part 1
    let part1_result = solve_part1(&input);
    println!("Part 1: {}", part1_result);

    // Part 2
    let part2_result = solve_part2(&input);
    println!("Part 2: {}", part2_result);
}

fn solve_part1(input: &str) -> i32 {
    // TODO: Implement part 1 solution
    0
}

fn solve_part2(input: &str) -> i32 {
    // TODO: Implement part 2 solution
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ""; // Add test input
        assert_eq!(solve_part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = ""; // Add test input
        assert_eq!(solve_part2(input), 0);
    }
}
