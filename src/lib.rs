use std::borrow::Cow;
use std::fs::File;
use std::{io, io::BufRead};
use thiserror::Error;

mod y2023;

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
    #[error("i/o error")]
    IoError(#[from] io::Error),
    #[error("{0}")]
    InputParseError(Cow<'static, str>),
}

fn read_input(year: u32, date: u8) -> Result<Vec<String>, io::Error> {
    let mut lines = Vec::new();

    let file = File::open(format!("./src/y{}/day{}/input.txt", year, date))?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            lines.push(line);
        }
    }
    Ok(lines)
}

pub fn run(year: u32, date: u8, part: u8) -> Result<u32, AdventError> {
    let lines = read_input(year, date)?;
    y2023::run(lines, date, part)
}
