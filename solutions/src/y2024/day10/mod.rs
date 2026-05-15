use crate::{AdventError, AdventProblem};
use std::collections::VecDeque;

pub struct Day10;

impl AdventProblem for Day10 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let trailheads = find_trailheads(&grid);
        let total = trailheads.iter().map(|t| t.score).sum();
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let trailheads = find_trailhead_ratings(&grid);
        let total = trailheads.iter().map(|t| t.score).sum();
        Ok(total)
    }
}

#[derive(Debug)]
struct Trailhead {
    start: (usize, usize),
    score: u32,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn next_space(
    space: (usize, usize),
    boundary: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let (row, col) = space;
    let (m, n) = boundary;
    match direction {
        Direction::North => {
            if row == 0 {
                None
            } else {
                Some((row - 1, col))
            }
        }
        Direction::South => {
            if row >= m - 1 {
                None
            } else {
                Some((row + 1, col))
            }
        }
        Direction::East => {
            if col >= n - 1 {
                None
            } else {
                Some((row, col + 1))
            }
        }
        Direction::West => {
            if col == 0 {
                None
            } else {
                Some((row, col - 1))
            }
        }
    }
}

fn find_trailheads(grid: &Vec<Vec<char>>) -> Vec<Trailhead> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut trailheads = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '0' {
                let score = compute_score(grid, (i, j), (m, n));
                trailheads.push(Trailhead {
                    start: (i, j),
                    score: score as u32,
                });
            }
        }
    }
    trailheads
}

fn find_trailhead_ratings(grid: &Vec<Vec<char>>) -> Vec<Trailhead> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut trailheads = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '0' {
                let rating = compute_rating(grid, (i, j), (m, n));
                trailheads.push(Trailhead {
                    start: (i, j),
                    score: rating as u32,
                });
            }
        }
    }
    trailheads
}

fn compute_score(grid: &Vec<Vec<char>>, start: (usize, usize), boundary: (usize, usize)) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut trailends = Vec::new();
    while !queue.is_empty() {
        let (space, height) = queue.pop_front().unwrap();
        for dir in dirs {
            if let Some((nr, nc)) = next_space(space, boundary, dir) {
                if let Some(d) = grid[nr][nc].to_digit(10) {
                    if d == height + 1 {
                        if d == 9 && !trailends.contains(&(nr, nc)) {
                            trailends.push((nr, nc));
                        } else {
                            queue.push_back(((nr, nc), d));
                        }
                    }
                }
            }
        }
    }
    trailends.len() as u32
}

fn compute_rating(grid: &Vec<Vec<char>>, start: (usize, usize), boundary: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    let mut rating = 0;
    while !queue.is_empty() {
        let (space, height) = queue.pop_front().unwrap();
        for dir in dirs {
            if let Some((nr, nc)) = next_space(space, boundary, dir) {
                if let Some(d) = grid[nr][nc].to_digit(10) {
                    if d == height + 1 {
                        if d == 9 {
                            rating += 1;
                        } else {
                            queue.push_back(((nr, nc), d));
                        }
                    }
                }
            }
        }
    }
    rating
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let lines = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
            "10456732",
        ];

        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let trailheads = find_trailheads(&grid);
        let total = trailheads.iter().map(|t| t.score).sum::<u32>();
        assert_eq!(36, total);

        let trailhead_ratings = find_trailhead_ratings(&grid);
        let total = trailhead_ratings.iter().map(|t| t.score).sum::<u32>();
        assert_eq!(81, total);
    }
}
