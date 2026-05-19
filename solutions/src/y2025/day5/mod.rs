use crate::{AdventError, AdventProblem};
use advent_common::range::ParseRangeInclusive;
use std::ops::RangeInclusive;

pub struct Day5;

impl AdventProblem for Day5 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let ingredient_db = IngredientDB::try_from(lines)?;
        Ok(ingredient_db.count_fresh())
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        Ok(0)
    }
}

struct IngredientDB {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredient_ids: Vec<usize>,
}

impl IngredientDB {
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
            println!("rng={rng:?}");
            fresh_ranges.push(rng);
            i += 1;
        }

        i += 1;

        while i < lines.len() {
            let line = &lines[i];
            ingredient_ids.push(line.parse::<usize>()?);
            i += 1;
        }

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
}
