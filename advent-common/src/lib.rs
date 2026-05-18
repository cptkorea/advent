//! Shared helpers for Advent of Code workspace crates.
//!
//! Each day module must expose `pub struct DayN` adjacent to the invocation (e.g.
//! `crate::y2024::day23::Day23`). The invoking crate must depend on the `paste` crate.

use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufRead};
use std::num::ParseIntError;
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

impl From<ParseIntError> for AdventError {
    fn from(err: ParseIntError) -> Self {
        Self::InputParseError(err.to_string().into())
    }
}

/// Reads puzzle lines from `{inputs_root}/inputs/y{year}/day{date}.txt`.
pub fn read_input_lines(
    inputs_root: impl AsRef<std::path::Path>,
    year: u32,
    date: u8,
) -> Result<Vec<String>, AdventError> {
    let path = PathBuf::from(inputs_root.as_ref())
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

#[macro_export]
macro_rules! define_advent_registry {
    ($($day:literal),* $(,)*) => {
        $(
            ::paste::paste! {
                mod [<day $day>];
            }
        )*

        pub fn factory(date: u8) -> ::std::boxed::Box<dyn $crate::AdventProblem> {
            match date {
                $(
                    $day => ::std::boxed::Box::new(::paste::paste! {
                        [<day $day>]::[<Day $day>]
                    }),
                )*
                _ => ::core::unimplemented!(),
            }
        }
    };
}

pub mod number;
pub mod rotation;
