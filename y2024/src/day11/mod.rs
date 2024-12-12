use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day11;

impl AdventProblem for Day11 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut nums = lines[0]
            .split(" ")
            .map(|n| n.parse().expect("numeric value"))
            .collect::<Vec<u64>>();

        for _ in 0..25 {
            let mut next = Vec::new();
            for n in nums {
                next.extend(blink(n));
            }
            nums = next;
        }

        Ok(nums.len() as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut stone_counts: HashMap<u64, usize> = lines[0]
            .split(" ")
            .map(|n| n.parse().expect("numeric value"))
            .fold(HashMap::new(), |mut map, n| {
                *map.entry(n).or_default() += 1;
                map
            });

        for _ in 0..75 {
            stone_counts = stone_counts.iter().fold(
                HashMap::<u64, usize>::new(),
                |mut counts, (&stone, &count)| {
                    let new_stones = blink(stone);
                    for s in new_stones {
                        *counts.entry(s).or_default() += count;
                    }
                    counts
                },
            );
        }

        println!("{}", stone_counts.values().sum::<usize>());

        Ok(0)
    }
}

fn blink(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }

    let digits = n.to_string();
    if digits.len() % 2 == 0 {
        let first = digits[..digits.len() / 2].parse::<u64>().unwrap();
        let second = digits[digits.len() / 2..].parse::<u64>().unwrap();
        vec![first, second]
    } else {
        vec![n * 2024]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_blink() {
        assert_eq!(vec![253000], blink(125));
        assert_eq!(vec![1, 7], blink(17));
        assert_eq!(vec![1, 0], blink(10));
        assert_eq!(vec![202400], blink(100));
        assert_eq!(vec![10, 0], blink(1000));
        assert_eq!(vec![1], blink(0));
    }
}
