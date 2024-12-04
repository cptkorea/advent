use std::collections::VecDeque;

use crate::{AdventError, AdventProblem};

pub struct Day4;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    fn offset(self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, 1),
            Self::NorthEast => (1, 1),
            Self::SouthWest => (-1, -1),
            Self::SouthEast => (1, -1),
        }
    }

    fn transition(self, row: usize, col: usize) -> (i32, i32) {
        let (ro, co) = self.offset();
        (row as i32 + ro, col as i32 + co)
    }
}

impl AdventProblem for Day4 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        Ok(count_xmas(grid))
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        Ok(count_xmas(grid))
    }
}

fn count_xmas(grid: Vec<Vec<char>>) -> u32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut queue = VecDeque::new();
    let mut count = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'X' {
                queue.push_back((i, j, Direction::North, 1));
                queue.push_back((i, j, Direction::South, 1));
                queue.push_back((i, j, Direction::East, 1));
                queue.push_back((i, j, Direction::West, 1));
                queue.push_back((i, j, Direction::NorthWest, 1));
                queue.push_back((i, j, Direction::NorthEast, 1));
                queue.push_back((i, j, Direction::SouthWest, 1));
                queue.push_back((i, j, Direction::SouthEast, 1));
            }
        }
    }

    while !queue.is_empty() {
        let (r, c, direction, offset) = queue.pop_front().unwrap();
        if offset == 4 {
            count += 1;
            continue;
        }

        let (nr, nc) = direction.transition(r, c);
        if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if grid[nr][nc] == XMAS[offset] {
            queue.push_back((nr, nc, direction, offset + 1));
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let grid = input.iter().map(|s| s.chars().collect()).collect();

        assert_eq!(18, count_xmas(grid));
    }
}
