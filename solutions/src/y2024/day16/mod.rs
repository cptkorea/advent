use crate::{AdventError, AdventProblem};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    u32,
};

pub struct Day16;

impl AdventProblem for Day16 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let min_score = find_lowest_score(&grid);
        Ok(min_score)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let visited_paths = count_visited_paths(&grid);
        Ok(visited_paths)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    score: u32,
    position: (usize, usize),
    direction: Direction,
    path: Vec<(usize, usize)>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.score.cmp(&self.score)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_lowest_score(grid: &Vec<Vec<char>>) -> u32 {
    let boundary = (grid.len(), grid[0].len());
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..boundary.0 {
        for j in 0..boundary.1 {
            if grid[i][j] == 'S' {
                start = (i, j);
            } else if grid[i][j] == 'E' {
                end = (i, j);
            } else {
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {
        score: 0,
        position: start,
        direction: Direction::Right,
        path: vec![],
    });

    while !heap.is_empty() {
        let State {
            score,
            position,
            direction,
            path: _,
        } = heap.pop().unwrap();
        let (r, c) = position;
        visited.insert(((r, c), direction));

        if (r, c) == end {
            return score;
        }

        if let Some((nr, nc)) = next_space((r, c), boundary, direction) {
            if !visited.contains(&((nr, nc), direction)) && grid[nr][nc] != '#' {
                heap.push(State {
                    score: score + 1,
                    position: (nr, nc),
                    direction,
                    path: vec![],
                });
            }
        }

        let clockwise = direction.rotate_clockwise();
        if !visited.contains(&((r, c), clockwise)) && grid[r][c] != '#' {
            heap.push(State {
                score: score + 1000,
                position: (r, c),
                direction: clockwise,
                path: vec![],
            });
        }

        let counterclockwise = direction.rotate_counterclockwise();
        if !visited.contains(&((r, c), clockwise)) && grid[r][c] != '#' {
            heap.push(State {
                score: score + 1000,
                position: (r, c),
                direction: counterclockwise,
                path: vec![],
            });
        }
    }

    u32::MAX
}

fn count_visited_paths(grid: &Vec<Vec<char>>) -> u32 {
    let boundary = (grid.len(), grid[0].len());
    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..boundary.0 {
        for j in 0..boundary.1 {
            if grid[i][j] == 'S' {
                start = (i, j);
            } else if grid[i][j] == 'E' {
                end = (i, j);
            } else {
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    heap.push(State {
        score: 0,
        position: start,
        direction: Direction::Right,
        path: vec![(start)],
    });

    let mut all_paths = vec![];

    while !heap.is_empty() {
        let State {
            score,
            position,
            direction,
            path,
        } = heap.pop().unwrap();
        let (r, c) = position;
        visited.insert(((r, c), direction));

        if (r, c) == end {
            all_paths.push(path.clone());
        }

        if let Some((nr, nc)) = next_space((r, c), boundary, direction) {
            if !visited.contains(&((nr, nc), direction)) && grid[nr][nc] != '#' {
                let mut next_path = path.clone();
                next_path.push((nr, nc));
                heap.push(State {
                    score: score + 1,
                    position: (nr, nc),
                    direction,
                    path: next_path,
                });
            }
        }

        let clockwise = direction.rotate_clockwise();
        if !visited.contains(&((r, c), clockwise)) && grid[r][c] != '#' {
            heap.push(State {
                score: score + 1000,
                position: (r, c),
                direction: clockwise,
                path: path.clone(),
            });
        }

        let counterclockwise = direction.rotate_counterclockwise();
        if !visited.contains(&((r, c), counterclockwise)) && grid[r][c] != '#' {
            heap.push(State {
                score: score + 1000,
                position: (r, c),
                direction: counterclockwise,
                path: path.clone(),
            });
        }
    }

    let mut distinct_spaces = HashSet::new();
    for paths in all_paths {
        distinct_spaces.extend(paths);
    }
    distinct_spaces.len() as u32
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_clockwise(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn rotate_counterclockwise(&self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }
}

fn next_space(
    space: (usize, usize),
    boundary: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let (row, col) = space;
    let (m, n) = boundary;
    match direction {
        Direction::Up => {
            if row == 0 {
                None
            } else {
                Some((row - 1, col))
            }
        }
        Direction::Down => {
            if row >= m - 1 {
                None
            } else {
                Some((row + 1, col))
            }
        }
        Direction::Left => {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        }
        Direction::Right => {
            if col >= n - 1 {
                None
            } else {
                Some((row, col + 1))
            }
        }
    }
}
