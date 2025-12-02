use std::fs;

const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

fn main() {
    let input = fs::read_to_string("day01/input.txt").expect("Failed to read input file");

    println!("Day 1: Advent of Code 2025");
    println!("=========================");
    let result = solution(&input, START_POS);
    println!("Result: {}", result);
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    distance: i32,
}

impl Rotation {
    fn parse(s: &str) -> Option<Self> {
        let direction = Direction::from_char(s.chars().next()?)?;
        let distance = s.get(1..)?.parse::<i32>().ok()?;
        Some(Rotation {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
struct DialState {
    position: i32,
    rotations: i32,
}

impl DialState {
    fn new(start_pos: i32) -> Self {
        Self {
            position: start_pos,
            rotations: 0,
        }
    }

    fn apply_rotation(&mut self, rotation: Rotation) {
        let new_pos = match rotation.direction {
            Direction::Left => self.position - rotation.distance,
            Direction::Right => self.position + rotation.distance,
        };

        let full_rotations = self.calculate_full_rotations(rotation, new_pos);
        self.rotations += full_rotations;
        self.position = new_pos.rem_euclid(DIAL_SIZE);

        if rotation.direction == Direction::Left && self.position == 0 && full_rotations == 0 {
            self.rotations += 1;
        }
    }

    fn calculate_full_rotations(&self, rotation: Rotation, new_position: i32) -> i32 {
        match rotation.direction {
            Direction::Left if new_position < 0 => {
                let offset = if self.position == 0 { 0 } else { DIAL_SIZE };
                (rotation.distance + offset - self.position) / DIAL_SIZE
            }
            Direction::Right if new_position >= DIAL_SIZE => new_position / DIAL_SIZE,
            _ => 0,
        }
    }
}

fn solution(input: &str, start_pos: i32) -> i32 {
    let mut state = DialState::new(start_pos);
    for rotation in input.trim().split_whitespace() {
        if let Some(parsed_rotation) = Rotation::parse(rotation) {
            state.apply_rotation(parsed_rotation);
        }
    }
    state.rotations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_rotation_right() {
        let input = "R10";
        assert_eq!(solution(input, START_POS), 0);
    }

    #[test]
    fn test_single_rotation_land_on_0() {
        let input = "L50";
        assert_eq!(solution(input, START_POS), 1);
    }

    #[test]
    fn test_overshoot_rotation() {
        let input = "L60";
        assert_eq!(solution(input, START_POS), 1);
    }

    #[test]
    fn test_overshoot_multiple_r_rotation() {
        let input = "R5500";
        assert_eq!(solution(input, START_POS), 55);
    }

    #[test]
    fn test_overshoot_multiple_l_rotation() {
        let input = "L200";
        assert_eq!(solution(input, START_POS), 2);
    }

    #[test]
    fn test_right_land_exactly_on_zero() {
        let input = "R50";
        assert_eq!(solution(input, START_POS), 1);
    }

    #[test]
    fn test_right_exactly_100() {
        let input = "R100";
        assert_eq!(solution(input, START_POS), 1);
    }

    #[test]
    fn test_from_position_52() {
        let input = "R48";
        assert_eq!(solution(input, 52), 1);
    }

    #[test]
    fn test_from_position_14_l_82() {
        let input = "L82";
        assert_eq!(solution(input, 14), 1);
    }

    #[test]
    fn test_start_at_0_go_right_small() {
        let input = "R1";
        assert_eq!(solution(input, 0), 0);
    }

    #[test]
    fn test_start_at_0_go_left_small() {
        let input = "L1";
        assert_eq!(solution(input, 0), 0);
    }

    #[test]
    fn test_start_at_0_go_right_100() {
        let input = "R100";
        assert_eq!(solution(input, 0), 1);
    }

    #[test]
    fn test_start_at_0_go_left_100() {
        let input = "L100";
        assert_eq!(solution(input, 0), 1);
    }

    #[test]
    fn test_from_99_right_1() {
        let input = "R1";
        assert_eq!(solution(input, 99), 1);
    }

    #[test]
    fn test_from_99_right_2() {
        let input = "R2";
        assert_eq!(solution(input, 99), 1);
    }

    #[test]
    fn test_from_99_right_101() {
        let input = "R101";
        assert_eq!(solution(input, 99), 2);
    }

    #[test]
    fn test_land_on_0_from_various_positions() {
        assert_eq!(solution("R50", 50), 1);
        assert_eq!(solution("L50", 50), 1);
    }

    #[test]
    fn test_distance_1() {
        assert_eq!(solution("R1", 50), 0);
        assert_eq!(solution("L1", 50), 0);
    }

    #[test]
    fn test_two_steps_both_land_on_zero() {
        let input = "L50 R100";
        assert_eq!(solution(input, 50), 2);
    }

    #[test]
    fn test_forward_backward() {
        let input = "R50 L50";
        assert_eq!(solution(input, 50), 1);
    }

    #[test]
    fn test_three_full_rotations() {
        let input = "R300";
        assert_eq!(solution(input, 50), 3);
    }

    #[test]
    fn test_from_1_left_1() {
        let input = "L1";
        assert_eq!(solution(input, 1), 1);
    }

    #[test]
    fn test_from_1_left_2() {
        let input = "L2";
        assert_eq!(solution(input, 1), 1);
    }

    #[test]
    fn test_from_1_left_101() {
        let input = "L101";
        assert_eq!(solution(input, 1), 2);
    }

    #[test]
    fn test_right_exactly_to_0() {
        let input = "R70";
        assert_eq!(
            solution(input, 30),
            1,
            "RIGHT landing on 0 must be counted!"
        );
    }

    #[test]
    fn test_sequence_with_right_landing() {
        let input = "R50 L100";
        assert_eq!(solution(input, 50), 2);
    }

    #[test]
    fn test_left_150_from_50() {
        let input = "L150";
        assert_eq!(solution(input, 50), 2);
    }

    #[test]
    fn test_left_201_from_1() {
        let input = "L201";
        assert_eq!(solution(input, 1), 3);
    }

    #[test]
    fn test_left_199_from_99() {
        let input = "L199";
        assert_eq!(solution(input, 99), 2);
    }

    #[test]
    fn test_left_multiple_full_rotations_from_zero() {
        let input = "L200";
        assert_eq!(solution(input, 0), 2);
    }

    #[test]
    fn test_left_multiple_full_rotations_from_small_pos() {
        let input = "L210";
        assert_eq!(solution(input, 10), 3);
    }

    #[test]
    fn test_left_small_no_wrap() {
        let input = "L10";
        assert_eq!(solution(input, 50), 0);
    }

    #[test]
    fn test_sample_input() {
        let input = "L68 L30 R48 L5 R60 L55 L1 L99 R14 L82";
        let result = solution(input, START_POS);
        let expected = 6;
        assert_eq!(
            result, expected,
            "Result should be {}, but was {}",
            expected, result
        );
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(solution("", START_POS), 0);
    }

    #[test]
    fn test_whitespace_only_input() {
        assert_eq!(solution("   \n\t  ", START_POS), 0);
    }

    #[test]
    fn test_invalid_direction_is_ignored() {
        assert_eq!(solution("X10", START_POS), 0);
    }

    #[test]
    fn test_zero_distance_no_cross() {
        assert_eq!(solution("R0", START_POS), 0);
        assert_eq!(solution("L0", START_POS), 0);
    }

    #[test]
    fn test_mixed_valid_and_invalid_tokens() {
        let input = "R30 X5 L20";
        assert_eq!(solution(input, START_POS), 0);
    }

    #[test]
    fn test_newline_separated_tokens() {
        let input = "R70\nR40";
        assert_eq!(solution(input, START_POS), 1);
    }

    #[test]
    fn test_exact_multiple_rotations_from_zero() {
        assert_eq!(solution("R200", 0), 2);
        assert_eq!(solution("R300", 0), 3);
    }

    #[test]
    fn test_zig_zag_crossing() {
        let input = "L2 R2";
        assert_eq!(solution(input, 1), 2);
    }

    #[test]
    fn test_alternating_around_zero() {
        let input = "R2 L2";
        assert_eq!(solution(input, 99), 2);
    }
}
