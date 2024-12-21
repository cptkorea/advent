use crate::{AdventError, AdventProblem};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
};

pub struct Day20;

impl AdventProblem for Day20 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let shortest_distances = find_shortest_distances(&grid);
        let num_shortcuts = find_2ps_shortcuts(&grid, &shortest_distances);

        Ok(num_shortcuts)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let shortest_distances = find_shortest_distances(&grid);
        let num_shortcuts = find_20ps_shortcuts(&grid, &shortest_distances);

        Ok(num_shortcuts)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    row: usize,
    col: usize,
    distance: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.distance.cmp(&self.distance)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_distances(grid: &Vec<Vec<char>>) -> Vec<Vec<u32>> {
    let boundary = (grid.len(), grid[0].len());
    let mut end = (0, 0);

    for i in 0..boundary.0 {
        for j in 0..boundary.1 {
            if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }

    let mut shortest_distances: Vec<Vec<u32>> = Vec::new();
    for _ in 0..boundary.0 {
        let mut row = Vec::new();
        for _ in 0..boundary.1 {
            row.push(u32::MAX);
        }
        shortest_distances.push(row);
    }
    shortest_distances[end.0][end.1] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(State {
        distance: 0,
        row: end.0,
        col: end.1,
    });

    while !heap.is_empty() {
        let State { distance, row, col } = heap.pop().unwrap();
        for dir in [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ] {
            if let Some((nr, nc)) = next_space((row, col), boundary, 1, dir) {
                if grid[nr][nc] != '#' && distance + 1 < shortest_distances[nr][nc] {
                    shortest_distances[nr][nc] = distance + 1;
                    heap.push(State {
                        distance: distance + 1,
                        row: nr,
                        col: nc,
                    });
                }
            }
        }
    }

    shortest_distances
}

fn find_2ps_shortcuts(grid: &Vec<Vec<char>>, shortest_distances: &Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    let boundary = (grid.len(), grid[0].len());

    for i in 0..shortest_distances.len() {
        for j in 0..shortest_distances[i].len() {
            if grid[i][j] == '#' {
                continue;
            }

            for dir in [
                Direction::Up,
                Direction::Left,
                Direction::Right,
                Direction::Down,
            ] {
                if let Some((nr, nc)) = next_space((i, j), boundary, 2, dir) {
                    if grid[nr][nc] != '#'
                        && shortest_distances[nr][nc] + 2 < shortest_distances[i][j]
                    {
                        let shortcut = shortest_distances[i][j] - shortest_distances[nr][nc] - 2;
                        if shortcut >= 100 {
                            total += 1;
                        }
                    }
                }
            }
        }
    }

    total
}

fn find_20ps_shortcuts(grid: &Vec<Vec<char>>, shortest_distances: &Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    let boundary = (grid.len(), grid[0].len());

    for i in 0..shortest_distances.len() {
        for j in 0..shortest_distances[i].len() {
            if grid[i][j] == '#' {
                continue;
            }

            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back(((i, j), 0));
            visited.insert((i, j));

            while !queue.is_empty() {
                let ((r, c), dist) = queue.pop_front().unwrap();
                if dist > 20 {
                    continue;
                }

                if grid[r][c] != '#' && shortest_distances[r][c] + dist < shortest_distances[i][j] {
                    let shortcut = shortest_distances[i][j] - shortest_distances[r][c] - dist;
                    if shortcut >= 100 {
                        total += 1;
                    }
                }

                for dir in [
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                ] {
                    if let Some((nr, nc)) = next_space((r, c), boundary, 1, dir) {
                        if !visited.contains(&(nr, nc)) {
                            visited.insert((nr, nc));
                            queue.push_back(((nr, nc), dist + 1));
                        }
                    }
                }
            }
        }
    }

    total
}

fn next_space(
    space: (usize, usize),
    boundary: (usize, usize),
    offset: usize,
    direction: Direction,
) -> Option<(usize, usize)> {
    let (row, col) = space;
    let (m, n) = boundary;
    match direction {
        Direction::Up => {
            if row < offset {
                None
            } else {
                Some((row - offset, col))
            }
        }
        Direction::Down => {
            if row >= m - offset {
                None
            } else {
                Some((row + offset, col))
            }
        }
        Direction::Left => {
            if col < offset {
                None
            } else {
                Some((row, col - offset))
            }
        }
        Direction::Right => {
            if col >= n - offset {
                None
            } else {
                Some((row, col + offset))
            }
        }
    }
}
