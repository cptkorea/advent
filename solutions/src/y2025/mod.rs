use super::{AdventError, AdventProblem};

advent_common::define_advent_registry!();

pub(crate) fn run_with_lines(
    lines: Vec<String>,
    date: u8,
    part: u8,
) -> Result<u32, AdventError> {
    factory(date).run(lines, part)
}
