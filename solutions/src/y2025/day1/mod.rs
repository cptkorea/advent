use crate::{AdventError, AdventProblem};
use advent_common::rotation::Rotation;

struct Turn {
    direction: Rotation,
    magnitude: usize,
}

impl TryFrom<&String> for Turn {
    type Error = AdventError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(AdventError::InputParseError("line is empty".into()));
        }

        let bytes = value.as_bytes();

        let dir = match bytes[0] {
            b'R' => Rotation::Clockwise,
            b'L' => Rotation::CounterClockwise,
            c => {
                return Err(AdventError::InputParseError(
                    format!("first character {c} is not R/L").into(),
                ));
            }
        };

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
                        pos -= mag;
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
            let m = turn.magnitude;

            cnt += m / NUM_DIALS;
            let mag = m % NUM_DIALS;

            match turn.direction {
                Rotation::Clockwise => {
                    // Remainder: extra hit iff we land on/pass 0 once more before the lap completes.
                    if (pos + mag) >= NUM_DIALS {
                        cnt += 1;
                    }
                    // (pos + q * NUM_DIALS + mag) mod NUM_DIALS == (pos + mag) mod NUM_DIALS
                    pos = (pos + mag) % NUM_DIALS;
                }
                Rotation::CounterClockwise => {
                    if pos == 0 {
                        // From 0, hits occur at ticks NUM_DIALS, 2*NUM_DIALS, … within [1..mag].
                        cnt += mag / NUM_DIALS;
                    } else if mag >= pos {
                        // Hits at pos, pos+NUM_DIALS, … up to mag.
                        cnt += 1 + (mag - pos) / NUM_DIALS;
                    }

                    if mag > pos {
                        pos = NUM_DIALS + pos - mag;
                    } else {
                        pos -= mag;
                    }
                }
            }
        }

        Ok(cnt as u32)
    }
}
