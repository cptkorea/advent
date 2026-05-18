use advent_common::floodfill::{try_transition, EIGHT_DIRECTIONS};

use crate::{AdventError, AdventProblem};

pub struct Day4;

impl AdventProblem for Day4 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let cnt = count_neighbor_squares(&grid);

        Ok(cnt)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let mut grid = lines
            .iter()
            .map(|s| s.chars().collect())
            .collect::<Vec<Vec<_>>>();

        let max_cnt = simulate_removal(&mut grid);
        Ok(max_cnt)
    }
}

fn is_paper_roll(c: char) -> bool {
    c == '@'
}

fn count_neighbor_squares(grid: &[Vec<char>]) -> usize {
    let boundary = (grid.len(), grid[0].len());
    let mut cnt = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if !is_paper_roll(*val) {
                continue;
            }

            let mut nei_cnt = 0;
            for dir in EIGHT_DIRECTIONS {
                if let Some((nr, nc)) = try_transition((r, c), boundary, dir) {
                    let nei_val = grid[nr][nc];
                    if is_paper_roll(nei_val) {
                        nei_cnt += 1;
                    }
                }
            }

            if nei_cnt < 4 {
                cnt += 1;
            }
        }
    }

    cnt
}

pub fn simulate_removal(grid: &mut [Vec<char>]) -> usize {
    let mut has_removal = true;
    let mut total_cnt = 0;
    let boundary = (grid.len(), grid[0].len());

    while has_removal {
        has_removal = false;
        let mut cnt = 0;
        let mut removed = vec![];

        for (r, row) in grid.iter().enumerate() {
            for (c, val) in row.iter().enumerate() {
                if !is_paper_roll(*val) {
                    continue;
                }

                let mut nei_cnt = 0;

                for dir in EIGHT_DIRECTIONS {
                    if let Some((nr, nc)) = try_transition((r, c), boundary, dir) {
                        let nei_val = grid[nr][nc];
                        if is_paper_roll(nei_val) {
                            nei_cnt += 1;
                        }
                    }
                }

                if nei_cnt < 4 {
                    has_removal = true;
                    cnt += 1;
                    removed.push((r, c));
                }
            }
        }

        for (r, c) in removed {
            grid[r][c] = 'x';
        }

        if has_removal {
            total_cnt += cnt;
        }
    }

    total_cnt
}

#[cfg(test)]
mod test {
    use advent_common::floodfill::construct_grid;

    use super::*;

    #[test]
    fn sample_part_1() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        let grid = construct_grid(&input);
        assert_eq!(13, count_neighbor_squares(&grid));
    }

    #[test]
    fn sample_part_2() {
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];

        let mut grid = construct_grid(&input);
        // let cnt = simulate_removal(&mut grid);

        // for (r, row) in grid.iter().enumerate() {
        // println!("row={row:?}")
        // }
        assert_eq!(43, simulate_removal(&mut grid));
    }
}
