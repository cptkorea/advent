use advent_common::number::NumSequence;

use crate::{AdventError, AdventProblem};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub struct Day10;

impl AdventProblem for Day10 {
    type Answer = u64;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let mut total = 0;
        for line in lines {
            let m = Machine::try_from(line.as_str())?;
            total += m.min_presses();
        }

        // let res = presses.iter().fold(1, |p, s| p * s);
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let mut total = 0;
        for line in lines {
            let m = Machine::try_from(line.as_str())?;
            total += m.min_presses_p2();
        }

        // let res = presses.iter().fold(1, |p, s| p * s);
        Ok(total)
    }
}

struct Machine {
    pub requirements: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub counters: Vec<usize>,
}

impl TryFrom<&str> for Machine {
    type Error = AdventError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let parts = s.split(" ").collect::<Vec<_>>();
        let n = parts.len();

        let button_seq = parts[0].chars().collect::<Vec<_>>();

        let slice = &button_seq[1..button_seq.len() - 1];

        let requirements: Vec<bool> = (&button_seq[1..button_seq.len() - 1])
            .iter()
            .filter(|&&b| b == '.' || b == '#')
            .map(|&b| if b == '#' { true } else { false })
            .collect();

        let mut buttons = Vec::with_capacity(n - 2);
        let mut counters = Vec::with_capacity(10);
        for &rem in parts.iter().skip(1) {
            if rem.chars().next() == Some('{') {
                counters = NumSequence::<usize>::try_from(rem)?.nums;
                break;
            }

            let num_seq: NumSequence<usize> = NumSequence::try_from(rem)?;
            buttons.push(num_seq.nums);
        }

        Ok(Machine {
            requirements,
            buttons,
            counters,
        })
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Position {
    states: Vec<bool>,
}

impl Position {
    fn new(n: usize) -> Self {
        Self {
            states: vec![false; n],
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Counter {
    states: Vec<usize>,
}

impl Counter {
    fn new(n: usize) -> Self {
        Self { states: vec![0; n] }
    }
}

impl Machine {
    fn min_presses(&self) -> u64 {
        let n: usize = self.requirements.len();
        let mut min_steps = HashMap::new();
        let start = Position::new(n);
        min_steps.insert(start.clone(), 0);

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, start.clone())));

        let target = Position {
            states: self.requirements.clone(),
        };

        while !min_steps.contains_key(&target) {
            let (dist, pos) = min_heap.pop().unwrap().0;

            if pos == target {
                return dist;
            }

            for seq in &self.buttons {
                let mut new_pos = pos.clone();
                for &lt in seq {
                    new_pos.states[lt] = !new_pos.states[lt];
                }

                min_heap.push(Reverse((dist + 1, new_pos)));
            }
        }

        u64::MAX
    }

    fn min_presses_p2(&self) -> u64 {
        let n: usize = self.requirements.len();
        let mut min_steps = HashMap::new();
        let start = Counter::new(n);
        min_steps.insert(start.clone(), 0);

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, start.clone())));

        let target = Counter {
            states: self.counters.clone(),
        };

        while !min_steps.contains_key(&target) {
            let (dist, pos) = min_heap.pop().unwrap().0;

            println!("dist={dist}, pos={pos:?}");

            if pos == target {
                return dist;
            }

            for seq in &self.buttons {
                let mut new_pos = pos.clone();
                for &lt in seq {
                    new_pos.states[lt] += lt;
                }

                min_heap.push(Reverse((dist + 1, new_pos)));
            }
        }

        u64::MAX
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_machine() {
        let s = "[..####.] (2,3,4,5) (0,2,3,5,6) (1,4,6) (2,3,4) (0,5) {15,17,19,19,32,24,21}";
        let m = Machine::try_from(s).expect("invalid machine");

        assert_eq!(
            vec![false, false, true, true, true, true, false],
            m.requirements
        );
        assert_eq!(
            vec![
                vec![2, 3, 4, 5],
                vec![0, 2, 3, 5, 6],
                vec![1, 4, 6],
                vec![2, 3, 4],
                vec![0, 5],
            ],
            m.buttons
        );
        assert_eq!(vec![15, 17, 19, 19, 32, 24, 21], m.counters);
    }

    #[test]
    fn sample_part_1() {
        let machine = Machine {
            requirements: vec![false, true, true, false],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            counters: vec![],
        };

        assert_eq!(2, machine.min_presses());
    }

    #[test]
    fn sample_part_2() {
        let machine = Machine {
            requirements: vec![false, true, true, false],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            counters: vec![3, 5, 4, 7],
        };

        assert_eq!(10, machine.min_presses_p2());
    }
}
