use crate::{AdventError, AdventProblem};

pub struct Day2;

impl AdventProblem for Day2 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| Game::try_from(s.as_str()).unwrap())
            .filter(|g| g.is_valid())
            .fold(0, |t, g| t + g.id);
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        Ok(1)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    samples: Vec<Sample>,
}

impl TryFrom<&str> for Game {
    type Error = AdventError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if &s[..4] != "Game" {
            return Err(AdventError::InputParseError("missing game header".into()));
        }

        let idx = s
            .find(":")
            .ok_or(AdventError::InputParseError("missing semicolon".into()))?;
        let id = (&s[5..idx])
            .parse::<u32>()
            .map_err(|e| AdventError::InputParseError(e.to_string().into()))?;

        let mut samples = Vec::new();
        for samp in (&s[idx + 1..]).split(";") {
            samples.push(Sample::try_from(samp)?);
        }

        Ok(Self { id, samples })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        self.samples.iter().all(|s| s.is_valid())
    }
}

#[derive(Debug, Default)]
pub struct Sample {
    red: u32,
    blue: u32,
    green: u32,
}

impl TryFrom<&str> for Sample {
    type Error = AdventError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut sample = Sample::default();
        for c in s.trim().split(",") {
            let mut tokens = c.trim().split(" ");
            let cnt = tokens
                .next()
                .unwrap()
                .parse::<u32>()
                .map_err(|e| AdventError::InputParseError(e.to_string().into()))?;

            match tokens.next().unwrap() {
                "red" => sample.red = cnt,
                "green" => sample.green = cnt,
                "blue" => sample.blue = cnt,
                c => unimplemented!("unknown color {}", c),
            }
        }
        Ok(sample)
    }
}

impl Sample {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
