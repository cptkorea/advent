use crate::{AdventError, AdventProblem};
use computer::Computer;

mod computer;

pub struct Day17;

impl AdventProblem for Day17 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut computer = parse_lines(&lines);
        computer.run();
        println!("{:?}", computer.outputs().iter());
        Ok(0)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        Ok(0)
    }
}

fn parse_lines(lines: &Vec<String>) -> Computer {
    let a = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .expect("numeric register value");

    let b = lines[1]
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .expect("numeric register value");

    let c = lines[2]
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .expect("numeric register value");

    let instructions = lines[4]
        .split(": ")
        .nth(1)
        .unwrap()
        .split(",")
        .into_iter()
        .map(|s| s.parse().expect("numeric opcode"))
        .collect::<Vec<u8>>();

    Computer::new(instructions, a, b, c)
}
