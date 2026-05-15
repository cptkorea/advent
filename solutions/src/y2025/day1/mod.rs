use crate::{AdventError, AdventProblem};
use advent_common::rotation::Rotation;

struct Turn {
    direction: Rotation,
    magnitude: usize,
}

impl TryFrom<&String> for Turn {
    type Error = AdventError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err(AdventError::InputParseError("line is empty".into()));
        }

        let bytes = value.as_bytes();
        let dir;

        match bytes[0] {
            b'R' => {
                dir = Rotation::Clockwise;
            }
            b'L' => {
                dir = Rotation::CounterClockwise;
            }
            c => {
                return Err(AdventError::InputParseError(
                    format!("first character {c} is not R/L").into(),
                ))
            }
        }

        let rem = &value[1..];
        let mag = rem.parse::<usize>().map_err(|_e| {
            AdventError::InputParseError(format!("val {rem} is not a number").into())
        })?;

        Ok(Turn {
            direction: dir,
            magnitude: mag,
        })
    }
}

pub struct Day1;

const NUM_DIALS: usize = 100;

/// Count of clicks `k` in `1..=steps` where the ticker is on `0`, starting `pos ∈ [0, N)`,
/// moving clockwise one tick along `0..N-1`.
fn tick_landings_on_zero_cw(pos: usize, steps: usize, n: usize) -> usize {
    if steps == 0 || n == 0 {
        return 0;
    }
    let pos = pos % n;
    let first = if pos == 0 { n } else { n - pos };
    if steps < first {
        0
    } else {
        1 + (steps - first) / n
    }
}

/// Same for counter-clockwise (one tick decreases `mod n`, hits `0` on step `pos`, `pos+n`, …).
fn tick_landings_on_zero_ccw(pos: usize, steps: usize, n: usize) -> usize {
    if steps == 0 || n == 0 {
        return 0;
    }
    let pos = pos % n;
    let first = if pos == 0 { n } else { pos };
    if steps < first {
        0
    } else {
        1 + (steps - first) / n
    }
}

fn pos_after_ccw(pos: usize, steps: usize, n: usize) -> usize {
    (pos as i128 - steps as i128).rem_euclid(n as i128) as usize
}

impl AdventProblem for Day1 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut pos = 50;
        let mut cnt = 0;

        for line in &lines {
            let turn = Turn::try_from(line)?;
            let mag = turn.magnitude % 100;
            match turn.direction {
                Rotation::Clockwise => pos = (pos + mag) % NUM_DIALS,
                Rotation::CounterClockwise => {
                    if mag > pos {
                        pos = NUM_DIALS + pos - mag;
                    } else {
                        pos = pos - mag;
                    }
                }
            }

            if pos == 0 {
                cnt += 1;
            }
        }

        Ok(cnt as u32)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut pos = 50;
        let mut cnt = 0;

        for line in &lines {
            let turn = Turn::try_from(line)?;
            match turn.direction {
                Rotation::Clockwise => {
                    cnt += tick_landings_on_zero_cw(pos, turn.magnitude, NUM_DIALS);
                    pos = (pos + turn.magnitude) % NUM_DIALS;
                }
                Rotation::CounterClockwise => {
                    cnt += tick_landings_on_zero_ccw(pos, turn.magnitude, NUM_DIALS);
                    pos = pos_after_ccw(pos, turn.magnitude, NUM_DIALS);
                }
            }
        }

        Ok(cnt as u32)
    }
}
