use crate::{AdventError, AdventProblem};

pub struct Day3;

impl AdventProblem for Day3 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines.iter().map(|s| parse_multiplications(s)).sum();
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut switch = true;
        let mut total = 0;

        for line in lines {
            let (res, s) = parse_multiplications_with_switch(&line, switch);
            switch = s;
            total += res;
        }

        Ok(total)
    }
}

fn parse_multiplications(s: &str) -> u32 {
    let mut i = 0;
    let mut total = 0;
    let n = s.len();
    let chars = s.chars().collect::<Vec<_>>();

    while i < n {
        if i < n - 4 && &s[i..i + 4] == "mul(" {
            let (res, idx) = parse_mul_sign(&chars, i + 4);
            if let Some((first, second)) = res {
                total += first * second;
            }
            i = idx;
        }
        i += 1;
    }
    total
}

fn parse_multiplications_with_switch(s: &str, mut process: bool) -> (u32, bool) {
    let mut i = 0;
    let mut total = 0;
    let n = s.len();
    let chars = s.chars().collect::<Vec<_>>();

    while i < n {
        if i < n - 7 && &s[i..i + 7] == "don't()" {
            process = false;
        } else if i < n - 4 && &s[i..i + 4] == "do()" {
            process = true;
        }

        if process && i < n - 4 && &s[i..i + 4] == "mul(" {
            let (res, idx) = parse_mul_sign(&chars, i + 4);
            if let Some((first, second)) = res {
                total += first * second;
            }
            i = idx;
        }
        i += 1;
    }
    (total, process)
}

fn parse_mul_sign(chars: &Vec<char>, i: usize) -> (Option<(u32, u32)>, usize) {
    let n = chars.len();
    let (first, mut i) = parse_number(&chars, i);
    if first.is_none() || i >= n || chars[i] != ',' {
        return (None, i);
    } else {
        i += 1;
    }
    let (second, i) = parse_number(&chars, i);
    if second.is_none() || i >= n || chars[i] != ')' {
        return (None, i);
    }
    (Some((first.unwrap(), second.unwrap())), i)
}

fn parse_number(chars: &Vec<char>, mut i: usize) -> (Option<u32>, usize) {
    let n = chars.len();
    if i >= n || !chars[i].is_digit(10) {
        return (None, i);
    }

    let mut num = 0;
    while i < n && chars[i].is_digit(10) {
        num *= 10;
        num += chars[i].to_digit(10).unwrap();
        i += 1;
    }
    (Some(num), i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_mul() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let factor = parse_multiplications(input);
        assert_eq!(2 * 4 + 5 * 5 + 11 * 8 + 8 * 5, factor);
    }

    #[test]
    fn parse_mul_with_switch() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let factor = parse_multiplications_with_switch(input);
        assert_eq!(48, factor);
    }
}
