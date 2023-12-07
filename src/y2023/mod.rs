use crate::{AdventError, AdventProblem};

mod day1;
mod day2;

pub fn run(lines: Vec<String>, date: u8, part: u8) -> Result<u32, AdventError> {
    let d = factory(date);
    d.run(lines, part)
}

pub fn factory(date: u8) -> Box<dyn AdventProblem> {
    match date {
        1 => Box::new(day1::Day1),
        2 => Box::new(day2::Day2),
        _ => unimplemented!(),
    }
}
