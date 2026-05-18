use super::{AdventError, PuzzleAnswer};

advent_common::define_advent_registry!(1, 2, 3);

pub(crate) fn run_with_lines(
    lines: Vec<String>,
    date: u8,
    part: u8,
) -> Result<PuzzleAnswer, AdventError> {
    run_registered_day(lines, date, part)
}
