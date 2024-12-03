use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day2;

impl AdventProblem for Day2 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let num_safe = lines
            .iter()
            .filter(|&s| is_safe_sequence(parse_numbers(s)))
            .count();
        Ok(num_safe as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let num_safe = lines
            .iter()
            .filter(|&s| is_safe_with_threshold(parse_numbers(s), 1))
            .count();
        Ok(num_safe as u32)
    }
}

fn parse_numbers(s: &str) -> Vec<u32> {
    let parts = s.split_whitespace();
    let numbers = parts
        .into_iter()
        .map(|p| p.parse::<u32>().expect("numeric sample"))
        .collect::<Vec<_>>();

    numbers
}

fn is_safe_sequence(sequence: Vec<u32>) -> bool {
    if sequence.len() <= 1 {
        return true;
    }

    let first = sequence[0];
    let second = sequence[1];

    let mut increasing = false;
    if second > first {
        increasing = true;
    }

    for i in 1..sequence.len() {
        let (first, second) = (sequence[i - 1], sequence[i]);
        if !is_safe(first, second, increasing) {
            return false;
        }
    }

    true
}

fn is_safe_with_threshold(sequence: Vec<u32>, threshold: usize) -> bool {
    if is_safe_sequence(sequence.clone()) {
        return true;
    }

    for i in 0..sequence.len() {
        let chunk = [&sequence[..i], &sequence[i + 1..]].concat();
        if is_safe_sequence(chunk) {
            return true;
        }
    }
    false
}

fn is_safe(first: u32, second: u32, increasing: bool) -> bool {
    let diff_check = 1 <= second.abs_diff(first) && second.abs_diff(first) <= 3;
    let monotonic_check = increasing && first < second || !increasing && second < first;
    diff_check && monotonic_check
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line() {
        let input = "7 6 4 2 1";

        let numbers = parse_numbers(input);
        assert_eq!(vec![7, 6, 4, 2, 1], numbers);
    }

    #[test]
    fn safety() {
        assert!(is_safe_sequence(vec![7, 6, 4, 2, 1]));
        assert!(!is_safe_sequence(vec![1, 2, 7, 8, 9]));
        assert!(!is_safe_sequence(vec![9, 7, 6, 2, 1]));
        assert!(!is_safe_sequence(vec![1, 3, 2, 4, 5]));
        assert!(!is_safe_sequence(vec![8, 6, 4, 4, 1]));
        assert!(is_safe_sequence(vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn safety_with_threshold() {
        assert!(is_safe_with_threshold(vec![7, 6, 4, 2, 1], 1));
        assert!(!is_safe_with_threshold(vec![1, 2, 7, 8, 9], 1));
        assert!(!is_safe_with_threshold(vec![9, 7, 6, 2, 1], 1));
        assert!(is_safe_with_threshold(vec![1, 3, 2, 4, 5], 1));
        assert!(is_safe_with_threshold(vec![8, 6, 4, 4, 1], 1));
        assert!(is_safe_with_threshold(vec![1, 3, 6, 7, 9], 1));
    }
}
