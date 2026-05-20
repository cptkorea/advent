//! Shared helpers for Advent of Code workspace crates.
//!
//! Each day module must expose `pub struct DayN` adjacent to [`define_advent_registry`] (e.g.
//! `crate::y2024::day23::Day23`). The invoking crate must depend on the `paste` crate.

use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PuzzleAnswer {
    USIZE(usize),
    U32(u32),
    U64(u64),
    Str(String),
}

impl fmt::Display for PuzzleAnswer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PuzzleAnswer::USIZE(u) => write!(f, "{u}"),
            PuzzleAnswer::U32(v) => write!(f, "{v}"),
            PuzzleAnswer::U64(v) => write!(f, "{v}"),
            PuzzleAnswer::Str(s) => write!(f, "{s}"),
        }
    }
}

impl From<u32> for PuzzleAnswer {
    fn from(v: u32) -> Self {
        Self::U32(v)
    }
}

impl From<u64> for PuzzleAnswer {
    fn from(v: u64) -> Self {
        Self::U64(v)
    }
}

impl From<String> for PuzzleAnswer {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl From<usize> for PuzzleAnswer {
    fn from(u: usize) -> Self {
        Self::USIZE(u)
    }
}

pub trait AdventProblem {
    type Answer: Into<PuzzleAnswer>;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError>;
    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError>;

    fn run(&self, lines: Vec<String>, part: u8) -> Result<PuzzleAnswer, AdventError> {
        match part {
            1 => self.run_part_1(lines).map(Into::into),
            2 => self.run_part_2(lines).map(Into::into),
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

impl From<ParseIntError> for AdventError {
    fn from(err: ParseIntError) -> Self {
        Self::InputParseError(err.to_string().into())
    }
}

/// Reads puzzle lines from `{crate_root}/src/y{year}/inputs/day{date}.txt`.
///
/// Pass the solving crate root (typically `PathBuf::from(env!("CARGO_MANIFEST_DIR"))` from `solutions`).
pub fn read_input_lines(
    crate_root: impl AsRef<std::path::Path>,
    year: u32,
    date: u8,
) -> Result<Vec<String>, AdventError> {
    let path = PathBuf::from(crate_root.as_ref())
        .join("src")
        .join(format!("y{}", year))
        .join("inputs")
        .join(format!("day{}.txt", date));
    let mut lines = Vec::new();
    let file = File::open(path)?;
    for next in io::BufReader::new(file).lines() {
        lines.push(next?);
    }
    Ok(lines)
}

#[macro_export]
macro_rules! define_advent_registry {
    ($($day:literal),* $(,)*) => {
        $(
            ::paste::paste! {
                mod [<day $day>];
            }
        )*

        pub(crate) fn run_registered_day(
            lines: ::std::vec::Vec<::std::string::String>,
            date: u8,
            part: u8,
        ) -> ::std::result::Result<$crate::PuzzleAnswer, $crate::AdventError> {
            match date {
                $(
                    $day => ::paste::paste! {
                        <[<day $day>]::[<Day $day>] as $crate::AdventProblem>::run(
                            &[<day $day>]::[<Day $day>],
                            lines,
                            part,
                        )
                    },
                )*
                _ => ::core::unimplemented!(),
            }
        }
    };
}

pub mod arithmetic;
pub mod floodfill;
pub mod number;
pub mod range;
pub mod rotation;
pub mod ufind;
