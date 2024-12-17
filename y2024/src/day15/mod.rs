use crate::{AdventError, AdventProblem};
use grid::{Direction, Grid, Space};

mod grid;

pub struct Day15;

impl AdventProblem for Day15 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (mut grid, i) = parse_grid(&lines);
        let directions = parse_directions(&lines, i + 1);

        for dir in directions {
            grid.move_robot(dir);
        }

        let mut total = 0;
        for (i, row) in grid.spaces.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                if *space == Space::Box {
                    total += 100 * i + j;
                }
            }
        }

        Ok(total as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        Ok(0)
    }
}

fn parse_grid(lines: &Vec<String>) -> (Grid, usize) {
    let mut spaces = Vec::new();
    let (mut i, n) = (0, lines.len());
    let mut robot = (0, 0);

    while i < n {
        let line = &lines[i];
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let space = Space::from(c);
            if space == Space::Robot {
                robot = (i, j);
            }
            row.push(space);
        }
        spaces.push(row);
        i += 1;
    }

    (Grid { spaces, robot }, i)
}

fn parse_directions(lines: &Vec<String>, start: usize) -> Vec<Direction> {
    let mut i = start;
    let mut directions = Vec::new();

    while i < lines.len() {
        let line = &lines[i];
        for c in line.chars() {
            directions.push(Direction::from(c));
        }
        i += 1;
    }

    directions
}
