use crate::{AdventError, AdventProblem};
use std::collections::{HashMap, HashSet};

macro_rules! regex {
    ($re:literal $(,)?) => {{
        use {regex::Regex, std::sync::OnceLock};

        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

pub struct Day4;

impl AdventProblem for Day4 {
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

        for i in 1..=cards.len() {
            buffer.insert(i as u32, 1);
        }

        for card in cards {
            let num_cards = *buffer.get(&card.id).unwrap_or(&0);
            total += num_cards;
            for i in 1..=card.matches() {
                buffer
                    .entry(card.id + i)
                    .and_modify(|e| *e = *e + num_cards)
                    .or_insert(2);
            }
        }

        Ok(total)
    }
}

struct ScratchCard {
    id: u32,
    winning_nums: HashSet<u32>,
    draw: HashSet<u32>,
}

impl ScratchCard {
    fn matches(&self) -> u32 {
        let mut matches = 0;
        for num in &self.draw {
            if self.winning_nums.contains(num) {
                matches += 1;
            }
        }
        matches
    }

    fn score(&self) -> u32 {
        let num_matches = self.matches();
        if num_matches == 0 {
            0
        } else {
            u32::pow(2, num_matches - 1)
        }
    }
}

impl From<&str> for ScratchCard {
    fn from(s: &str) -> Self {
        let (_, [id, winning, draw]) =
            regex!("Card\\s+(\\d+): (?<winning>[\\d\\s]+) \\| (?<draw>[\\d\\s]+)")
                .captures(s)
                .expect("scratchcard should have winning and draw numbers separated by |")
                .extract();

        ScratchCard {
            id: id.parse().expect("id must be numeric"),
            winning_nums: parse_numeric_sequence(winning),
            draw: parse_numeric_sequence(draw),
        }
    }
}

fn parse_numeric_sequence(s: &str) -> HashSet<u32> {
    let mut nums = HashSet::new();
    for part in s.trim().split(" ") {
        if let Ok(num) = part.parse::<u32>() {
            nums.insert(num);
        }
    }
    nums
}
