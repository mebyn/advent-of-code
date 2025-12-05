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

    fn from_2d_string(input: &str) -> Self {
        let lines = input.trim().lines().collect::<Vec<&str>>();
        Self {
            rows: lines
                .iter()
                .map(|line| line.trim().chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
            width: lines.first().unwrap().len(),
            height: lines.len(),
        }
    }
}

fn solution(input: &str) -> i64 {
    let grid = Grid::from_2d_string(input);
    let mut paper_roll_count = 0;
    for (y, row) in grid.rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != '@' {
                continue;
            }
            let cell_neighbors = grid.get_cell_neighbors(x, y);
            let paper_roll_neighbours_size = cell_neighbors.iter().filter(|&c| *c == '@').count();
            if paper_roll_neighbours_size < 4 {
                paper_roll_count += 1;
            }
        }
    }
    paper_roll_count
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
        assert_eq!(solution(input), 13);
    }
}
