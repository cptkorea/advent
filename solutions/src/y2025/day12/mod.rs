//! Day 12 input: numbered tile bitmaps (`id:` + `#`/`.` grid) plus `WxH:` scalar rows.

use crate::{AdventError, AdventProblem};
use std::borrow::Cow;

pub struct Day12;

impl AdventProblem for Day12 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let puzzle = Puzzle::try_from_lines(&lines)?;

        let mut cnt = 0;
        for row in &puzzle.sized_rows {
            let cell_req = row
                .values
                .iter()
                .enumerate()
                .map(|(i, &freq)| puzzle.tile_blocks[i].size * freq)
                .sum::<usize>();

            if cell_req <= row.height * row.width {
                cnt += 1;
            }
        }

        Ok(cnt)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let _data = Puzzle::try_from_lines(&lines)?;
        Ok(0)
    }
}

/// Full parsed problem input: fixed tile patterns and dimension-tagged number rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    /// bitmaps keyed by numeric id (`0:` … `5:` in the puzzle)
    pub tile_blocks: Vec<TileBlock>,
    /// lines like `4x4: 0 0 0 0 2 0`
    pub sized_rows: Vec<SizedRow>,
}

/// One tile bitmap: `id:` then a rectangle of `#` (set) / `.` (clear).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TileBlock {
    pub id: u32,
    /// Number of `#` cells in `bitmap`.
    pub size: usize,
    pub bitmap: Vec<Vec<bool>>,
}

/// One `width x height :` line with trailing integers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizedRow {
    pub width: usize,
    pub height: usize,
    pub values: Vec<usize>,
}

impl Puzzle {
    /// Parse full text (e.g. puzzle file contents).
    pub fn parse(input: &str) -> Result<Self, AdventError> {
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0usize;
        let mut tile_blocks = Vec::new();
        let mut sized_rows = Vec::new();

        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() {
                i += 1;
                continue;
            }

            if let Some(sr) = parse_sized_row_line(line)? {
                sized_rows.push(sr);
                i += 1;
                continue;
            }

            if let Some(id) = parse_tile_id_header(line) {
                i += 1;
                while i < lines.len() && lines[i].trim().is_empty() {
                    i += 1;
                }
                let mut bitmap = Vec::new();
                while i < lines.len() {
                    let row = lines[i].trim();
                    if row.is_empty() {
                        break;
                    }
                    if is_section_header(row) {
                        break;
                    }
                    bitmap.push(parse_bitmap_row(row)?);
                    i += 1;
                }
                if bitmap.is_empty() {
                    return Err(invalid("tile block has no grid rows"));
                }
                let w = bitmap[0].len();
                if bitmap.iter().any(|r| r.len() != w) {
                    return Err(invalid("tile grid rows have different widths"));
                }

                let hash_count = bitmap.iter().flatten().filter(|&&cell| cell).count();

                tile_blocks.push(TileBlock {
                    id,
                    size: hash_count,
                    bitmap,
                });
                continue;
            }

            return Err(invalid(format!("unrecognized line: {line:?}")));
        }

        Ok(Puzzle {
            tile_blocks,
            sized_rows,
        })
    }

    pub fn try_from_lines<S: AsRef<str>>(lines: &[S]) -> Result<Self, AdventError> {
        Self::parse(
            &lines
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

fn invalid(msg: impl Into<Cow<'static, str>>) -> AdventError {
    AdventError::InputParseError(msg.into())
}

/// `WxH: v0 v1 ...`
fn parse_sized_row_line(line: &str) -> Result<Option<SizedRow>, AdventError> {
    let Some(colon) = line.find(':') else {
        return Ok(None);
    };
    let (lhs, rhs) = line.split_at(colon);
    let rhs = rhs[1..].trim();
    let lhs = lhs.trim();
    let Some((w, h)) = lhs.split_once('x').or_else(|| lhs.split_once('X')) else {
        return Ok(None);
    };
    if w.is_empty()
        || h.is_empty()
        || !w.chars().all(|c| c.is_ascii_digit())
        || !h.chars().all(|c| c.is_ascii_digit())
    {
        return Ok(None);
    }
    let width: usize = w.parse().map_err(|_| invalid("bad width in WxH line"))?;
    let height: usize = h.parse().map_err(|_| invalid("bad height in WxH line"))?;

    let values: Vec<usize> = if rhs.is_empty() {
        Vec::new()
    } else {
        rhs.split_whitespace()
            .map(|t| t.parse().map_err(|_| invalid("bad integer in WxH line")))
            .collect::<Result<Vec<_>, _>>()?
    };

    Ok(Some(SizedRow {
        width,
        height,
        values,
    }))
}

/// Plain `id:` with no `x` before the colon (distinct from `12x5:`).
fn parse_tile_id_header(line: &str) -> Option<u32> {
    let line = line.trim();
    let id_part = line.strip_suffix(':')?;
    if id_part.contains('x') || id_part.contains('X') {
        return None;
    }
    id_part.parse::<u32>().ok()
}

fn is_section_header(trimmed: &str) -> bool {
    parse_tile_id_header(trimmed).is_some() || matches!(parse_sized_row_line(trimmed), Ok(Some(_)))
}

fn parse_bitmap_row(line: &str) -> Result<Vec<bool>, AdventError> {
    line.chars()
        .map(|c| match c {
            '#' => Ok(true),
            '.' => Ok(false),
            _ => Err(invalid(format!("invalid tile character: {c:?}"))),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn parse_sample() {
        let p = Puzzle::parse(SAMPLE).expect("parse");

        assert_eq!(p.tile_blocks.len(), 6);

        assert_eq!(p.tile_blocks[0].id, 0);
        assert_eq!(
            p.tile_blocks[0].bitmap,
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, true, false],
            ]
        );

        assert_eq!(p.tile_blocks[0].size, 7);

        assert_eq!(p.tile_blocks[1].id, 1);
        assert_eq!(p.tile_blocks[1].size, 7);
        assert_eq!(p.tile_blocks[1].bitmap[2], vec![false, true, true]);

        assert_eq!(p.tile_blocks[2].size, 7);

        assert_eq!(p.tile_blocks[3].size, 7);

        assert_eq!(p.tile_blocks[4].size, 7);

        assert_eq!(p.tile_blocks[5].id, 5);
        assert_eq!(p.tile_blocks[5].size, 7);
        assert_eq!(
            p.tile_blocks[5].bitmap,
            vec![
                vec![true, true, true],
                vec![false, true, false],
                vec![true, true, true],
            ]
        );

        assert_eq!(p.sized_rows.len(), 3);
        assert_eq!(
            p.sized_rows[0],
            SizedRow {
                width: 4,
                height: 4,
                values: vec![0, 0, 0, 0, 2, 0],
            }
        );
        assert_eq!(
            p.sized_rows[1],
            SizedRow {
                width: 12,
                height: 5,
                values: vec![1, 0, 1, 0, 2, 2],
            }
        );
        assert_eq!(
            p.sized_rows[2],
            SizedRow {
                width: 12,
                height: 5,
                values: vec![1, 0, 1, 0, 3, 2],
            }
        );
    }
}
