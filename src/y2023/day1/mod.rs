use crate::{AdventError, AdventProblem};

pub struct Day1;

impl AdventProblem for Day1 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| first_last_digit(s))
            .fold(0, |x, y| x + y.0 * 10 + y.1);
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| alphabetic_first_last_digit(s))
            .fold(0, |x, y| x + y.0 * 10 + y.1);
        Ok(total)
    }
}

fn first_last_digit(s: &str) -> (u32, u32) {
    let (mut first, mut last) = (0, 0);

    for c in s.chars() {
        if let Some(d) = c.to_digit(10) {
            first = d;
        }
    }

    for c in s.chars().rev() {
        if let Some(d) = c.to_digit(10) {
            last = d;
        }
    }

    (first, last)
}

fn alphabetic_first_last_digit(s: &str) -> (u32, u32) {
    let (mut first, mut last) = (0, 0);

    let n = s.len();
    let chars = s.chars().collect::<Vec<_>>();

    for (i, c) in chars.iter().enumerate() {
        if let Some(d) = c.to_digit(10) {
            first = d;
            break;
        } else if let Some(d) = match_digit(&s[i..]) {
            first = d;
            break;
        }
    }

    for (i, c) in chars.iter().rev().enumerate() {
        if let Some(d) = c.to_digit(10) {
            last = d;
            break;
        } else if let Some(d) = match_digit(&s[n - i - 1..]) {
            last = d;
            break;
        }
    }

    (first, last)
}

fn match_digit(s: &str) -> Option<u32> {
    let n = s.len();
    match &s[0..1] {
        "o" => {
            if n >= 3 && &s[..3] == "one" {
                return Some(1);
            }
        }
        "t" => {
            if n >= 3 && &s[..3] == "two" {
                return Some(2);
            }

            if n >= 5 && &s[..5] == "three" {
                return Some(3);
            }
        }
        "f" => {
            if n >= 4 && &s[..4] == "four" {
                return Some(4);
            }

            if n >= 4 && &s[..4] == "five" {
                return Some(5);
            }
        }
        "s" => {
            if n >= 3 && &s[..3] == "six" {
                return Some(6);
            }

            if n >= 5 && &s[..5] == "seven" {
                return Some(7);
            }
        }
        "e" => {
            if n >= 5 && &s[..5] == "eight" {
                return Some(8);
            }
        }
        "n" => {
            if n >= 4 && &s[..4] == "nine" {
                return Some(9);
            }
        }
        _ => (),
    }

    None
}
