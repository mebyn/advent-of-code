use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("day07/input.txt").expect("Failed to read input file");

    println!("Day 7: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

const BEAM_ENTRACE: char = 'S';

fn solution(input: &str) -> i64 {
    let mut lines = input.trim().lines().rev().collect::<Vec<&str>>();
    let mut beam_positions: Vec<Vec<usize>> = vec![vec![
        lines
            .last()
            .expect("Invalid input!")
            .chars()
            .position(|c| c == BEAM_ENTRACE)
            .expect("Entrace is missing!"),
    ]];
    let mut split_count = 0;
    while let Some(line) = lines.pop() {
        let splitters_positions = line
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        if splitters_positions.is_empty() {
            continue;
        }
        let last_positions = beam_positions
            .last()
            .expect("There should at least be an entrance!");

        let mut new_positions = Vec::new();
        for beam_pos in last_positions {
            if splitters_positions.contains(beam_pos) {
                split_count += 1;
                new_positions.push(beam_pos - 1);
                new_positions.push(beam_pos + 1);
            } else {
                new_positions.push(*beam_pos);
            }

        }
        new_positions.sort();
        new_positions.dedup();

        println!("{}", line);
        // println!("{:?}", new_positions);
        (0..line.len()).for_each(|i| {
            if new_positions.contains(&i) {
                print!("|");
            } else {
                print!(".");
            }
        });
        println!("  >> Split Count: {:?}", split_count);

        if new_positions.is_empty() {
            continue;
        }
        beam_positions.push(new_positions);
    }
    split_count
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
