use crate::{AdventError, AdventProblem};
use advent_common::range::ParseRangeInclusive;
// use core::range::Range;
use std::{cmp::Ordering, ops::RangeInclusive};

pub struct Day5;

impl AdventProblem for Day5 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let ingredient_db = IngredientDB::try_from(lines)?;
        Ok(ingredient_db.count_fresh())
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let ingredient_db = IngredientDB::try_from(lines)?;
        Ok(ingredient_db.range_count())
    }
}

struct IngredientDB {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredient_ids: Vec<usize>,
}

impl IngredientDB {
    fn range_count(&self) -> usize {
        let mut last_rng = self.fresh_ranges[0].clone();
        let mut total_cnt = last_rng.end() - last_rng.start() + 1;

        for rng in self.fresh_ranges.iter().skip(1) {
            // if rng.start() < last_rng.end() || rng.end() < last_rng.end() {
            //     continue;
            // }

            if last_rng.end() < rng.start() {
                last_rng = rng.clone();
                total_cnt += rng.end() - rng.start() + 1;
            } else if last_rng.end() == rng.start() {
                total_cnt += rng.end() - rng.start();
                last_rng = RangeInclusive::new(last_rng.start().clone(), rng.end().clone())
            } else {
                if rng.end() <= last_rng.end() {
                    continue;
                }

                let end = std::cmp::max(last_rng.end(), rng.end());
                total_cnt += end - last_rng.end();
                last_rng = RangeInclusive::new(last_rng.start().clone(), end.clone());
            }
        }

        total_cnt
    }

    fn count_fresh(&self) -> usize {
        self.ingredient_ids
            .iter()
            .filter(|&id| self.is_fresh(*id))
            .count()
    }

    fn is_fresh(&self, id: usize) -> bool {
        self.fresh_ranges.iter().any(|rng| rng.contains(&id))
    }
}

impl TryFrom<Vec<String>> for IngredientDB {
    type Error = AdventError;

    fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
        let mut fresh_ranges: Vec<RangeInclusive<usize>> = vec![];
        let mut ingredient_ids: Vec<usize> = vec![];

        let mut i = 0;
        while lines[i] != "" {
            let rng = RangeInclusive::parse(&lines[i])?;
            fresh_ranges.push(rng);
            i += 1;
        }

        i += 1;

        while i < lines.len() {
            let line = &lines[i];
            ingredient_ids.push(line.parse::<usize>()?);
            i += 1;
        }

        fresh_ranges.sort_by(|r1, r2| {
            let s_cmp = r1.start().cmp(r2.start());
            if s_cmp != Ordering::Equal {
                return s_cmp;
            }
            r1.end().cmp(r2.end())
        });

        Ok(Self {
            fresh_ranges,
            ingredient_ids,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let db = IngredientDB {
            fresh_ranges: vec![
                RangeInclusive::new(3, 5),
                RangeInclusive::new(10, 14),
                RangeInclusive::new(16, 20),
                RangeInclusive::new(12, 18),
            ],
            ingredient_ids: vec![1, 5, 8, 11, 17, 32],
        };

        assert_eq!(3, db.count_fresh());
    }

    #[test]
    fn sample_part_2() {
        let db = IngredientDB {
            fresh_ranges: vec![
                RangeInclusive::new(3, 5),
                RangeInclusive::new(10, 14),
                RangeInclusive::new(12, 18),
                RangeInclusive::new(16, 20),
            ],
            ingredient_ids: vec![1, 5, 8, 11, 17, 32],
        };

        assert_eq!(14, db.range_count());
    }
}
