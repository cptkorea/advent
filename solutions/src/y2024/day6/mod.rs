use crate::{AdventError, AdventProblem};
use std::collections::HashSet;

pub struct Day6;

impl AdventProblem for Day6 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let start = find_start(&grid);
        let visited = find_visited(&grid, start);
        Ok(visited.len() as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let start = find_start(&grid);
        let mut visited = find_visited(&grid, start);
        visited.remove(&start);

        let obstacles = find_obstacles(&mut grid, start, &visited);
        Ok(obstacles.len() as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&self) -> Self {
        match *self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let (m, n) = (grid.len(), grid[0].len());

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '^' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn find_visited(grid: &Vec<Vec<char>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let (mut curr, mut curr_dir) = (start, Direction::North);
    visited.insert(curr);

    while let Some((row, col, direction)) = transition(grid, curr, curr_dir) {
        curr = (row, col);
        curr_dir = direction;
        visited.insert(curr);
    }
    visited
}

fn find_obstacles(
    grid: &mut Vec<Vec<char>>,
    start: (usize, usize),
    visited: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut obstacles = HashSet::new();
    for &(row, col) in visited {
        grid[row][col] = '#';
        if try_traversal(grid, start) {
            obstacles.insert((row, col));
        }
        grid[row][col] = '.';
    }
    obstacles
}

fn try_traversal(grid: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
    let mut visited = HashSet::new();
    let (mut curr, mut curr_dir) = (start, Direction::North);
    visited.insert((curr, Direction::North));

    while let Some((row, col, direction)) = transition(grid, curr, curr_dir) {
        curr = (row, col);
        curr_dir = direction;
        if visited.contains(&(curr, curr_dir)) {
            return true;
        }
        visited.insert((curr, curr_dir));
    }
    false
}

fn transition(
    grid: &Vec<Vec<char>>,
    curr: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize, Direction)> {
    let (m, n) = (grid.len(), grid[0].len());
    let (row, col) = curr;
    let (next_row, next_col) = match direction {
        Direction::North => {
            if row == 0 {
                return None;
            }
            (row - 1, col)
        }
        Direction::South => {
            if row == m - 1 {
                return None;
            }
            (row + 1, col)
        }
        Direction::East => {
            if col == n - 1 {
                return None;
            }
            (row, col + 1)
        }
        Direction::West => {
            if col == 0 {
                return None;
            }
            (row, col - 1)
        }
    };

    if grid[next_row][next_col] == '#' {
        return Some((row, col, direction.rotate()));
    } else {
        return Some((next_row, next_col, direction));
    }
}
