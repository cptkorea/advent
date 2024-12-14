use crate::{AdventError, AdventProblem};

pub struct Day13;

impl AdventProblem for Day13 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let games = parse_lines(&lines, 0);
        let total = games
            .iter()
            .map(|g| match g.solve() {
                Some((a, b)) => 3 * (a as u64) + (b as u64),
                None => 0,
            })
            .sum::<u64>();

        println!("total={}", total);
        Ok(0)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let games = parse_lines(&lines, 10000000000000);
        let total = games
            .iter()
            .map(|g| match g.solve() {
                Some((a, b)) => 3 * (a as u64) + (b as u64),
                None => 0,
            })
            .sum::<u64>();

        println!("total={}", total);
        Ok(0)
    }
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        use {regex::Regex, std::sync::OnceLock};

        static RE: OnceLock<Regex> = OnceLock::new();
        RE.get_or_init(|| Regex::new($re).unwrap())
    }};
}

fn parse_lines(lines: &Vec<String>, offset: u64) -> Vec<Game> {
    let mut i = 0;
    let n = lines.len();

    let button_a_regex = regex!("Button A: X\\+(\\d+), Y\\+(\\d+)");
    let button_b_regex = regex!("Button B: X\\+(\\d+), Y\\+(\\d+)");
    let prize_regex = regex!("Prize: X=(\\d+), Y=(\\d+)");

    let mut games = Vec::new();

    while i < n {
        let (_, [x_a, y_a]) = button_a_regex.captures(&lines[i]).unwrap().extract();
        let (_, [x_b, y_b]) = button_b_regex.captures(&lines[i + 1]).unwrap().extract();
        let (_, [p_a, p_b]) = prize_regex.captures(&lines[i + 2]).unwrap().extract();

        let (x_a, y_a) = (x_a.parse().unwrap(), y_a.parse().unwrap());
        let (x_b, y_b) = (x_b.parse().unwrap(), y_b.parse().unwrap());
        let (p_a, p_b) = (p_a.parse::<u64>().unwrap(), p_b.parse::<u64>().unwrap());

        games.push(Game {
            button_a: (x_a, y_a),
            button_b: (x_b, y_b),
            prize: (p_a + offset as u64, p_b + offset as u64),
        });

        i += 4;
    }

    games
}

struct Game {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl Game {
    /// solve the system of equations using the matrix inverse formula
    /// verify that the solution is valid through multiplication instead of dividing floats
    fn solve(&self) -> Option<(i64, i64)> {
        let (a, b, c, d) = (
            self.button_a.0 as i64,
            self.button_b.0 as i64,
            self.button_a.1 as i64,
            self.button_b.1 as i64,
        );

        let factor = a * d - b * c;
        let (p_a, p_b) = (self.prize.0 as i64, self.prize.1 as i64);

        let s_a = (d * p_a - b * p_b) / factor;
        let s_b = (-c * p_a + a * p_b) / factor;

        if a * s_a + b * s_b == p_a && c * s_a + d * s_b == p_b {
            Some((s_a, s_b))
        } else {
            None
        }
    }
}
