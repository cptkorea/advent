use std::ops::{Add, Mul};

use crate::{AdventError, AdventProblem};

pub struct Day7;

impl AdventProblem for Day7 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| Calibration::from(s))
            .filter(|c| c.can_finish())
            .map(|c| c.total)
            .sum::<u64>();

        println!("total={}", total);

        Ok(0)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| Calibration::from(s))
            .filter(|c| c.can_finish_with_concatenate())
            .map(|c| c.total)
            .sum::<u64>();

        println!("total={}", total);

        Ok(0)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Calibration {
    total: u64,
    parts: Vec<u64>,
}

impl Calibration {
    fn can_finish(&self) -> bool {
        self.try_evaluate(1, self.parts[0], &vec![u64::add, u64::mul])
    }

    fn can_finish_with_concatenate(&self) -> bool {
        self.try_evaluate(1, self.parts[0], &vec![u64::add, u64::mul, concatenate])
    }

    fn try_evaluate<F>(&self, idx: usize, running: u64, operators: &Vec<F>) -> bool
    where
        F: Fn(u64, u64) -> u64 + Clone,
    {
        if running > self.total {
            return false;
        }
        if idx == self.parts.len() {
            return running == self.total;
        }

        for op in operators {
            if self.try_evaluate(idx + 1, op(running, self.parts[idx]), operators) {
                return true;
            }
        }
        false
    }
}

fn concatenate(a: u64, b: u64) -> u64 {
    let mut total = a.to_string();
    total.push_str(&b.to_string());

    total.parse::<u64>().expect("concatenate NaN")
}

impl<S> From<S> for Calibration
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let mut split = s.as_ref().split(":");
        let total = split
            .next()
            .expect("missing total calibration value")
            .parse::<u64>()
            .expect("non-numeric total value");

        let parts = split.next().expect("missing calibartion parts")[1..]
            .split(" ")
            .into_iter()
            .map(|s| s.parse::<u64>().expect("non-numeric calibration part"))
            .collect();

        Calibration { total, parts }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let line = "161011: 16 10 13";
        assert_eq!(
            Calibration {
                total: 161011,
                parts: vec![16, 10, 13],
            },
            Calibration::from(line),
        );
    }

    #[test]
    fn can_finish() {
        let calibration = Calibration {
            total: 3267,
            parts: vec![81, 40, 27],
        };

        assert!(calibration.can_finish());
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(123, concatenate(1, 23));
    }

    #[test]
    fn can_finish_with_concatenate() {
        let calibration = Calibration {
            total: 7290,
            parts: vec![6, 8, 6, 15],
        };

        assert!(calibration.can_finish_with_concatenate());
    }
}
