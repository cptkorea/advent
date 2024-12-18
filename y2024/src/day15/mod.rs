use crate::{AdventError, AdventProblem};
use grid::{Direction, Grid, Space};

mod grid;

pub struct Day15;

impl AdventProblem for Day15 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (mut grid, i) = parse_grid(&lines);
        let directions = parse_directions(&lines, i + 1);

        for dir in directions {
            grid.move_robot_1(dir);
        }

        let mut total = 0;
        for (i, row) in grid.spaces.iter().enumerate() {
            let mut line = String::new();
            for (j, space) in row.iter().enumerate() {
                line.push(space.as_char());
                if *space == Space::BoxLeft {
                    total += 100 * i + j;
                }
            }
            println!("{}", line);
        }

        Ok(total as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (mut grid, i) = parse_wide_grid(&lines);
        let directions = parse_directions(&lines, i + 1);

        for dir in directions {
            grid.move_robot_2(dir);
        }

        let mut total = 0;
        for (i, row) in grid.spaces.iter().enumerate() {
            let mut line = String::new();
            for (j, space) in row.iter().enumerate() {
                line.push(space.as_char());
                if *space == Space::BoxLeft {
                    total += 100 * i + j;
                }
            }
            println!("{}", line);
        }

        Ok(total as u32)
    }
}

fn parse_grid<S: AsRef<str>>(lines: &Vec<S>) -> (Grid, usize) {
    let mut spaces = Vec::new();
    let (mut i, n) = (0, lines.len());
    let mut robot = (0, 0);

    while i < n {
        let line = lines[i].as_ref();
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

fn parse_wide_grid<S: AsRef<str>>(lines: &Vec<S>) -> (Grid, usize) {
    let mut spaces = Vec::new();
    let (mut i, n) = (0, lines.len());
    let mut robot = (0, 0);

    while i < n {
        let line = lines[i].as_ref();
        if line.is_empty() {
            break;
        }
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match Space::from(c) {
                Space::Empty => {
                    row.push(Space::Empty);
                    row.push(Space::Empty);
                }
                Space::BoxLeft => {
                    row.push(Space::BoxLeft);
                    row.push(Space::BoxRight);
                }
                Space::Robot => {
                    row.push(Space::Robot);
                    row.push(Space::Empty);
                    robot = (i, 2 * j);
                }
                Space::Wall => {
                    row.push(Space::Wall);
                    row.push(Space::Wall);
                }
                _ => panic!("inconsistent state"),
            }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_push_box_left() {
        let lines = vec![
            "#######", "#...#.#", "#.....#", "#..OO@#", "#..O..#", "#.....#", "#######",
        ];

        let (mut grid, _) = parse_wide_grid(&lines);
        grid.move_robot_2(Direction::Left);

        let expected = vec![
            "##############",
            "##......##..##",
            "##..........##",
            "##...[][]@..##",
            "##....[]....##",
            "##..........##",
            "##############",
        ];

        let mut actual = Vec::new();
        for row in grid.spaces.iter() {
            let mut line = String::new();
            for space in row {
                line.push(space.as_char());
            }
            actual.push(line);
        }

        for (i, &expect) in expected.iter().enumerate() {
            assert_eq!(expect, actual[i]);
        }
    }

    #[test]
    fn test_push_box_up() {
        let lines = vec![
            "#######", "#...#.#", "#.....#", "#..OO.#", "#..O..#", "#..@..#", "#######",
        ];

        let (mut grid, _) = parse_wide_grid(&lines);
        grid.move_robot_2(Direction::Up);

        let expected = vec![
            "##############",
            "##......##..##",
            "##....[]....##",
            "##....[][]..##",
            "##....@.....##",
            "##..........##",
            "##############",
        ];

        let mut actual = Vec::new();
        for row in grid.spaces.iter() {
            let mut line = String::new();
            for space in row {
                line.push(space.as_char());
            }
            actual.push(line);
        }

        for (i, &expect) in expected.iter().enumerate() {
            assert_eq!(expect, actual[i]);
        }
    }
}
