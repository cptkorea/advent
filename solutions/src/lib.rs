use std::borrow::Cow;
use std::path::PathBuf;

pub use advent_common::{AdventError, AdventProblem, PuzzleAnswer};

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        use {regex::Regex, std::sync::OnceLock};

        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

pub mod y2023;
pub mod y2024;
pub mod y2025;

pub fn run(year: u32, date: u8, part: u8) -> Result<PuzzleAnswer, AdventError> {
    let lines =
        advent_common::read_input_lines(PathBuf::from(env!("CARGO_MANIFEST_DIR")), year, date)?;
    match year {
        2023 => y2023::run_with_lines(lines, date, part),
        2024 => y2024::run_with_lines(lines, date, part),
        2025 => y2025::run_with_lines(lines, date, part),
        _ => Err(AdventError::InputParseError(Cow::Borrowed(
            "unsupported year",
        ))),
    }
}
