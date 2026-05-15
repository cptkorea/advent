use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day22;

impl AdventProblem for Day22 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| {
                let n = s.parse::<u64>().expect("numeric value");
                final_secret_number(n)
            })
            .sum::<u64>();

        println!("total={}", total);
        Ok(0)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut prices: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

        for line in &lines {
            let n = line.parse::<u64>().expect("numeric value");
            let next_prices = banana_prices(n);

            for (seq, p) in next_prices {
                *prices.entry(seq).or_insert(0) += p;
            }
        }

        println!(
            "best_price={}",
            prices.values().max().expect("empty sequence")
        );

        Ok(0)
    }
}

fn banana_prices(n: u64) -> HashMap<(i64, i64, i64, i64), u64> {
    let first = n;
    let second = next_secret_number(first);
    let third = next_secret_number(second);
    let fourth = next_secret_number(third);

    let mut prices: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

    let mut prev_seq = (
        compute_diff(first, second),
        compute_diff(second, third),
        compute_diff(third, fourth),
    );

    let mut prev = fourth;

    for _ in 3..2000 {
        let next = next_secret_number(prev);
        let price = next % 10;
        let diff = compute_diff(prev, next);

        let seq = (prev_seq.0, prev_seq.1, prev_seq.2, diff);

        if price > 0 {
            if !prices.contains_key(&seq) {
                prices.insert(seq, price);
            }
        }

        prev = next;
        prev_seq = (prev_seq.1, prev_seq.2, diff);
    }

    prices
}

fn compute_diff(first: u64, second: u64) -> i64 {
    (second % 10) as i64 - (first % 10) as i64
}

fn final_secret_number(n: u64) -> u64 {
    let mut next = n;
    for _ in 0..2000 {
        next = next_secret_number(next);
    }
    next
}

fn next_secret_number(n: u64) -> u64 {
    let next = mix_and_prune(64 * n, n);
    let next = mix_and_prune(next / 32, next);
    mix_and_prune(next * 2048, next)
}

fn mix_and_prune(x: u64, y: u64) -> u64 {
    (x ^ y) % 16777216
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let seq = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];

        let mut prev = 123;
        for next in seq {
            assert_eq!(next, next_secret_number(prev));
            prev = next;
        }
    }

    #[test]
    fn test_final_secret_number() {
        assert_eq!(8685429, final_secret_number(1));
        assert_eq!(4700978, final_secret_number(10));
        assert_eq!(15273692, final_secret_number(100));
        assert_eq!(8667524, final_secret_number(2024));
    }
}
