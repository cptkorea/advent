use advent_common::number::NumSequence;

use crate::{AdventError, AdventProblem};
use good_lp::{
    default_solver, variable, variables, Expression, ResolutionError, Solution, SolverModel, Variable,
};
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
        match self.min_presses_p2_ilp() {
            Ok(v) => v,
            Err(_) => self.min_presses_p2_bfs(),
        }
    }

    /// Minimum total presses via ILP: button `i` pressed `x_i` times (non‑negative integer);
    /// for each light `j`, `sum_{i : j ∈ button_i} x_i = counters[j]`. Objective: `min Σ x_i`.
    fn min_presses_p2_ilp(&self) -> Result<u64, ResolutionError> {
        let nb = self.buttons.len();
        let nl = self.counters.len();

        let mut vars = variables!();
        let x: Vec<Variable> = (0..nb)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        let mut objective = Expression::from(0.0);
        for &v in &x {
            objective = objective + v;
        }

        let mut model = vars.minimise(objective).using(default_solver);
        for j in 0..nl {
            let mut lhs = Expression::from(0.0);
            for (i, btn) in self.buttons.iter().enumerate() {
                if btn.contains(&j) {
                    lhs = lhs + x[i];
                }
            }
            model = model.with(lhs.eq(self.counters[j] as f64));
        }

        let solution = model.solve()?;
        let total: f64 = x.iter().map(|&v| solution.value(v)).sum();
        Ok(total.round() as u64)
    }

    fn min_presses_p2_bfs(&self) -> u64 {
        let n: usize = self.requirements.len();
        let mut min_steps = HashMap::new();
        let start = Counter::new(n);
        min_steps.insert(start.clone(), 0);

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, start.clone())));

        let target = Counter {
            states: self.counters.clone(),
        };

        while !min_heap.is_empty() {
            let (dist, counter) = min_heap.pop().unwrap().0;
            // println!("dist={dist}, counter={counter:?}");

            if counter == target {
                // continue 'iter;
                return dist;
            }

            'button: for seq in &self.buttons {
                let mut new_pos = counter.clone();
                for &lt in seq {
                    new_pos.states[lt] += 1;
                }

                for (i, &p) in new_pos.states.iter().enumerate() {
                    if p > target.states[i] {
                        // println!("exceeed state counter={counter:?}");
                        continue 'button;
                    }
                }

                if !min_steps.contains_key(&new_pos) || min_steps[&new_pos] > dist + 1 {
                    min_steps.insert(new_pos.clone(), dist + 1);
                    min_heap.push(Reverse((dist + 1, new_pos)));
                }
            }
        }

        min_steps[&target]
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
        let m1 = Machine {
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

        assert_eq!(10, m1.min_presses_p2());

        let m2 = Machine::try_from("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
            .unwrap();
        assert_eq!(12, m2.min_presses_p2());

        let m3 =
            Machine::try_from("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
                .unwrap();
        assert_eq!(11, m3.min_presses_p2());
    }

    #[test]
    fn min_presses_p2_ilp_matches_bfs_on_samples() {
        let m1 = Machine {
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
        assert_eq!(m1.min_presses_p2_ilp().unwrap(), m1.min_presses_p2_bfs());

        let m2 = Machine::try_from(
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        )
        .unwrap();
        assert_eq!(m2.min_presses_p2_ilp().unwrap(), m2.min_presses_p2_bfs());

        let m3 = Machine::try_from(
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        )
        .unwrap();
        assert_eq!(m3.min_presses_p2_ilp().unwrap(), m3.min_presses_p2_bfs());
    }
}
