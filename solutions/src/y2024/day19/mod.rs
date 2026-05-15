use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day19;

impl AdventProblem for Day19 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let patterns = lines[0].split(", ").collect::<Vec<_>>();
        let designs = lines.iter().skip(2).collect::<Vec<_>>();

        let mut total = 0;
        for d in designs {
            if is_constructable(d, &patterns, 0) {
                total += 1;
            }
        }

        Ok(total as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let patterns = lines[0].split(", ").collect::<Vec<_>>();
        let designs = lines.iter().skip(2).collect::<Vec<_>>();

        let mut total = 0;
        let mut sequence_counts: HashMap<&str, u64> = HashMap::new();
        for d in designs {
            total += count_constructable(d, &patterns, &mut sequence_counts, 0);
        }
        println!("{}", total);
        Ok(0)
    }
}

fn is_constructable(line: &str, patterns: &[&str], i: usize) -> bool {
    let n = line.len();
    if i == n {
        return true;
    }

    for &p in patterns {
        if i + p.len() <= n && &line[i..i + p.len()] == p {
            if is_constructable(line, patterns, i + p.len()) {
                return true;
            }
        }
    }
    false
}

fn count_constructable<'a>(
    line: &'a str,
    patterns: &[&str],
    sequence_counts: &mut HashMap<&'a str, u64>,
    i: usize,
) -> u64 {
    let n = line.len();

    if sequence_counts.contains_key(&line[i..]) {
        return *sequence_counts.get(&line[i..]).unwrap();
    }

    if i == n {
        return 1;
    }

    let mut total = 0;
    for &p in patterns {
        if i + p.len() <= n && &line[i..i + p.len()] == p {
            match sequence_counts.get(&line[i + p.len()..]) {
                Some(cnt) => {
                    total += *cnt;
                }
                None => {
                    let cnt = count_constructable(line, patterns, sequence_counts, i + p.len());
                    total += cnt;
                }
            }
        }
    }
    *sequence_counts.entry(&line[i..]).or_default() += total;

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        assert!(is_constructable("brwrr", &patterns, 0));
        assert!(is_constructable("bggr", &patterns, 0));
        assert!(is_constructable("gbbr", &patterns, 0));
        assert!(is_constructable("rrbgbr", &patterns, 0));
        assert!(!is_constructable("ubwu", &patterns, 0));
        assert!(is_constructable("bwurrg", &patterns, 0));
        assert!(is_constructable("brgr", &patterns, 0));
        assert!(!is_constructable("bbrgwb", &patterns, 0));
    }

    #[test]
    fn sample_2() {
        let patterns = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        let mut sequence_counts = HashMap::new();

        assert_eq!(
            2,
            count_constructable("brwrr", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            1,
            count_constructable("bggr", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            4,
            count_constructable("gbbr", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            6,
            count_constructable("rrbgbr", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            0,
            count_constructable("ubwu", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            1,
            count_constructable("bwurrg", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            2,
            count_constructable("brgr", &patterns, &mut sequence_counts, 0)
        );
        assert_eq!(
            0,
            count_constructable("bbrgwb", &patterns, &mut sequence_counts, 0)
        );
    }
}
