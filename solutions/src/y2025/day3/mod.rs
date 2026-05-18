use crate::{AdventError, AdventProblem};

pub struct Day3;

impl AdventProblem for Day3 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let res = lines
            .iter()
            .map(|s| find_max_joltage(s.as_str(), 2))
            .fold(0, |t, s| t + s);

        Ok(0)
        // Ok(res)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let res = lines
            .iter()
            .map(|s| find_max_joltage(s.as_str(), 12))
            .fold(0, |t, s| t + s);

        println!("res={res}");
        Ok(0)
    }
}

fn find_max_joltage(s: &str, pick: usize) -> u64 {
    let chars = s.chars().collect::<Vec<_>>();
    let n = chars.len();

    let mut first = 0;
    let mut f = 0usize;

    for p in 0..pick {
        // let start = if p == 0 { 0 } else { (f + 1) };

        let mut nxt = 0;

        let end = n - (pick - p);

        for i in f..=end {
            let c = chars[i];
            let d = c.to_digit(10).expect("not a digit") as u64;
            if d > nxt {
                nxt = d;
                f = i + 1
            }
        }

        first = first * 10 + nxt
    }

    first

    // let second = chars[n - 1].to_digit(10).expect("not a digit");

    // let (mut l, mut r) = (0, n - 1);

    // while l < r {
    //     if chars[l] > chars[r] {

    //     }

    //     l -= 1;
    //     r += 1;
    // }

    // first * 10 + second;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let cases = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];

        for (s, jolt) in cases {
            assert_eq!(jolt, find_max_joltage(s, 2));
        }
    }

    #[test]
    fn sample_part_2() {
        let cases = [
            ("987654321111111", 987654321111),
            ("811111111111119", 811111111119),
            ("234234234234278", 434234234278),
            ("818181911112111", 888911112111),
        ];

        for (s, jolt) in cases {
            assert_eq!(jolt, find_max_joltage(s, 12));
        }
    }
}
