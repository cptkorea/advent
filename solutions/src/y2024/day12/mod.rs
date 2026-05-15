use crate::{AdventError, AdventProblem};
use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

pub struct Day12;

impl AdventProblem for Day12 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let plots = find_plots(grid, discover_plot);
        let total_price = plots.iter().map(|p| p.price()).sum();
        Ok(total_price)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let plots = find_plots(grid, discover_sides);
        let total_price = plots.iter().map(|p| p.price()).sum();
        Ok(total_price)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
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
        Direction::NorthWest => {
            if row == 0 || col == 0 {
                None
            } else {
                Some((row - 1, col - 1))
            }
        }
        Direction::SouthEast => {
            if row == m - 1 || col == n - 1 {
                None
            } else {
                Some((row + 1, col + 1))
            }
        }
        Direction::NorthEast => {
            if row == 0 || col == n - 1 {
                None
            } else {
                Some((row - 1, col + 1))
            }
        }
        Direction::SouthWest => {
            if row == m - 1 || col == 0 {
                None
            } else {
                Some((row + 1, col - 1))
            }
        }
    }
}

struct Plot {
    plant: char,
    area: u32,
    measure: u32,
}

impl Plot {
    fn price(&self) -> u32 {
        self.area * self.measure
    }
}

impl fmt::Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("plant", &self.plant)
            .field("area", &self.area)
            .field("measure", &self.measure)
            .finish()
    }
}

fn find_plots(
    mut grid: Vec<Vec<char>>,
    plot_fn: fn(&mut Vec<Vec<char>>, start: (usize, usize)) -> Plot,
) -> Vec<Plot> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut plots = Vec::new();

    for i in 0..m {
        for j in 0..n {
            if grid[i][j].is_alphabetic() {
                let plot = plot_fn(&mut grid, (i, j));
                plots.push(plot);
            }
        }
    }

    plots
}

fn discover_plot(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> Plot {
    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let plant = grid[start.0][start.1];
    let boundary = (grid.len(), grid[0].len());

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();

    stack.push_back(start);
    visited.insert(start);

    let mut area = 0;
    let mut perimeter = 0;

    while !stack.is_empty() {
        let space = stack.pop_front().unwrap();

        let mut num_neighbors = 0;
        for dir in dirs {
            if let Some((nr, nc)) = next_space(space, boundary, dir) {
                if grid[nr][nc] == plant {
                    num_neighbors += 1;
                    if !visited.contains(&(nr, nc)) {
                        visited.insert((nr, nc));
                        stack.push_back((nr, nc));
                    }
                }
            }
        }
        perimeter += 4 - num_neighbors;
        area += 1;
    }

    for (row, col) in visited {
        grid[row][col] = '.';
    }

    Plot {
        plant,
        area,
        measure: perimeter,
    }
}

fn discover_sides(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> Plot {
    let dirs = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let plant = grid[start.0][start.1];
    let boundary = (grid.len(), grid[0].len());

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();

    stack.push_back(start);
    visited.insert(start);

    let mut area = 0;
    let mut num_sides = 0;

    while !stack.is_empty() {
        let space = stack.pop_front().unwrap();

        for dir in dirs {
            if let Some((nr, nc)) = next_space(space, boundary, dir) {
                if grid[nr][nc] == plant {
                    if !visited.contains(&(nr, nc)) {
                        visited.insert((nr, nc));
                        stack.push_back((nr, nc));
                    }
                }
            }
        }

        num_sides += count_corners(grid, space, boundary, plant);
        area += 1;
    }

    for (row, col) in visited {
        grid[row][col] = '.';
    }

    Plot {
        plant,
        area,
        measure: num_sides,
    }
}

fn count_corners(
    grid: &Vec<Vec<char>>,
    space: (usize, usize),
    boundary: (usize, usize),
    plant: char,
) -> u32 {
    let mut num_corners = 0;

    let neighbors = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::NorthWest,
        Direction::SouthWest,
    ]
    .into_iter()
    .map(|d| {
        if let Some((nr, nc)) = next_space(space, boundary, d) {
            if grid[nr][nc] == plant {
                return true;
            }
        }
        false
    })
    .collect::<Vec<_>>();

    let corners = [(0, 2, 4), (1, 2, 5), (0, 3, 6), (1, 3, 7)];

    for (c1, c2, c3) in corners {
        if !neighbors[c1] && !neighbors[c2] {
            num_corners += 1;
        } else if neighbors[c1] && neighbors[c2] && !neighbors[c3] {
            num_corners += 1;
        }
    }

    num_corners
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_plot_perimeter() {
        let garden = ["AAAA", "BBCD", "BBCC", "EEEC"];
        let grid = garden
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let plots = find_plots(grid, discover_plot);
        let price = plots.iter().map(|p| p.price()).sum::<u32>();
        assert_eq!(140, price);
    }

    #[test]
    fn small_plot_corners() {
        let garden = ["AAAA", "BBCD", "BBCC", "EEEC"];
        let grid = garden
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let plots = find_plots(grid, discover_sides);
        let price = plots.iter().map(|p| p.price()).sum::<u32>();
        assert_eq!(80, price);
    }
}
