use crate::{AdventError, AdventProblem};
use std::collections::{HashMap, HashSet};

pub struct Day11;

impl AdventProblem for Day11 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let server_rack = ServerRack::try_from(lines)?;

        // let res = presses.iter().fold(1, |p, s| p * s);
        Ok(server_rack.count_paths())
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let server_rack = ServerRack::try_from(lines)?;

        // let res = presses.iter().fold(1, |p, s| p * s);
        Ok(server_rack.count_dac_fft_paths())
        // Ok(0)
    }
}

const START: &'static str = "you";
const SVR: &'static str = "svr";
const TERMINAL: &'static str = "out";

const DAC: &'static str = "dac";
const FFT: &'static str = "fft";

struct ServerRack {
    adj_list: HashMap<String, Vec<String>>,
}

impl ServerRack {
    fn count_paths(&self) -> usize {
        let mut memo = HashMap::new();
        self.path_count_exclusive(START, TERMINAL, &mut memo)
    }

    fn count_dac_fft_paths(&self) -> usize {
        let mut memo = HashMap::new();

        let dac_fft_paths = self.path_count_exclusive(DAC, FFT, &mut memo);
        let fft_dac_paths = self.path_count_exclusive(FFT, DAC, &mut memo);
        let st_fft_paths = self.path_count_exclusive(SVR, FFT, &mut memo);
        let st_dac_paths = self.path_count_exclusive(SVR, DAC, &mut memo);
        let dac_end_paths = self.path_count_exclusive(DAC, TERMINAL, &mut memo);
        let fft_end_paths = self.path_count_exclusive(FFT, TERMINAL, &mut memo);

        let branch_a = st_fft_paths
            .saturating_mul(fft_dac_paths)
            .saturating_mul(dac_end_paths);
        let branch_b = st_dac_paths
            .saturating_mul(dac_fft_paths)
            .saturating_mul(fft_end_paths);
        branch_a.saturating_add(branch_b)
    }

    /// Count of directed paths from `start` to `terminal`, using the **same traversal rules** as
    /// before: stepping only along outgoing edges, with the `"out"` sentinel rule preserved.
    /// `(start, terminal)` results are memoized — count is summed over outgoing neighbors without
    /// materializing path strings.
    fn path_count_exclusive(
        &self,
        start: &str,
        terminal: &str,
        memo: &mut HashMap<(String, String), usize>,
    ) -> usize {
        let key = (start.to_string(), terminal.to_string());
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }

        let count = if start == TERMINAL && terminal != TERMINAL {
            0
        } else if start == terminal {
            1
        } else {
            let mut total = 0usize;
            for nei in self.adj_list.get(start).into_iter().flatten() {
                total = total.saturating_add(self.path_count_exclusive(nei, terminal, memo));
            }
            total
        };

        memo.insert(key, count);
        count
    }
}

impl<S: AsRef<str>> TryFrom<Vec<S>> for ServerRack {
    type Error = AdventError;

    fn try_from(lines: Vec<S>) -> Result<Self, Self::Error> {
        // let mut nodes = HashSet::new();
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();

        for line in lines {
            let line_str = line.as_ref();
            let n = &line_str[..3];

            // nodes.insert(n.into());

            let neighbors = line_str[5..].split(" ").collect::<Vec<_>>();
            for nei in neighbors {
                // nodes.insert(nei.into());
                adj_list.entry(n.into()).or_default().push(nei.into());
            }
        }

        Ok(Self { adj_list })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let lines = vec![
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ];

        let rack = ServerRack::try_from(lines).unwrap();
        assert_eq!(5, rack.count_paths());
    }

    #[test]
    fn sample_part_2() {
        let lines = vec![
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];

        let rack = ServerRack::try_from(lines).unwrap();
        assert_eq!(2, rack.count_dac_fft_paths());
    }
}
