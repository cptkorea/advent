use std::collections::{HashMap, HashSet, VecDeque};

pub struct Grid {
    pub spaces: Vec<Vec<Space>>,
    pub robot: (usize, usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Space {
    Empty,
    Robot,
    Box,
    Wall,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            '.' => Self::Empty,
            _ => unimplemented!("unknown space type"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unimplemented!("unknown direction"),
        }
    }
}

impl Grid {
    pub fn move_robot(&mut self, direction: Direction) {
        let space = self.robot;
        if let Some((nr, nc)) = next_space(space, self.boundary(), direction) {
            match self.spaces[nr][nc] {
                Space::Empty => {
                    self.spaces[space.0][space.1] = Space::Empty;
                    self.robot = (nr, nc);
                }
                Space::Box => {
                    let (mut r, mut c) = (nr, nc);
                    while let Some((ar, ac)) = next_space((r, c), self.boundary(), direction) {
                        (r, c) = (ar, ac);
                        if self.spaces[ar][ac] == Space::Box {
                            continue;
                        } else {
                            break;
                        }
                    }

                    if self.spaces[r][c] == Space::Empty {
                        self.spaces[space.0][space.1] = Space::Empty;
                        self.spaces[nr][nc] = Space::Robot;
                        self.spaces[r][c] = Space::Box;
                        self.robot = (nr, nc);
                    }
                }
                Space::Robot => {
                    panic!("2nd robot detected on board");
                }
                Space::Wall => return,
            };
        }
    }

    fn boundary(&self) -> (usize, usize) {
        (self.spaces.len(), self.spaces[0].len())
    }
}

pub fn next_space(
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
