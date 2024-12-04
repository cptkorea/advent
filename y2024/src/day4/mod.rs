use std::collections::{HashSet, VecDeque};

use crate::{AdventError, AdventProblem};

pub struct Day4;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
            Self::NorthWest => (-1, -1),
            Self::NorthEast => (-1, 1),
            Self::SouthWest => (1, -1),
            Self::SouthEast => (1, 1),
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

        Ok(count_cross_mas(grid))
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

/**
 * M
 *  A
 *   S
 *
 */
fn count_cross_mas(grid: Vec<Vec<char>>) -> u32 {
    let end_points = find_mas(grid);
    let mut pairs = HashSet::new();

    for &(r, c, direction) in end_points.iter() {
        match direction {
            /*
             *   S   *
             *     A
             *   *   M
             */
            Direction::NorthWest => {
                if end_points.contains(&(r, c + 2, Direction::NorthEast)) {
                    pairs.insert((r + 1, c + 1));
                }
                if end_points.contains(&(r + 2, c, Direction::SouthWest)) {
                    pairs.insert((r + 1, c + 1));
                }
            }
            /*
             *   *   S
             *     A
             *   M   *
             */
            Direction::NorthEast => {
                if end_points.contains(&(r + 2, c, Direction::SouthEast)) {
                    pairs.insert((r + 1, c - 1));
                }
                if c >= 2 && end_points.contains(&(r, c - 2, Direction::NorthWest)) {
                    pairs.insert((r + 1, c - 1));
                }
            }
            /*
             *   M   *
             *     A
             *   *   S
             */
            Direction::SouthEast => {
                if c >= 2 && end_points.contains(&(r, c - 2, Direction::SouthWest)) {
                    pairs.insert((r - 1, c - 1));
                }
                if r >= 2 && end_points.contains(&(r - 2, c, Direction::NorthEast)) {
                    pairs.insert((r - 1, c - 1));
                }
            }
            /*
             *   *   M
             *     A
             *   S   *
             */
            Direction::SouthWest => {
                if end_points.contains(&(r, c + 2, Direction::SouthEast)) {
                    pairs.insert((r - 1, c + 1));
                }
                if r >= 2 && end_points.contains(&(r - 2, c, Direction::NorthWest)) {
                    pairs.insert((r - 1, c + 1));
                }
            }
            _ => println!("unknown direction found"),
        }
    }

    pairs.len() as u32
}

fn find_mas(grid: Vec<Vec<char>>) -> HashSet<(usize, usize, Direction)> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut queue = VecDeque::new();
    let mut end_points = HashSet::new();
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'M' {
                queue.push_back((i, j, Direction::NorthWest, 2));
                queue.push_back((i, j, Direction::NorthEast, 2));
                queue.push_back((i, j, Direction::SouthWest, 2));
                queue.push_back((i, j, Direction::SouthEast, 2));
            }
        }
    }

    while !queue.is_empty() {
        let (r, c, direction, offset) = queue.pop_front().unwrap();
        if offset == 4 {
            end_points.insert((r, c, direction));
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
    end_points
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn xmas_sample() {
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

    #[test]
    fn cross_mas_sample() {
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

        assert_eq!(9, count_cross_mas(grid));
    }
}
