use crate::{AdventError, AdventProblem};

macro_rules! regex {
    ($re:literal $(,)?) => {{
        use {regex::Regex, std::sync::OnceLock};

        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

pub struct Day2;

impl AdventProblem for Day2 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| Game::try_from(s.as_str()).expect("unable to parse game"))
            .filter(|g| g.bound.is_valid())
            .fold(0, |t, g| t + g.id);
        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let total = lines
            .iter()
            .map(|s| Game::try_from(s.as_str()).unwrap())
            .fold(0, |t, g| t + g.bound.power());
        Ok(total)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    bound: Bound,
}

impl TryFrom<&str> for Game {
    type Error = AdventError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if &s[..4] != "Game" {
            return Err(AdventError::InputParseError("missing game header".into()));
        }

        let (_, [id, remainder]) = regex!("Game (?<id>\\d+): (?<remainder>.*)")
            .captures(s)
            .ok_or_else(|| AdventError::InputParseError("unable to find game record".into()))?
            .extract();

        let id = id.parse::<u32>().expect("game id should be numeric");
        let best_sample = Bound::try_from(remainder)?;

        Ok(Self {
            id,
            bound: best_sample,
        })
    }
}

#[derive(Debug, Default)]
pub struct Bound {
    red: u32,
    green: u32,
    blue: u32,
}

impl TryFrom<&str> for Bound {
    type Error = AdventError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        let samples = s.split(";");
        for event in samples {
            for pick in event.split(",") {
                let mut tokens = pick.trim().split(" ");
                let cnt = tokens
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .map_err(|e| AdventError::InputParseError(e.to_string().into()))?;

                match tokens.next().unwrap() {
                    "red" => red = std::cmp::max(cnt, red),
                    "green" => green = std::cmp::max(cnt, green),
                    "blue" => blue = std::cmp::max(cnt, blue),
                    c => unimplemented!("unknown color {}", c),
                }
            }
        }
        Ok(Bound { red, green, blue })
    }
}

impl Bound {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_sample() {
        let b: Bound = Bound::try_from("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();

        assert_eq!(b.red, 4);
        assert_eq!(b.green, 2);
        assert_eq!(b.blue, 6);
    }

    #[test]
    fn parse_game() {
        let g = Game::try_from(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        )
        .unwrap();

        assert_eq!(g.id, 4);
        assert_eq!(g.bound.red, 14);
        assert_eq!(g.bound.green, 3);
        assert_eq!(g.bound.blue, 15);
    }
}
