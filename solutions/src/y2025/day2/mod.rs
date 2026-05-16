use crate::{AdventError, AdventProblem};

pub struct Day2;

impl AdventProblem for Day2 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut total_cnt = 0;

        let s = &lines[0];
        println!("{s}");

        for part in lines[0].split(",") {
            let mut num_split = part.split("-").take(2);
            let (s, e) = (
                num_split
                    .next()
                    .ok_or_else(|| {
                        AdventError::InputParseError("missing numeric range start/end".into())
                    })?
                    .parse::<u64>()?,
                num_split
                    .next()
                    .ok_or_else(|| {
                        AdventError::InputParseError("missing numeric range start/end".into())
                    })?
                    .parse::<u64>()?,
            );

            total_cnt += invalid_id_sum(s, e, is_repeat_seq);
        }

        println!("total sum = {total_cnt}");
        Ok(0)

        // Ok(total_cnt as u64)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut total_cnt = 0;

        let s = &lines[0];
        println!("{s}");

        for part in lines[0].split(",") {
            let mut num_split = part.split("-").take(2);
            let (s, e) = (
                num_split
                    .next()
                    .ok_or_else(|| {
                        AdventError::InputParseError("missing numeric range start/end".into())
                    })?
                    .parse::<u64>()?,
                num_split
                    .next()
                    .ok_or_else(|| {
                        AdventError::InputParseError("missing numeric range start/end".into())
                    })?
                    .parse::<u64>()?,
            );

            total_cnt += invalid_id_sum(s, e, is_multi_repeat_seq);
        }

        println!("total sum = {total_cnt}");
        Ok(0)
    }
}

fn invalid_id_sum<F: Fn(u64) -> bool>(s: u64, e: u64, f: F) -> u64 {
    let mut cnt = 0;
    for n in s..=e {
        if f(n) {
            cnt += n;
        }
    }
    cnt
}

fn is_repeat_seq(id: u64) -> bool {
    let chars = id.to_string().chars().collect::<Vec<char>>();

    let n = chars.len();
    if !n.is_multiple_of(2) {
        return false;
    }

    for i in 0..(n / 2) {
        if chars[i] != chars[n / 2 + i] {
            return false;
        }
    }

    true
}

fn is_multi_repeat_seq(id: u64) -> bool {
    let chars = id.to_string().chars().collect::<Vec<char>>();

    let n = chars.len();

    fn is_d_repeat_seq(chars: &[char], d: usize) -> bool {
        let num_chunks = chars.len() / d;

        for i in 0..d {
            for j in 1..=(num_chunks - 1) {
                if chars[i] != chars[i + j * d] {
                    return false;
                }
            }
        }

        true
    }

    for d in (1..=n / 2).rev() {
        if !n.is_multiple_of(d) {
            continue;
        }

        if is_d_repeat_seq(&chars, d) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let cases = [
            (11, 22, 33),
            (95, 115, 99),
            (998, 1012, 2019),
            (565653, 565659, 0),
        ];

        for (s, e, expect) in cases {
            assert_eq!(expect, invalid_id_sum(s, e, is_repeat_seq));
        }
    }

    #[test]
    fn sample_part_2() {
        let cases = [
            (11, 22, 33),
            (95, 115, 210),
            (998, 1012, 2009),
            (565653, 565659, 565656),
        ];

        for (s, e, expect) in cases {
            assert_eq!(expect, invalid_id_sum(s, e, is_multi_repeat_seq));
        }
    }
}
