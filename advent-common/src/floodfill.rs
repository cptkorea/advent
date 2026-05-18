//! Shared helpers to run a [FloodFill](https://en.wikipedia.org/wiki/Flood_fill) algorithm
//!

pub const EIGHT_DIRECTIONS: [EightWayDirection; 8] = [
    EightWayDirection::North,
    EightWayDirection::South,
    EightWayDirection::East,
    EightWayDirection::West,
    EightWayDirection::NorthWest,
    EightWayDirection::NorthEast,
    EightWayDirection::SouthWest,
    EightWayDirection::SouthEast,
];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EightWayDirection {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl EightWayDirection {
    fn offset(self) -> (i32, i32) {
        match self {
            Self::North => (0, 1),
            Self::South => (0, -1),
            Self::East => (1, 0),
            Self::West => (-1, 0),
            Self::NorthWest => (-1, -1),
            Self::NorthEast => (-1, 1),
            Self::SouthWest => (1, -1),
            Self::SouthEast => (1, 1),
        }
    }
}

pub fn try_transition(
    pos: (usize, usize),
    boundary: (usize, usize),
    dir: EightWayDirection,
) -> Option<(usize, usize)> {
    let (dr, dc) = dir.offset();
    let (row, col) = pos;
    let (m, n) = boundary;

    if (row == m - 1 && dr > 0) || (col == n - 1 && dc > 0) {
        return None;
    }

    let nr = if dr >= 0 {
        row.checked_add(dr as usize)
    } else {
        row.checked_sub((-dr) as usize)
    };

    let nc = if dc >= 0 {
        col.checked_add(dc as usize)
    } else {
        col.checked_sub((-dc) as usize)
    };

    match (nr, nc) {
        (Some(nr), Some(nc)) => Some((nr, nc)),
        _ => None,
    }
}

pub fn construct_grid<S: ToString>(lines: &[S]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|s| s.to_string().chars().collect())
        .collect()
}
