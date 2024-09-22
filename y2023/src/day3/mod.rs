use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day3;

impl AdventProblem for Day3 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid: Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
        let num_cols = grid[0].len();
        let adj_coords = compute_adj_coords(&grid);

        let mut total = 0;
        for (row, chars) in grid.iter().enumerate() {
            let mut i = 0;
            while i < chars.len() {
                let (mut num, mut j) = (0, i);
                while j < num_cols {
                    if let Some(d) = chars[j].to_digit(10) {
                        num = 10 * num + d;
                        j += 1;
                    } else {
                        break;
                    }
                }

                if num > 0 {
                    for col in i..j {
                        if adj_coords.contains_key(&Coordinate { row, col }) {
                            total += num;
                            break;
                        }
                    }
                }
                i = j + 1;
            }
        }

        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let grid: Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
        let num_cols = grid[0].len();
        let adj_coords = compute_adj_coords(&grid);
        let mut gears: HashMap<Coordinate, Vec<u32>> = HashMap::new();

        for (row, chars) in grid.iter().enumerate() {
            let mut i = 0;
            while i < chars.len() {
                let (mut num, mut j) = (0, i);
                while j < num_cols {
                    if let Some(d) = chars[j].to_digit(10) {
                        num = 10 * num + d;
                        j += 1;
                    } else {
                        break;
                    }
                }

                if num > 0 {
                    for col in i..j {
                        if let Some(symbol) = adj_coords.get(&Coordinate { row, col }) {
                            if symbol.value == '*' {
                                let adj_nums = gears.entry(symbol.location).or_default();
                                if !adj_nums.contains(&num) {
                                    adj_nums.push(num);
                                }
                            }
                        }
                    }
                }
                i = j + 1;
            }
        }

        let total: u32 = gears
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v[0] * v[1])
            .sum();

        Ok(total)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    location: Coordinate,
    value: char,
}

fn compute_adj_coords(grid: &Vec<Vec<char>>) -> HashMap<Coordinate, Symbol> {
    let mut coordinates = HashMap::new();
    for (row, chars) in grid.iter().enumerate() {
        for (col, ch) in chars.iter().enumerate() {
            if is_symbol(*ch) {
                let symbol = Symbol {
                    location: Coordinate { row, col },
                    value: *ch,
                };
                for adj_row in (row - 1)..=(row + 1) {
                    for adj_col in (col - 1)..=(col + 1) {
                        coordinates
                            .entry(Coordinate {
                                row: adj_row,
                                col: adj_col,
                            })
                            .or_insert(symbol);
                    }
                }
            }
        }
    }
    coordinates
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}
