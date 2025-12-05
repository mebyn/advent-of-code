use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("day03/input.txt").expect("Failed to read input file");

    println!("day03: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

fn find_largest_digit(numbers: &str, battery_size: u16) -> u64 {
    let chars = numbers.trim().chars().collect::<Vec<char>>();
    let mut battery_cells: Vec<char> = Vec::with_capacity(battery_size as usize);
    for _ in 0..battery_size {
        let mut skip_index = 0;
        let mut digit = 9;
        while digit >= 1 {
            let digit_char = digit.to_string().chars().next().unwrap();
            let digit_option = chars
                .iter()
                .skip(skip_index)
                .find_position(|c| digit_char == **c);
            digit -= 1;
            if digit_option.is_some() {
                let digit_index = skip_index + digit_option.unwrap().0;
                let remaining_char_count = chars.len() - digit_index;
                if remaining_char_count < battery_size as usize - battery_cells.len() {
                    continue;
                }
                battery_cells.push(digit_char);
                skip_index = digit_index + 1;
                digit = 9;
            }
            if battery_cells.len() == battery_size as usize {
                return battery_cells
                    .iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
            }
        }
    }
    0
}

fn solution(input: &str) -> u64 {
    const BATTERY_SIZE: u16 = 12;
    input
        .trim()
        .split("\n")
        .map(|s| find_largest_digit(s, BATTERY_SIZE))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_highest_number_with_battery_size_3() {
        assert_eq!(find_largest_digit("811111111111191", 3), 891);
        assert_eq!(find_largest_digit("234234234234278", 3), 478);
        assert_eq!(find_largest_digit("818181911112111", 3), 921);
    }

    #[test]
    fn test_should_return_highest_number_with_battery_size_2() {
        assert_eq!(find_largest_digit("987654321111111", 2), 98);
        assert_eq!(find_largest_digit("811111111111119", 2), 89);
        assert_eq!(find_largest_digit("234234234234278", 2), 78);
        assert_eq!(find_largest_digit("818181911112111", 2), 92);
    }

    #[test]
    fn test_solution_sample_input() {
        let input = "
        987654321111111
        811111111111119
        234234234234278
        818181911112111
        ";
        assert_eq!(solution(input), 3121910778619);
    }

    #[test]
    fn test_largest_digit() {
        let input = "3465793544554539453556366463344563446545344434374421565553674754454364545353445746344674866324626454";
        assert_eq!(solution(input), 99);
    }
}
