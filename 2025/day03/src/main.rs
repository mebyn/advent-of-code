use std::fs;

fn main() {
    let input = fs::read_to_string("day03/input.txt").expect("Failed to read input file");

    println!("day03: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

struct Battery {
    cells: Vec<char>,
    size: usize,
}

impl Battery {
    fn new(size: usize) -> Self {
        Self {
            cells: Vec::with_capacity(size),
            size,
        }
    }

    fn is_full(&self) -> bool {
        self.cells.len() == self.size
    }

    fn remaining_capacity(&self) -> usize {
        self.size - self.cells.len()
    }

    fn total_joltage(&self) -> u64 {
        self.cells
            .iter()
            .fold(0u64, |acc, &c| acc * 10 + c.to_digit(10).unwrap() as u64)
    }

    fn fill_up_cells_from(&mut self, joltages: &[char]) {
        let mut remaining_joltages = joltages;
        while let Some((index, joltage)) = self.find_best_joltage(remaining_joltages) {
            self.cells.push(joltage);
            remaining_joltages = &remaining_joltages[index + 1..];
            if self.is_full() {
                break;
            }
        }
    }

    fn find_best_joltage(&self, joltages: &[char]) -> Option<(usize, char)> {
        for digit_char in ('1'..='9').rev() {
            if let Some(digit_index) = joltages.iter().position(|&c| c == digit_char) {
                let remaining_jolts = joltages.len() - digit_index;
                if remaining_jolts >= self.remaining_capacity() {
                    return Some((digit_index, digit_char));
                }
            }
        }
        None
    }
}

fn create_battery(input: &str, battery_size: usize) -> u64 {
    let joltages: Vec<char> = input.trim().chars().collect();
    let mut battery = Battery::new(battery_size);
    battery.fill_up_cells_from(&joltages);
    battery.total_joltage()
}

fn solution(input: &str) -> u64 {
    const BATTERY_SIZE: usize = 12;
    input
        .trim()
        .lines()
        .map(|s| create_battery(s, BATTERY_SIZE))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_highest_number_with_battery_size_3() {
        assert_eq!(create_battery("811111111111191", 3), 891);
        assert_eq!(create_battery("234234234234278", 3), 478);
        assert_eq!(create_battery("818181911112111", 3), 921);
    }

    #[test]
    fn test_should_return_highest_number_with_battery_size_2() {
        assert_eq!(create_battery("987654321111111", 2), 98);
        assert_eq!(create_battery("811111111111119", 2), 89);
        assert_eq!(create_battery("234234234234278", 2), 78);
        assert_eq!(create_battery("818181911112111", 2), 92);
    }

    #[test]
    fn test_should_return_highest_number_with_battery_size_4() {
        let input = "3465793544554539453556366463344563446545344434374421565553674754454364545353445746344674866324626454";
        assert_eq!(create_battery(input, 4), 9986);
    }

    #[test]
    fn test_should_return_highest_number_with_battery_size_12() {
        const BATTERY_SIZE: usize = 12;
        assert_eq!(
            create_battery("987654321111111", BATTERY_SIZE),
            987654321111
        );
        assert_eq!(
            create_battery("811111111111119", BATTERY_SIZE),
            811111111119
        );
        assert_eq!(
            create_battery("234234234234278", BATTERY_SIZE),
            434234234278
        );
        assert_eq!(
            create_battery("818181911112111", BATTERY_SIZE),
            888911112111
        );
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
}
