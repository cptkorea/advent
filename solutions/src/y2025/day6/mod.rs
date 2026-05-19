use std::num::ParseIntError;

use crate::{AdventError, AdventProblem};
use advent_common::arithmetic::Operator;
// use core::range::Range;
// use std::{cmp::Ordering, ops::RangeInclusive};

pub struct Day6;

impl AdventProblem for Day6 {
    type Answer = u64;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let hwk = MathHomework::try_from(lines)?;
        Ok(hwk.solve())
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        Ok(0)
    }
}

#[derive(Debug)]
struct MathHomework {
    groups: Vec<NumberGroup>,
}

#[derive(Debug)]
struct NumberGroup {
    operator: Operator,
    nums: Vec<u32>,
}

impl MathHomework {
    fn solve(&self) -> u64 {
        self.groups.iter().map(|g| g.reduce()).fold(0, |s, n| s + n)
    }
}

impl NumberGroup {
    fn reduce(&self) -> u64 {
        match self.operator {
            Operator::Add => self.nums.iter().fold(0, |s, &n| s + n as u64),
            Operator::Multiply => self.nums.iter().fold(1, |p, &n| p * n as u64),
            _ => unimplemented!("operator not implemented"),
        }
    }
}

impl<S: AsRef<str>> TryFrom<Vec<S>> for MathHomework {
    type Error = AdventError;

    fn try_from(lines: Vec<S>) -> Result<Self, Self::Error> {
        let n = lines.len();
        // let mut groups: Vec<MathHomework> = vec![];

        let mut groups = lines[n - 1]
            .as_ref()
            .split_whitespace()
            .map(|op| {
                let res = Operator::try_from(op);
                res.map(|op| NumberGroup {
                    operator: op,
                    nums: vec![],
                })
            })
            .collect::<Result<Vec<_>, AdventError>>()?;

        let num_groups = groups.len();

        for line in lines.iter().take(n - 1) {
            let num_split: Vec<u32> = line
                .as_ref()
                .split_whitespace()
                .map(|n| n.parse::<u32>().map_err(AdventError::from))
                .collect::<Result<Vec<_>, AdventError>>()?;

            for i in 0..num_groups {
                groups[i].nums.push(num_split[i]);
            }
        }

        Ok(Self { groups })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let lines = vec![
            "123 328  51 64",
            "45 64  387 23 ",
            "6 98  215 314",
            "*   +   *   + ",
        ];

        let hwk = MathHomework::try_from(lines).expect("bad input");

        // println!("hwk={hwk:?}");
        assert_eq!(4277556, hwk.solve());
    }
}
