use std::borrow::Cow;
use std::fs::File;
use std::{io, io::BufRead};
use thiserror::Error;

mod day1;
mod day2;
mod day3;
mod day4;

pub fn factory(date: u8) -> Box<dyn AdventProblem> {
    match date {
        1 => Box::new(day1::Day1),
        2 => Box::new(day2::Day2),
        3 => Box::new(day3::Day3),
        4 => Box::new(day4::Day4),
        _ => unimplemented!(),
    }
}

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
}

fn read_input_file(year: u32, date: u8) -> Result<Vec<String>, io::Error> {
    let mut lines = Vec::new();
    let file = File::open(format!("./y{}/inputs/day{}.txt", year, date))?;
    for next in io::BufReader::new(file).lines() {
        match next {
            Ok(line) => lines.push(line),
            Err(err) => return Err(err),
        }
    }
    Ok(lines)
}

pub fn run(year: u32, date: u8, part: u8) -> Result<u32, AdventError> {
    let lines = read_input_file(year, date)?;
    run_with_lines(lines, date, part)
}

pub fn run_with_lines(lines: Vec<String>, date: u8, part: u8) -> Result<u32, AdventError> {
    let d = factory(date);
    d.run(lines, part)
}
