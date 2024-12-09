use crate::{AdventError, AdventProblem};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub struct Day8;

impl AdventProblem for Day8 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (m, n) = (grid.len(), grid[0].len());
        let antenna_coords = find_antennas(&grid);

        let mut all_antinode_coords: HashSet<(usize, usize)> = HashSet::new();
        for coords in antenna_coords.values() {
            let antinode_coords = antinode_coords(coords, m, n);
            all_antinode_coords.extend(&antinode_coords);
        }

        Ok(all_antinode_coords.len() as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let (m, n) = (grid.len(), grid[0].len());
        let antenna_coords = find_antennas(&grid);

        let mut all_antinode_coords: HashSet<(usize, usize)> = HashSet::new();
        for coords in antenna_coords.values() {
            let antinode_coords = antinode_coords_v2(coords, m, n);
            all_antinode_coords.extend(&antinode_coords);
        }

        Ok(all_antinode_coords.len() as u32)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

fn find_antennas(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            let c = grid[i][j];
            if c.is_ascii_alphabetic() || c.is_numeric() {
                antennas.entry(c).or_default().push((i, j));
            }
        }
    }

    for v in antennas.values_mut() {
        v.sort_by(|c1, c2| {
            if c1.0.cmp(&c2.0) != Ordering::Equal {
                c1.0.cmp(&c2.0)
            } else {
                c1.1.cmp(&c2.1)
            }
        });
    }

    antennas
}

/// counts the total number of antinodes given the coordinates of all antennas
/// m & n are the grid row & col boundaries
fn antinode_coords(coords: &[(usize, usize)], m: usize, n: usize) -> HashSet<(usize, usize)> {
    let num_antennas = coords.len();
    let mut antinodes = HashSet::new();

    for i in 0..num_antennas {
        let first = coords[i];
        for j in i + 1..num_antennas {
            let second = coords[j];
            let (dr, dc) = (abs_diff(first.0, second.0), abs_diff(first.1, second.1));
            /* Assuming coords are sorted by row, there are two possible orientations
             * First is left of second
             * a
             *        a
             * First is right of second
             *        a
             * a
             */
            // First antenna is on the left, antinode will be left of first, right of second
            if first.1 < second.1 {
                if let Some(coord) =
                    next_antinode(first, second, dr, dc, m, n, Direction::NorthWest)
                {
                    antinodes.insert(coord);
                }
                if let Some(coord) =
                    next_antinode(first, second, dr, dc, m, n, Direction::SouthEast)
                {
                    antinodes.insert(coord);
                }
            }
            // First is on the right, antinode will be right of first, left of second
            else {
                if let Some(coord) =
                    next_antinode(first, second, dr, dc, m, n, Direction::NorthEast)
                {
                    antinodes.insert(coord);
                }
                if let Some(coord) =
                    next_antinode(first, second, dr, dc, m, n, Direction::SouthWest)
                {
                    antinodes.insert(coord);
                }
            }
        }
    }
    antinodes
}

/// counts the total number of antinodes given the coordinates of all antennas
/// using new algorithm propagating the entire grid
/// m & n are the grid row & col boundaries
fn antinode_coords_v2(coords: &[(usize, usize)], m: usize, n: usize) -> HashSet<(usize, usize)> {
    let num_antennas = coords.len();
    let mut antinodes = HashSet::new();

    for i in 0..num_antennas {
        let first = coords[i];
        for j in i + 1..num_antennas {
            let second = coords[j];
            let (dr, dc) = (abs_diff(first.0, second.0), abs_diff(first.1, second.1));
            /* Assuming coords are sorted by row, there are two possible orientations
             * First is left of second
             * a
             *        a
             * First is right of second
             *        a
             * a
             */
            // First antenna is on the left, antinode will be left of first, right of second
            if first.1 < second.1 {
                let (mut left, mut right) = (first, second);
                antinodes.insert(left);
                antinodes.insert(right);

                while let Some(coord) =
                    next_antinode(left, right, dr, dc, m, n, Direction::NorthWest)
                {
                    antinodes.insert(coord);
                    right = left;
                    left = coord;
                }

                (left, right) = (first, second);
                while let Some(coord) =
                    next_antinode(left, right, dr, dc, m, n, Direction::SouthEast)
                {
                    antinodes.insert(coord);
                    left = right;
                    right = coord;
                }
            }
            // First is on the right, antinode will be right of first, left of second
            else {
                let (mut right, mut left) = (first, second);
                antinodes.insert(left);
                antinodes.insert(right);

                while let Some(coord) =
                    next_antinode(left, right, dr, dc, m, n, Direction::NorthEast)
                {
                    antinodes.insert(coord);
                    left = right;
                    right = coord;
                }

                (left, right) = (first, second);
                while let Some(coord) =
                    next_antinode(left, right, dr, dc, m, n, Direction::SouthWest)
                {
                    antinodes.insert(coord);
                    right = left;
                    left = coord;
                }
            }
        }
    }
    antinodes
}

/// Continue generating the next antinode if it is a valid square on the grid
fn next_antinode(
    first: (usize, usize),
    second: (usize, usize),
    dr: usize,
    dc: usize,
    m: usize,
    n: usize,
    direction: Direction,
) -> Option<(usize, usize)> {
    match direction {
        Direction::NorthWest => {
            if first.0 >= dr && first.1 >= dc {
                return Some((first.0 - dr, first.1 - dc));
            }
        }
        Direction::SouthEast => {
            if second.0 + dr < m && second.1 + dc < n {
                return Some((second.0 + dr, second.1 + dc));
            }
        }
        Direction::NorthEast => {
            if first.0 >= dr && first.1 + dc < n {
                return Some((first.0 - dr, first.1 + dc));
            }
        }
        Direction::SouthWest => {
            if second.0 + dr < m && second.1 >= dc {
                return Some((second.0 + dr, second.1 - dc));
            }
        }
    }
    None
}

/// helper function to take absolute difference without overflowing
fn abs_diff(x: usize, y: usize) -> usize {
    if x >= y {
        x - y
    } else {
        y - x
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_antinode() {
        let first = (4, 5);
        let second = (5, 7);

        assert_eq!(
            Some((3, 3)),
            next_antinode(first, second, 1, 2, 10, 10, Direction::NorthWest)
        );

        assert_eq!(
            Some((6, 9)),
            next_antinode(first, second, 1, 2, 10, 10, Direction::SouthEast)
        );

        let first = (4, 5);
        let second = (5, 2);

        assert_eq!(
            Some((3, 8)),
            next_antinode(first, second, 1, 3, 10, 10, Direction::NorthEast)
        );

        assert_eq!(
            None,
            next_antinode(first, second, 1, 3, 10, 10, Direction::SouthWest)
        );
    }
}
