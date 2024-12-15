use crate::{regex, AdventError, AdventProblem};
use std::collections::{HashMap, HashSet};

pub struct Day14;

impl AdventProblem for Day14 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| {
                let mut robot = parse_line(s);
                robot.move_spaces(100, GRID_WIDTH, GRID_HEIGHT);
                robot.quadrant(GRID_WIDTH, GRID_HEIGHT)
            })
            .fold(HashMap::<Quadrant, u32>::new(), |mut res, q| {
                if let Some(q) = q {
                    *res.entry(q).or_default() += 1;
                }
                res
            })
            .values()
            .product();

        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut robots = lines.iter().map(|s| parse_line(s)).collect::<Vec<_>>();

        for iter in 1..=10000 {
            let mut positions = HashSet::new();
            for robot in robots.iter_mut() {
                robot.move_spaces(1, GRID_WIDTH, GRID_HEIGHT);
                positions.insert(robot.pos);
            }

            let mut manhattan_dist = 0;
            for p1 in positions.iter() {
                for p2 in positions.iter() {
                    manhattan_dist += i64::abs(p1.0 - p2.0);
                    manhattan_dist += i64::abs(p1.1 - p2.1);
                }
            }
            println!("Iter={}, manhattan_dist={}", iter, manhattan_dist);
            if manhattan_dist < 10_000_000 {
                println!("Found christmas tree at iter iter={}", iter);
                break;
            }
        }

        let positions: HashSet<(i64, i64)> =
            robots
                .iter()
                .map(|r| r.pos)
                .fold(HashSet::new(), |mut positions, p| {
                    positions.insert(p);
                    positions
                });

        for i in 0..GRID_WIDTH {
            let mut line = String::new();
            for j in 0..GRID_HEIGHT {
                if positions.contains(&(i, j)) {
                    line.push_str("*");
                } else {
                    line.push_str(".");
                }
            }
            println!("{}", line);
        }

        Ok(0)
    }
}

fn parse_line(line: &str) -> Robot {
    let robot_regex = regex!("p=(\\d+),(\\d+) v=(-?\\d+),(-?\\d+)");
    let (_, [x, y, vx, vy]) = robot_regex
        .captures(line)
        .expect("robot position and velocity")
        .extract();

    let (x, y) = (
        x.parse::<i64>().expect("numeric x-position"),
        y.parse::<i64>().expect("numeric y-position"),
    );

    let (vx, vy) = (
        vx.parse::<i64>().expect("numeric x-velocity"),
        vy.parse::<i64>().expect("numeric y-velocity"),
    );

    Robot {
        pos: (x, y),
        velocity: (vx, vy),
    }
}

const GRID_WIDTH: i64 = 101;
const GRID_HEIGHT: i64 = 103;

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    pos: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    fn move_spaces(&mut self, s: i64, width: i64, height: i64) {
        let x = self.pos.0 + self.velocity.0 * s;
        let y = self.pos.1 + self.velocity.1 * s;
        self.pos = (modulus(x, width), modulus(y, height));
    }

    fn quadrant(&self, width: i64, height: i64) -> Option<Quadrant> {
        let (x_m, y_m) = (width / 2, height / 2);
        if self.pos.0 > x_m && self.pos.1 < y_m {
            Some(Quadrant::First)
        } else if self.pos.0 < x_m && self.pos.1 < y_m {
            Some(Quadrant::Second)
        } else if self.pos.0 < x_m && self.pos.1 > y_m {
            Some(Quadrant::Third)
        } else if self.pos.0 > x_m && self.pos.1 > y_m {
            Some(Quadrant::Fourth)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

fn modulus(a: i64, b: i64) -> i64 {
    let rem = a % b;
    if rem < 0 {
        rem + b
    } else {
        rem
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sample() {
        assert_eq!(
            Robot {
                pos: (0, 4),
                velocity: (3, -3),
            },
            parse_line("p=0,4 v=3,-3")
        );
    }

    #[test]
    fn test_move() {
        let mut robot = Robot {
            pos: (2, 4),
            velocity: (2, -3),
        };

        robot.move_spaces(5, 11, 7);
        assert_eq!((1, 3), robot.pos);
    }

    #[test]
    fn test_quadrant() {
        let robot = Robot {
            pos: (2, 4),
            velocity: (2, -3),
        };

        assert_eq!(Some(Quadrant::Third), robot.quadrant(11, 7));
    }

    #[test]
    fn sample() {
        let lines = [
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ];

        let robots = lines.iter().map(|s| parse_line(s)).collect::<Vec<_>>();
    }
}
