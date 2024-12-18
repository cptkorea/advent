use std::collections::VecDeque;

pub struct Grid {
    pub spaces: Vec<Vec<Space>>,
    pub robot: (usize, usize),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Space {
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
    Wall,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::BoxLeft,
            '@' => Self::Robot,
            '.' => Self::Empty,
            _ => unimplemented!("unknown space type"),
        }
    }
}

impl Space {
    pub fn as_char(&self) -> char {
        match self {
            Self::Wall => '#',
            Self::Robot => '@',
            Self::Empty => '.',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    pub fn move_robot_1(&mut self, direction: Direction) {
        let space = self.robot;
        if let Some((nr, nc)) = self.next_space(space, direction) {
            match self.spaces[nr][nc] {
                Space::Empty => {
                    self.spaces[space.0][space.1] = Space::Empty;
                    self.spaces[nr][nc] = Space::Robot;
                    self.robot = (nr, nc);
                }
                Space::BoxLeft => {
                    let (mut r, mut c) = (nr, nc);
                    while let Some((ar, ac)) = self.next_space((r, c), direction) {
                        (r, c) = (ar, ac);
                        if self.spaces[ar][ac] == Space::BoxLeft {
                            continue;
                        } else {
                            break;
                        }
                    }

                    if self.spaces[r][c] == Space::Empty {
                        self.spaces[space.0][space.1] = Space::Empty;
                        self.spaces[nr][nc] = Space::Robot;
                        self.spaces[r][c] = Space::BoxLeft;
                        self.robot = (nr, nc);
                    }
                }
                Space::BoxRight => {
                    unimplemented!("single grid, box_right not implemented!");
                }
                Space::Robot => {
                    panic!("2nd robot detected on board");
                }
                Space::Wall => return,
            };
        }
    }

    pub fn move_robot_2(&mut self, direction: Direction) {
        let space = self.robot;
        if let Some((nr, nc)) = self.next_space(space, direction) {
            match self.spaces[nr][nc] {
                Space::Empty => {
                    self.spaces[space.0][space.1] = Space::Empty;
                    self.spaces[nr][nc] = Space::Robot;
                    self.robot = (nr, nc);
                }
                Space::BoxLeft => match direction {
                    Direction::Right => {
                        self.move_boxes_right((nr, nc));
                    }
                    Direction::Up => {
                        self.move_boxes_up((nr, nc));
                    }
                    Direction::Down => {
                        self.move_boxes_down((nr, nc));
                    }
                    _ => panic!("inconsistent state, visiting box-left from left"),
                },
                Space::BoxRight => match direction {
                    Direction::Left => {
                        self.move_boxes_left((nr, nc));
                    }
                    Direction::Up => {
                        self.move_boxes_up((nr, nc));
                    }
                    Direction::Down => {
                        self.move_boxes_down((nr, nc));
                    }
                    _ => panic!("inconsistent state, visiting box-right from right"),
                },
                Space::Robot => {
                    panic!("2nd robot detected on board");
                }
                Space::Wall => return,
            };
        }
    }

    fn move_boxes_right(&mut self, space: (usize, usize)) {
        let (mut r, mut c) = space;
        while let Some((ar, ac)) = self.next_space((r, c), Direction::Right) {
            (r, c) = (ar, ac);
            if self.spaces[ar][ac] == Space::BoxRight || self.spaces[ar][ac] == Space::BoxLeft {
                continue;
            } else {
                break;
            }
        }

        if self.spaces[r][c] == Space::Empty {
            for (i, col) in (space.1 + 1..=c).rev().enumerate() {
                if i % 2 == 0 {
                    self.spaces[r][col] = Space::BoxRight;
                } else {
                    self.spaces[r][col] = Space::BoxLeft;
                }
            }

            let (robot_r, robot_c) = self.robot;
            self.spaces[robot_r][robot_c] = Space::Empty;
            self.spaces[robot_r][robot_c + 1] = Space::Robot;
            self.robot = (robot_r, robot_c + 1);
        }
    }

    fn move_boxes_left(&mut self, space: (usize, usize)) {
        let (mut r, mut c) = space;
        while let Some((ar, ac)) = self.next_space((r, c), Direction::Left) {
            (r, c) = (ar, ac);
            if self.spaces[ar][ac] == Space::BoxRight || self.spaces[ar][ac] == Space::BoxLeft {
                continue;
            } else {
                break;
            }
        }

        if self.spaces[r][c] == Space::Empty {
            for (i, col) in (c..space.1).enumerate() {
                if i % 2 == 0 {
                    self.spaces[r][col] = Space::BoxLeft;
                } else {
                    self.spaces[r][col] = Space::BoxRight;
                }
            }

            let (robot_r, robot_c) = self.robot;
            self.spaces[robot_r][robot_c] = Space::Empty;
            self.spaces[robot_r][robot_c - 1] = Space::Robot;
            self.robot = (robot_r, robot_c - 1);
        }
    }

    fn move_boxes_up(&mut self, (r, c): (usize, usize)) {
        let mut boxes_to_move = VecDeque::new();
        let mut queue = VecDeque::new();
        queue.push_back((r, c));
        boxes_to_move.push_back((r, c));

        if self.spaces[r][c] == Space::BoxLeft {
            queue.push_back((r, c + 1));
            boxes_to_move.push_back((r, c + 1));
        } else if self.spaces[r][c] == Space::BoxRight {
            queue.push_back((r, c - 1));
            boxes_to_move.push_back((r, c - 1));
        }

        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (r, c) = queue.pop_front().unwrap();
                if let Some((nr, nc)) = self.next_space((r, c), Direction::Up) {
                    if self.spaces[nr][nc] == Space::Wall {
                        return;
                    } else if self.spaces[nr][nc] == Space::BoxLeft {
                        if !boxes_to_move.contains(&(nr, nc)) {
                            queue.push_back((nr, nc));
                            boxes_to_move.push_back((nr, nc));
                        }
                        if !boxes_to_move.contains(&(nr, nc + 1)) {
                            queue.push_back((nr, nc + 1));
                            boxes_to_move.push_back((nr, nc + 1));
                        }
                    } else if self.spaces[nr][nc] == Space::BoxRight {
                        if !boxes_to_move.contains(&(nr, nc)) {
                            queue.push_back((nr, nc));
                            boxes_to_move.push_back((nr, nc));
                        }
                        if !boxes_to_move.contains(&(nr, nc - 1)) {
                            queue.push_back((nr, nc - 1));
                            boxes_to_move.push_back((nr, nc - 1));
                        }
                    }
                }
            }
        }

        while let Some((r, c)) = boxes_to_move.pop_back() {
            if self.spaces[r - 1][c] != Space::Empty {
                panic!("trying to push box into non-empty space");
            }
            self.spaces[r - 1][c] = self.spaces[r][c];
            self.spaces[r][c] = Space::Empty;
        }
        let (robot_r, robot_c) = self.robot;
        self.spaces[robot_r][robot_c] = Space::Empty;
        self.spaces[robot_r - 1][robot_c] = Space::Robot;
        self.robot = (robot_r - 1, robot_c);
    }

    fn move_boxes_down(&mut self, (r, c): (usize, usize)) {
        let mut boxes_to_move = VecDeque::new();
        let mut queue = VecDeque::new();

        queue.push_back((r, c));
        boxes_to_move.push_back((r, c));

        if self.spaces[r][c] == Space::BoxLeft {
            queue.push_back((r, c + 1));
            boxes_to_move.push_back((r, c + 1));
        } else if self.spaces[r][c] == Space::BoxRight {
            queue.push_back((r, c - 1));
            boxes_to_move.push_back((r, c - 1));
        }

        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (r, c) = queue.pop_front().unwrap();
                if let Some((nr, nc)) = self.next_space((r, c), Direction::Down) {
                    if self.spaces[nr][nc] == Space::Wall {
                        return;
                    } else if self.spaces[nr][nc] == Space::BoxLeft {
                        if !boxes_to_move.contains(&(nr, nc)) {
                            queue.push_back((nr, nc));
                            boxes_to_move.push_back((nr, nc));
                        }
                        if !boxes_to_move.contains(&(nr, nc + 1)) {
                            queue.push_back((nr, nc + 1));
                            boxes_to_move.push_back((nr, nc + 1));
                        }
                    } else if self.spaces[nr][nc] == Space::BoxRight {
                        if !boxes_to_move.contains(&(nr, nc)) {
                            queue.push_back((nr, nc));
                            boxes_to_move.push_back((nr, nc));
                        }
                        if !boxes_to_move.contains(&(nr, nc - 1)) {
                            queue.push_back((nr, nc - 1));
                            boxes_to_move.push_back((nr, nc - 1));
                        }
                    }
                }
            }
        }

        while let Some((r, c)) = boxes_to_move.pop_back() {
            if self.spaces[r + 1][c] != Space::Empty {
                panic!("trying to push box into non-empty space");
            }
            self.spaces[r + 1][c] = self.spaces[r][c];
            self.spaces[r][c] = Space::Empty;
        }
        let (robot_r, robot_c) = self.robot;
        self.spaces[robot_r][robot_c] = Space::Empty;
        self.spaces[robot_r + 1][robot_c] = Space::Robot;
        self.robot = (robot_r + 1, robot_c);
    }

    fn boundary(&self) -> (usize, usize) {
        (self.spaces.len(), self.spaces[0].len())
    }

    fn next_space(&self, space: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let (row, col) = space;
        let (m, n) = self.boundary();
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
}
