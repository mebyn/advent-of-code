use std::fs;

fn main() {
    let input = fs::read_to_string("day04/input.txt").expect("Failed to read input file");

    println!("Day 4: Advent of Code 2025");
    println!("=========================");

    let result = solution(&input);
    println!("Result: {}", result);
}

#[derive(Debug, Clone)]
enum Neighbor {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Neighbor {
    fn offset(&self) -> (isize, isize) {
        match self {
            Self::TopLeft => (-1, -1),
            Self::Top => (0, -1),
            Self::TopRight => (1, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::BottomLeft => (-1, 1),
            Self::Bottom => (0, 1),
            Self::BottomRight => (1, 1),
        }
    }

    const ALL: [Self; 8] = [
        Self::TopLeft,
        Self::Top,
        Self::TopRight,
        Self::Left,
        Self::Right,
        Self::BottomLeft,
        Self::Bottom,
        Self::BottomRight,
    ];
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get_cell_neighbors(&self, x: usize, y: usize) -> Vec<char> {
        let mut adjacent_cells: Vec<char> = Vec::new();
        for neighbor in Neighbor::ALL {
            let (dx, dy) = neighbor.offset();
            let offset_x = x as isize + dx;
            let offset_y = y as isize + dy;
            if offset_x >= 0
                && offset_x < self.width as isize
                && offset_y >= 0
                && offset_y < self.height as isize
            {
                adjacent_cells.push(self.rows[offset_y as usize][offset_x as usize]);
            }
        }
        adjacent_cells
    }

    fn remove_rolls(&mut self) -> Option<u64> {
        let mut removed_rolls = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.rows[y][x];
                if cell != '@' {
                    continue;
                }
                let cell_neighbors = self.get_cell_neighbors(x, y);
                let paper_roll_neighbours_size =
                    cell_neighbors.iter().filter(|&c| *c == '@').count();
                if paper_roll_neighbours_size < 4 {
                    removed_rolls += 1;
                    self.rows[y][x] = '.';
                }
            }
        }
        (removed_rolls > 0).then_some(removed_rolls)
    }

    fn from_2d_string(input: &str) -> Self {
        let lines = input.trim().lines().collect::<Vec<&str>>();
        Self {
            rows: lines
                .iter()
                .map(|line| line.trim().chars().collect())
                .collect(),
            width: lines.first().unwrap().len(),
            height: lines.len(),
        }
    }
}

fn solution(input: &str) -> u64 {
    let mut grid = Grid::from_2d_string(input);
    let mut total_removed_rolls = 0u64;
    while let Some(removed_rolls) = grid.remove_rolls() {
        total_removed_rolls += removed_rolls;
    }
    total_removed_rolls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let input = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        ";
        assert_eq!(solution(input), 43);
    }
}
