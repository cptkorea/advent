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

        println!("antenna_coords={:?}", &antenna_coords);

        let mut all_antinode_coords: HashSet<(usize, usize)> = HashSet::new();

        for (c, coords) in antenna_coords.iter() {
            let antinode_coords = antinode_coords(coords, m, n);
            println!("c={}, antinode_coords={:?}", c, &antinode_coords);
            all_antinode_coords.extend(&antinode_coords);
        }

        Ok(all_antinode_coords.len() as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        Ok(0)
    }
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
            println!(
                "first={:?}, second={:?}, dr={}, dc={}",
                first, second, dr, dc
            );
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
                if first.0 >= dr && first.1 >= dc {
                    antinodes.insert((first.0 - dr, first.1 - dc));
                }
                if second.0 + dr < m && second.1 + dc < n {
                    antinodes.insert((second.0 + dr, second.1 + dc));
                }
            }
            // First is on the right, antinode will be right of first, left of second
            else {
                if first.0 >= dr && first.1 + dc < n {
                    antinodes.insert((first.0 - dr, first.1 + dc));
                }
                if second.0 + dr < m && second.1 >= dc {
                    antinodes.insert((second.0 + dr, second.1 - dc));
                }
            }
        }
    }
    antinodes
}

/// helper function to take absolute difference without overflowing
fn abs_diff(x: usize, y: usize) -> usize {
    if x >= y {
        x - y
    } else {
        y - x
    }
}
