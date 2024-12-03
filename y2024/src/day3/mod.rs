use crate::{AdventError, AdventProblem};

pub struct Day3;

impl AdventProblem for Day3 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines.iter().map(|s| reduce(parse_multiplication(s))).sum();
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        Ok(0)
    }
}

fn parse_multiplication(s: &str) -> Vec<(u32, u32)> {
    let mut i = 0;
    let n = s.len();
    let mut factors = Vec::new();
    let chars = s.chars().collect::<Vec<_>>();

    while i < n {
        if i < n - 4 && &s[i..i + 4] == "mul(" {
            i += 4;
            let first;
            match parse_number(&chars, i) {
                Some((num, idx)) => {
                    first = num;
                    i = idx;
                }
                None => continue,
            }

            if i >= n || chars[i] != ',' {
                continue;
            } else {
                i += 1;
            }

            let second;
            match parse_number(&chars, i) {
                Some((num, idx)) => {
                    second = num;
                    i = idx;
                }
                None => continue,
            }
            if i >= n || chars[i] != ')' {
                continue;
            }
            factors.push((first, second));
        }
        i += 1;
    }

    factors
}

fn reduce(factors: Vec<(u32, u32)>) -> u32 {
    factors.iter().map(|(f, s)| f * s).sum()
}

fn parse_number(chars: &Vec<char>, mut i: usize) -> Option<(u32, usize)> {
    let n = chars.len();
    if i >= n || !chars[i].is_digit(10) {
        return None;
    }

    let mut num = 0;
    while i < n && chars[i].is_digit(10) {
        num *= 10;
        num += chars[i].to_digit(10).unwrap();
        i += 1;
    }
    Some((num, i))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_mul() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let factors = parse_multiplication(input);
        assert_eq!(vec![(2, 4), (5, 5), (11, 8), (8, 5)], factors);
    }
}
