use crate::{AdventError, AdventProblem};
use std::{cmp::Ordering, collections::BinaryHeap, u32};

pub struct Day18;

impl AdventProblem for Day18 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = parse_lines(&lines, 1024, (70, 70));
        let score = find_lowest_score(&grid, (70, 70));
        Ok(score)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut grid = parse_lines(&lines, 1024, (70, 70));

        for k in 1025..lines.len() {
            let mut split = lines[k].split(",");
            let j = split
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .expect("numeric value");
            let i = split
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .expect("numeric value");

            grid[i][j] = '#';

            let score = find_lowest_score(&grid, (70, 70));
            if score == u32::MAX {
                println!("no path possible k={}, space=({})", k, lines[k]);
                break;
            }
        }

        Ok(0)
    }
}

fn parse_lines(lines: &Vec<String>, limit: usize, boundary: (usize, usize)) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for _ in 0..=boundary.0 {
        let mut row = Vec::new();
        for _ in 0..=boundary.1 {
            row.push('.');
        }
        grid.push(row);
    }

    let limit = usize::min(lines.len(), limit);
    for i in 0..limit {
        let mut split = lines[i].split(",");
        let j = split
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .expect("numeric value");
        let i = split
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .expect("numeric value");

        grid[i][j] = '#';
    }

    grid
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    score: u32,
    position: (usize, usize),
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_lowest_score(grid: &Vec<Vec<char>>, end: (usize, usize)) -> u32 {
    let boundary = (end.0 + 1, end.1 + 1);
    let mut heap = BinaryHeap::new();
    heap.push(State {
        score: 0,
        position: (0, 0),
    });

    let mut min_scores: Vec<Vec<u32>> = Vec::new();
    for _ in 0..boundary.0 {
        let mut row = Vec::new();
        for _ in 0..boundary.1 {
            row.push(u32::MAX);
        }
        min_scores.push(row);
    }
    min_scores[0][0] = 0;

    while !heap.is_empty() {
        let State { score, position } = heap.pop().unwrap();
        let (r, c) = position;

        for dir in [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ] {
            if let Some((nr, nc)) = next_space((r, c), boundary, dir) {
                if grid[nr][nc] != '#' && score + 1 < min_scores[nr][nc] {
                    min_scores[nr][nc] = score + 1;

                    heap.push(State {
                        score: score + 1,
                        position: (nr, nc),
                    });
                }
            }
        }
    }

    min_scores[end.0][end.1]
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
