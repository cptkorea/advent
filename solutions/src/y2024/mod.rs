use super::AdventError;

advent_common::define_advent_registry!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
);

pub(crate) fn run_with_lines(lines: Vec<String>, date: u8, part: u8) -> Result<u32, AdventError> {
    factory(date).run(lines, part)
}
