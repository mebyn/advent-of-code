use std::{fs, time::Instant};

fn main() {
    let input = fs::read_to_string("day02/input.txt").expect("Failed to read input file");

    println!("Day 2: Advent of Code 2025");
    println!("=========================");

    let start = Instant::now();
    let result = solution(&input);
    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("Time: {:?}", elapsed);
}

struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn new(range: &str) -> Self {
        let (min, max) = range.split_once("-").unwrap();
        Self {
            min: min.parse::<i64>().unwrap(),
            max: max.parse::<i64>().unwrap(),
        }
    }
}

fn check_repeating_pattern(digit: &i64) -> bool {
    let digit_str = digit.to_string();
    let digit_bytes = digit_str.as_bytes();
    (1..digit_bytes.len())
        .filter(|&chunk_size| digit_bytes.len() % chunk_size == 0)
        .any(|chunk_size| {
            let first_chunk = &digit_bytes[..chunk_size];
            digit_bytes
                .chunks(chunk_size)
                .all(|chunk| first_chunk == chunk)
        })
}

fn solution(input: &str) -> i64 {
    input
        .trim()
        .split(",")
        .map(|range| Range::new(range))
        .flat_map(|range| range.min..=range.max)
        .filter(|digit| check_repeating_pattern(digit))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id_11_22() {
        assert_eq!(solution("11-22"), 33);
    }

    #[test]
    fn test_invalid_id_1188511880_1188511890() {
        assert_eq!(solution("1188511880-1188511890"), 1188511885);
    }

    #[test]
    fn test_invalid_id_95_115() {
        assert_eq!(solution("95-115"), 210);
    }

    #[test]
    fn test_invalid_id_123123123_123123123() {
        assert_eq!(solution("123123123-123123123"), 123123123);
    }

    #[test]
    fn test_sample_input() {
        assert_eq!(
            solution(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        );
    }
}
