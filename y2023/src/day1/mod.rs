use crate::{AdventError, AdventProblem};

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub struct Day1;

impl AdventProblem for Day1 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines.iter().map(|s| calibration_value(s)).sum();
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines.iter().map(|s| alpha_calibration_value(s)).sum();
        Ok(total)
    }
}

fn calibration_value(s: &str) -> u32 {
    let mut calibration_value = 0;

    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            calibration_value += 10 * d;
            break;
        }
    }

    for c in s.chars().rev() {
        if let Some(d) = c.to_digit(10) {
            calibration_value += d;
            break;
        }
    }
    println!("forward s={}, calibration={}", s, calibration_value);
    calibration_value
}

fn alpha_calibration_value(s: &str) -> u32 {
    let mut calibration_value = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            calibration_value += 10 * d;
            break;
        }

        if let Some(d) = find_digit_from_start(s, i) {
            calibration_value += 10 * d;
            break;
        }
    }

    for (j, c) in s.chars().rev().enumerate() {
        if let Some(d) = c.to_digit(10) {
            calibration_value += d;
            break;
        }

        if let Some(d) = find_digit_from_end(s, s.len() - j) {
            calibration_value += d;
            break;
        }
    }

    calibration_value
}

/// searches s ending at index j for an alphabetic number digit match
fn find_digit_from_start(s: &str, i: usize) -> Option<u32> {
    for (v, &num) in NUMBERS.iter().enumerate() {
        // SAFETY: Take max with 0 first to avoid unsigned integer underflow
        let j = std::cmp::min(i + num.len(), s.len());
        if &s[i..j] == num {
            return Some(v as u32 + 1);
        }
    }
    None
}

/// searches s ending at index j for an alphabetic number digit match
fn find_digit_from_end(s: &str, j: usize) -> Option<u32> {
    for (v, &num) in NUMBERS.iter().enumerate() {
        // SAFETY: Take max with 0 first to avoid unsigned integer underflow
        let i = j.checked_sub(num.len()).unwrap_or(0);
        if &s[i..j] == num {
            return Some(v as u32 + 1);
        }
    }
    None
}
