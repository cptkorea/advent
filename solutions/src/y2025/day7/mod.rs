use crate::{AdventError, AdventProblem};
use std::collections::HashSet;

pub struct Day7;

impl AdventProblem for Day7 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let manifold = Manifold::try_from(lines)?;
        Ok(manifold.count_splits())
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let manifold = Manifold::try_from(lines)?;
        Ok(manifold.count_timelines_dp())
    }
}

pub struct Manifold {
    start: (usize, usize),
    grid: Vec<Vec<char>>,
    splitters: Vec<(usize, usize)>,
}

impl<S: AsRef<str>> TryFrom<Vec<S>> for Manifold {
    type Error = AdventError;

    fn try_from(lines: Vec<S>) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<char>> = lines.iter().map(|l| l.as_ref().chars().collect()).collect();

        let mut start: Option<(usize, usize)> = None;
        let mut splitters = vec![];

        for (r, row) in grid.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if *val == 'S' {
                    start = Some((r, c));
                } else if *val == '^' {
                    splitters.push((r, c));
                }
            }
        }

        let Some(start) = start else {
            return Err(AdventError::InputParseError(
                "missing start position 'S'".into(),
            ));
        };

        Ok(Manifold {
            start,
            splitters,
            grid,
        })
    }
}

impl Manifold {
    fn count_splits(&self) -> usize {
        let mut split_cols = HashSet::new();
        split_cols.insert(self.start.1);

        let mut num_splits = 0;
        let n = self.grid[0].len();

        for (_, c) in self.splitters.iter() {
            if split_cols.contains(c) {
                if c > &0 && !split_cols.contains(&(*c - 1)) {
                    split_cols.insert(*c - 1);
                }

                if c < &(n - 1) && !split_cols.contains(&(*c + 1)) {
                    split_cols.insert(*c + 1);
                }
                num_splits += 1;
                split_cols.remove(c);
            }
        }

        num_splits
    }

    /// Count distinct timelines via DP: frequencies per column descend the manifold unchanged
    /// between rows; at each splitter `^` in column `c`, all mass moves to `c-1` and `c+1`.
    ///
    /// Order of splitters within the same row is left-to-right (stable sort by column index).
    fn count_timelines_dp(&self) -> usize {
        let h = self.grid.len();
        let w = self.grid[0].len();

        let mut freq = vec![0usize; w];
        freq[self.start.1] = 1;

        for r in 0..h {
            let mut cols: Vec<usize> = (0..w).filter(|&c| self.grid[r][c] == '^').collect();

            if cols.is_empty() {
                continue;
            }

            cols.sort();

            for c in cols {
                let k = freq[c];
                if k == 0 {
                    continue;
                }

                freq[c] -= k;
                freq[c - 1] += k;
                freq[c + 1] += k;
            }
        }

        freq.iter().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let lines = vec![
            ".......S.......",
            ".......|.......",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let manifold = Manifold::try_from(lines).expect("valid manifold");
        assert_eq!(21, manifold.count_splits());
    }

    #[test]
    fn sample_part_2() {
        let lines = vec![
            ".......S.......",
            ".......|.......",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];

        let manifold = Manifold::try_from(lines).expect("valid manifold");
        assert_eq!(40, manifold.count_timelines_dp());
    }
}
