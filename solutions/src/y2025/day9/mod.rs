use crate::{AdventError, AdventProblem};
use advent_common::{direction::OrdinalDirection, number::Pair};
use std::collections::HashSet;

type Point = Pair<usize, usize>;

pub struct Day9;

impl AdventProblem for Day9 {
    type Answer = u64;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let red_tiles = lines
            .iter()
            .map(|s| Pair::try_from(s.as_str()))
            .collect::<Result<Vec<_>, AdventError>>()?;

        let mut max_area = 0;
        for (i, t1) in red_tiles.iter().enumerate() {
            for t2 in red_tiles.iter().skip(i + 1) {
                let dx = u64::abs_diff(t1.first, t2.first) + 1;
                let dy = u64::abs_diff(t1.second, t2.second) + 1;

                max_area = std::cmp::max(dx * dy, max_area);
            }
        }

        Ok(max_area)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let vertices = lines
            .iter()
            .map(|s| Pair::try_from(s.as_str()))
            .collect::<Result<Vec<_>, AdventError>>()?;

        let boundary = compute_boundary(&vertices);

        let mut pairwise_areas = compute_areas(&vertices);
        pairwise_areas.sort_by_key(|(a, _, _)| std::cmp::Reverse(*a));

        let mut max_area = 0;
        for (area, p1, p2) in pairwise_areas {
            if rectangle_check(p1, p2, &boundary) {
                max_area = area;
                break;
            }
        }

        Ok(max_area)
    }
}

fn compute_boundary(tiles: &[Point]) -> HashSet<Point> {
    let mut boundary = HashSet::new();

    if tiles.is_empty() {
        return boundary;
    }

    let segment = |a: Point, b: Point, boundary: &mut HashSet<Point>| {
        if a.first == b.first {
            let s = a.second.min(b.second);
            let e = a.second.max(b.second);
            for i in s..=e {
                boundary.insert(Point::new(a.first, i));
            }
        } else if a.second == b.second {
            let s = a.first.min(b.first);
            let e = a.first.max(b.first);
            for i in s..=e {
                boundary.insert(Point::new(i, a.second));
            }
        }
    };

    let mut prev = tiles[0];
    for &next in tiles.iter().skip(1) {
        segment(prev, next, &mut boundary);
        prev = next;
    }
    segment(prev, tiles[0], &mut boundary);

    boundary
}

/// Compute areas for all cells corner pairs
fn compute_areas(vertices: &[Point]) -> Vec<(u64, &Point, &Point)> {
    let n = vertices.len();
    let mut all_areas = Vec::with_capacity(n * (n - 1) / 2);

    for (i, p1) in vertices.iter().enumerate() {
        for p2 in vertices.iter().skip(i + 1) {
            let dx = p1.first.abs_diff(p2.first) + 1;
            let dy = p1.second.abs_diff(p2.second) + 1;

            all_areas.push(((dx * dy) as u64, p1, p2));
        }
    }

    all_areas
}

// Checks whether the two points p1 & p2 form a rectangle given the boundary region
//
// There are two possible orientations, one where p1 is NW of p2
// p1
//        p2
// and the other where p1 is NE of p2
//        p1
// p2
//
fn rectangle_check(p1: &Point, p2: &Point, boundary: &HashSet<Point>) -> bool {
    let top = if p1.first < p2.first { p1 } else { p2 };
    let bot = if p1.first < p2.first { p2 } else { p1 };

    // Case 1, p1 is NW of p2, check for the two boundary points
    // NE of p1 & p2 and SW of p1 & p2
    if top.second < bot.second {
        let mut ne = false;
        let mut sw = false;
        for b in boundary {
            if b.first <= top.first && b.second >= bot.second {
                ne = true;
            }

            if b.first >= bot.first && b.second <= top.second {
                sw = true;
            }

            if ne && sw {
                return true;
            }
        }
    }
    // Case 1, p1 is NE of p2, check if there is a boundary point
    // that goes west of p2 and is above p1
    else {
        let mut nw = false;
        let mut se = false;

        for b in boundary {
            if b.first <= top.first && b.second <= bot.second {
                nw = true;
            }

            if b.first >= bot.first && b.second >= top.second {
                se = true;
            }

            if nw && se {
                return true;
            }
        }
    }

    false
}

/// Checks whether a point p1 is encapsulated by boundary points in all 4 directions
fn grid_surrounded_directions(
    p1: &Point,
    p2: &Point,
    boundary: &HashSet<Point>,
) -> Vec<OrdinalDirection> {
    let mut top_left = false;
    let mut top_right = false;
    let mut bottom_left = false;
    let mut bottom_right = false;

    for b in boundary.iter() {
        if p1 == b {
            continue;
        }

        if b.first >= p1.first && b.first >= p2.first && b.second >= p1.second {
            bottom_right = true;
        }

        if b.first >= p1.first && b.first >= p2.first && b.second <= p1.second {
            bottom_left = true;
        }

        if b.first <= p1.first && b.first <= p2.first && b.second <= p1.second {
            top_left = true;
        }

        if b.first <= p1.first && b.first <= p2.first && b.second >= p1.second {
            top_right = true;
        }

        if top_left && top_right && bottom_left && bottom_right {
            return vec![
                OrdinalDirection::NorthWest,
                OrdinalDirection::NorthEast,
                OrdinalDirection::SouthWest,
                OrdinalDirection::SouthEast,
            ];
        }
    }

    let mut dirs = Vec::with_capacity(4);
    if top_left {
        dirs.push(OrdinalDirection::NorthWest);
    }

    if top_right {
        dirs.push(OrdinalDirection::NorthEast);
    }

    if bottom_left {
        dirs.push(OrdinalDirection::SouthWest);
    }

    if bottom_right {
        dirs.push(OrdinalDirection::SouthEast);
    }

    return dirs;
}

/// Each corner must satisfy its own predicate: full quadrant hit (`len == 4`) **or** the diagonally
/// relevant ordinal is absent. **Both** corners are required (`&&` between the two sides) — an
/// outer **or** between vertices would accept a pair when only one corner matched.
fn corner_checks_ok(
    p1_dirs: &[OrdinalDirection],
    p2_dirs: &[OrdinalDirection],
    p1_corner: OrdinalDirection,
    p2_corner: OrdinalDirection,
) -> bool {
    (p1_dirs.len() == 4 || !p1_dirs.contains(&p1_corner))
        && (p2_dirs.len() == 4 || !p2_dirs.contains(&p2_corner))
}

/// For a vertex pair `(p1, p2)` as emitted by [`compute_areas`] (**lower polygon index** is `p1`),
/// decide which two opposite corners of their axis-aligned bbox the points occupy, and run
/// [`corner_checks_ok`] with the matching ordinals (`p1`’s corner first, `p2`’s second).
///
/// `y` increases downward (larger `second` is further **south**). Summary table (`·` = bbox):
///
/// ```text
/// p2.first > p1.first:
///   p2.second > p1.second   → p1 NW, p2 SE     (p1 top-left, p2 bottom-right)
///   p2.second < p1.second → p1 SW, p2 NE
///   else                    → same row, not opposite diagonal corners
///
/// p2.first < p1.first:
///   p2.second > p1.second → p1 NE, p2 SW
///   p2.second < p1.second → p1 SE, p2 NW
///   else                  → same row
///
/// p2.first == p1.first    → vertical segment, not opposite corners of a non-degenerate rect
/// ```
///
/// Returns `false` when the pair does not sit on two **opposite** corners of a non-degenerate axis
/// bbox — i.e. same `first` (vertical segment) or same `second` (horizontal segment), so the bbox
/// has zero width or zero height.
fn axis_pair_corner_check(
    p1: &Point,
    p2: &Point,
    p1_dirs: &[OrdinalDirection],
    p2_dirs: &[OrdinalDirection],
) -> bool {
    // p2 strictly east of p1
    if p2.first > p1.first {
        if p2.second > p1.second {
            // p1 = NW, p2 = SE
            corner_checks_ok(
                p1_dirs,
                p2_dirs,
                OrdinalDirection::NorthWest,
                OrdinalDirection::SouthEast,
            )
        } else if p2.second < p1.second {
            // p1 = SW, p2 = NE
            corner_checks_ok(
                p1_dirs,
                p2_dirs,
                OrdinalDirection::SouthWest,
                OrdinalDirection::NorthEast,
            )
        } else {
            false
        }
    } else if p2.first < p1.first {
        // p2 strictly west of p1
        if p2.second > p1.second {
            // p1 = NE, p2 = SW
            corner_checks_ok(
                p1_dirs,
                p2_dirs,
                OrdinalDirection::NorthEast,
                OrdinalDirection::SouthWest,
            )
        } else if p2.second < p1.second {
            // p1 = SE, p2 = NW
            corner_checks_ok(
                p1_dirs,
                p2_dirs,
                OrdinalDirection::SouthEast,
                OrdinalDirection::NorthWest,
            )
        } else {
            false
        }
    } else {
        // p2.first == p1.first — vertical segment, not opposite corners of a thick bbox
        false
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Reverse;

    use super::*;

    fn sample_polygon() -> Vec<Point> {
        vec![
            Pair::new(7, 1),
            Pair::new(11, 1),
            Pair::new(11, 7),
            Pair::new(9, 7),
            Pair::new(9, 5),
            Pair::new(2, 5),
            Pair::new(2, 3),
            Pair::new(7, 3),
        ]
    }

    #[test]
    fn axis_pair_corner_geometric_roles() {
        let full = vec![
            OrdinalDirection::NorthWest,
            OrdinalDirection::NorthEast,
            OrdinalDirection::SouthWest,
            OrdinalDirection::SouthEast,
        ];
        // With full quadrant dirs, `corner_checks_ok` is always satisfied — isolates branch logic.
        assert!(axis_pair_corner_check(
            &Point::new(0, 0),
            &Point::new(5, 6),
            &full,
            &full
        )); // NW–SE
        assert!(axis_pair_corner_check(
            &Point::new(0, 6),
            &Point::new(5, 0),
            &full,
            &full
        )); // SW–NE
        assert!(axis_pair_corner_check(
            &Point::new(5, 0),
            &Point::new(0, 6),
            &full,
            &full
        )); // NE–SW
        assert!(axis_pair_corner_check(
            &Point::new(5, 6),
            &Point::new(0, 0),
            &full,
            &full
        )); // SE–NW
        assert!(!axis_pair_corner_check(
            &Point::new(0, 3),
            &Point::new(5, 3),
            &full,
            &full
        ));
        assert!(!axis_pair_corner_check(
            &Point::new(3, 0),
            &Point::new(3, 6),
            &full,
            &full
        ));
    }

    #[test]
    fn sample_part_2() {
        let tiles = sample_polygon();

        let boundary = compute_boundary(&tiles);
        let mut pairwise_areas = compute_areas(&tiles);
        pairwise_areas.sort_by_key(|&(a, _, _)| Reverse(a));

        let mut max_area = 0;
        for (area, p1, p2) in pairwise_areas {
            if p1.first == 9 && p1.second == 5 && p2.first == 2 && p2.second == 3 {
                println!("fix");
            }

            if rectangle_check(p1, p2, &boundary) {
                max_area = area;
                break;
            }
        }

        // Draft predicate is not the full lagoon check; first accepted pair by area is 50 on the sample.
        assert_eq!(24, max_area);
    }
}
