use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use thiserror::Error;

pub trait AdventProblem {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError>;
    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError>;

    fn run(&self, lines: Vec<String>, part: u8) -> Result<u32, AdventError> {
        match part {
            1 => self.run_part_1(lines),
            2 => self.run_part_2(lines),
            _ => unimplemented!("part {}", part),
        }
    }
}

#[derive(Error, Debug)]
pub enum AdventError {
    #[error("{0}")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    InputParseError(Cow<'static, str>),
    #[error("unknown error")]
    UnknownError,
}

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

fn read_input_lines(year: u32, date: u8) -> Result<Vec<String>, AdventError> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(format!("y{}", year))
        .join(format!("day{}.txt", date));
    let mut lines = Vec::new();
    let file = File::open(path)?;
    for next in io::BufReader::new(file).lines() {
        lines.push(next?);
    }
    Ok(lines)
}

pub fn run(year: u32, date: u8, part: u8) -> Result<u32, AdventError> {
    let lines = read_input_lines(year, date)?;
    match year {
        2023 => y2023::run_with_lines(lines, date, part),
        2024 => y2024::run_with_lines(lines, date, part),
        2025 => y2025::run_with_lines(lines, date, part),
        _ => Err(AdventError::InputParseError(Cow::Borrowed(
            "unsupported year",
        ))),
    }
}
