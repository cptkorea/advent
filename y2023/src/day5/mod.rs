use crate::{AdventError, AdventProblem};
pub struct Day5;

impl AdventProblem for Day5 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| ScratchCard::from(s.as_str()).score())
            .sum();
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let cards = lines
            .iter()
            .map(|s| ScratchCard::from(s.as_str()))
            .collect::<Vec<_>>();

        let mut total = 0;
        let mut buffer = HashMap::new();
    }
}

struct Alamanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

struct Range {
    dest_start: u32,
    source_start: u32,
    len: u32,
}

impl Range {
    fn map(&self, n: u32) -> u32 {
        if self.source_start <= n && n <= self.source_start + self.len {
            self.dest_start + (n - self.source_start)
        } else {
            n
        }
    }
}
