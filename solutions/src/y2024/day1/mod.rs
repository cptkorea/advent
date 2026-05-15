use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day1;

impl AdventProblem for Day1 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (mut first_measurements, mut second_measurements) = parse_lines(&lines);
        first_measurements.sort();
        second_measurements.sort();

        let total = first_measurements
            .iter()
            .zip(second_measurements.iter())
            .map(|(f, s)| f.abs_diff(*s))
            .sum();

        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (first_measurements, second_measurements) = parse_lines(&lines);
        let frequencies = collect_frequencies(&second_measurements);

        let total = first_measurements
            .iter()
            .map(|v| match frequencies.get(v) {
                Some(freq) => v * freq,
                None => 0,
            })
            .sum();

        Ok(total)
    }
}

fn parse_lines(lines: &Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut first_measurements = Vec::with_capacity(lines.len());
    let mut second_measurements = Vec::with_capacity(lines.len());

    for line in lines {
        let (first, second) = parse_ids(line);
        first_measurements.push(first);
        second_measurements.push(second);
    }

    (first_measurements, second_measurements)
}

fn collect_frequencies(measurements: &Vec<u32>) -> HashMap<u32, u32> {
    let mut frequencies = HashMap::new();

    measurements.iter().for_each(|m| {
        *frequencies.entry(*m).or_default() += 1;
    });

    frequencies
}

fn parse_ids(s: &str) -> (u32, u32) {
    let mut parts = s.split_whitespace();
    let first = parts
        .next()
        .expect("numeric left value")
        .parse::<u32>()
        .expect("left value is non-numeric");

    let second = parts
        .next()
        .expect("numeric right value")
        .parse::<u32>()
        .expect("left value is non-numeric");

    (first, second)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_ids_small() {
        let input = "3   4";

        let (first, second) = parse_ids(input);
        assert_eq!(3, first);
        assert_eq!(4, second);
    }
}
