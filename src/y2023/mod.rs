use crate::{AdventError, AdventProblem};

mod day1;

pub fn run(lines: Vec<String>, date: u8, part: u8) -> Result<u32, AdventError> {
    let d = factory(date);
    d.run(lines, part)
}

pub fn factory(date: u8) -> impl AdventProblem {
    match date {
        1 => day1::Day1,
        _ => unimplemented!(),
    }
}
