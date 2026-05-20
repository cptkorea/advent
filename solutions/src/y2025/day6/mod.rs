use crate::{AdventError, AdventProblem};
use advent_common::arithmetic::Operator;

pub struct Day6;

impl AdventProblem for Day6 {
    type Answer = u64;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let hwk = MathHomework::try_from(lines)?;
        Ok(homework_total(&hwk.groups))
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let hwk = CephalopodHomework::try_from(lines)?;
        Ok(homework_total(&hwk.groups))
    }
}

#[derive(Debug)]
struct MathHomework {
    groups: Vec<NumberGroup>,
}

#[derive(Debug)]
struct CephalopodHomework {
    groups: Vec<NumberGroup>,
}

#[derive(Debug)]
struct NumberGroup {
    operator: Operator,
    nums: Vec<u32>,
}

#[derive(Debug)]
struct NumberGrid {
    operator: Operator,
    /// Width in characters for this block on each data row.
    width: usize,
    nums: Vec<Vec<Option<u32>>>,
}

/// One segment from the last line: operator character followed by a run of spaces that
/// delimits the next operator. The final column has no trailing spaces; its width is
/// `spaces_after + 1` so the last digit column is included.
#[derive(Debug)]
struct ColumnSpec {
    operator: Operator,
    width: usize,
}

fn homework_total(groups: &[NumberGroup]) -> u64 {
    groups.iter().map(NumberGroup::reduce).sum()
}

impl NumberGroup {
    fn reduce(&self) -> u64 {
        match self.operator {
            Operator::Add => self.nums.iter().fold(0u64, |s, &n| s + n as u64),
            Operator::Multiply => self.nums.iter().fold(1u64, |p, &n| p * n as u64),
            _ => unimplemented!("operator not implemented"),
        }
    }
}

impl<S: AsRef<str>> TryFrom<Vec<S>> for MathHomework {
    type Error = AdventError;

    fn try_from(lines: Vec<S>) -> Result<Self, Self::Error> {
        let n = lines.len();
        let mut groups = lines[n - 1]
            .as_ref()
            .split_whitespace()
            .map(|op| {
                Operator::try_from(op).map(|operator| NumberGroup {
                    operator,
                    nums: vec![],
                })
            })
            .collect::<Result<Vec<_>, AdventError>>()?;

        let num_groups = groups.len();

        for line in lines.iter().take(n - 1) {
            let num_split: Vec<u32> = line
                .as_ref()
                .split_whitespace()
                .map(|n| n.parse::<u32>().map_err(AdventError::from))
                .collect::<Result<Vec<_>, AdventError>>()?;

            for i in 0..num_groups {
                groups[i].nums.push(num_split[i]);
            }
        }

        Ok(Self { groups })
    }
}

impl<S: AsRef<str>> TryFrom<Vec<S>> for CephalopodHomework {
    type Error = AdventError;

    fn try_from(lines: Vec<S>) -> Result<Self, Self::Error> {
        let n = lines.len();
        if n < 2 {
            return Err(AdventError::InputParseError(
                "cephalopod homework needs at least a data line and operator line".into(),
            ));
        }

        let specs = parse_cephalopod_operator_layout(lines[n - 1].as_ref())?;
        let mut grids = specs
            .into_iter()
            .map(|spec| NumberGrid {
                operator: spec.operator,
                width: spec.width,
                nums: vec![],
            })
            .collect::<Vec<_>>();

        for line in lines.iter().take(n - 1) {
            let chars: Vec<char> = line.as_ref().chars().collect();
            let mut start = 0usize;

            for g in grids.iter_mut() {
                let end = start + g.width;
                let mut row = vec![None; g.width];

                for idx in start..end {
                    let c = chars[idx];
                    if c == ' ' {
                        continue;
                    }

                    let d = c.to_digit(10).ok_or_else(|| {
                        AdventError::InputParseError(
                            format!("expected digit or space, got {c:?}").into(),
                        )
                    })?;
                    row[idx - start] = Some(d);
                }

                start = end + 1;
                g.nums.push(row);
            }
        }

        let groups = grids
            .iter()
            .map(|g| {
                let nums: Vec<u32> = (0..g.width)
                    .map(|col| {
                        let mut agg = 0u32;
                        for row in 0..(n - 1) {
                            if let Some(d) = g.nums[row][col] {
                                agg = agg * 10 + d;
                            }
                        }
                        agg
                    })
                    .collect();

                NumberGroup {
                    operator: g.operator,
                    nums,
                }
            })
            .collect();

        Ok(Self { groups })
    }
}

/// Parses `"*   +   *   +  "` into operator + column widths (space runs between operators).
fn parse_cephalopod_operator_layout(last_line: &str) -> Result<Vec<ColumnSpec>, AdventError> {
    let chars: Vec<char> = last_line.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut specs = Vec::new();

    while i < len {
        if chars[i] == ' ' {
            i += 1;
            continue;
        }

        let operator = Operator::try_from(chars[i])?;
        i += 1;

        let mut num_spaces = 0usize;
        while i < len && chars[i] == ' ' {
            num_spaces += 1;
            i += 1;
        }

        // No spaces after the last operator on the line: the block still has one digit column.
        if i >= len {
            num_spaces += 1;
        }

        specs.push(ColumnSpec {
            operator,
            width: num_spaces,
        });
    }

    Ok(specs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let lines = vec![
            "123 328  51 64",
            "45 64  387 23 ",
            "6 98  215 314",
            "*   +   *   + ",
        ];

        let hwk = MathHomework::try_from(lines).expect("bad input");

        assert_eq!(4277556, homework_total(&hwk.groups));
    }

    #[test]
    fn sample_part_2() {
        let lines = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ];

        let hwk = CephalopodHomework::try_from(lines).expect("bad input");

        assert_eq!(3263827, homework_total(&hwk.groups));
    }

    #[test]
    fn cephalopod_operator_layout_parses_operators_and_widths() {
        let last = "*   +   *   +  ";
        let specs = parse_cephalopod_operator_layout(last).expect("layout");
        assert_eq!(specs.len(), 4);
        assert_eq!(specs[0].width, 3);
        assert_eq!(specs[1].width, 3);
        assert_eq!(specs[2].width, 3);
        assert_eq!(specs[3].width, 3);
        assert!(matches!(specs[0].operator, Operator::Multiply));
        assert!(matches!(specs[1].operator, Operator::Add));
    }
}
