use crate::{AdventError, AdventProblem};
use advent_common::number::XYZCoord;
use advent_common::ufind::UnionFind;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Day8;

impl AdventProblem for Day8 {
    type Answer = usize;

    fn run_part_1(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let junctions = lines
            .iter()
            .map(|s| XYZCoord::try_from(s.as_str()))
            .collect::<Result<Vec<_>, AdventError>>()?;

        let (mut uf, _) = connect_junctions(&junctions, 1000);
        let (f, s, t) = get_three_largest_groups(&mut uf);
        Ok(f * s * t)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<Self::Answer, AdventError> {
        let junctions = lines
            .iter()
            .map(|s| XYZCoord::try_from(s.as_str()))
            .collect::<Result<Vec<_>, AdventError>>()?;

        let num_junctions = junctions.len();
        println!("junctions={num_junctions}");

        let (mut min, mut max) = (1000, 10_000);
        let mut last_conn = (0, 0);
        let mut last_uf = None;

        while min < max {
            let mid = (min + max) / 2;
            let (mut uf, lc) = connect_junctions(&junctions, mid);

            if uf.disjoint_groups().len() == 1 {
                max = mid;
                last_uf = Some(uf);
                last_conn = lc;
            } else {
                min = mid + 1;
            }
        }

        let mut groups = last_uf.unwrap().disjoint_groups();
        groups.sort_by_key(|&(size, _)| Reverse(size));

        let (first, second) = last_conn;

        let j1 = &junctions[first].clone();
        let j2: &XYZCoord = &junctions[second].clone();

        Ok((j1.x * j2.x) as usize)
    }
}

fn connect_junctions(junctions: &[XYZCoord], num_iter: usize) -> (UnionFind, (usize, usize)) {
    let n = junctions.len();

    let mut union_find = UnionFind::new(n);
    let mut pq = BinaryHeap::new();
    let mut processed_nodes = vec![false; n];

    for (i, j1) in junctions.iter().enumerate() {
        for (j, j2) in junctions.iter().enumerate().skip(i + 1) {
            let dist = j1.dist_sq(j2);
            pq.push(Reverse((dist, i, j)));
        }
    }

    let mut last_conn = (0, 0);

    for n_iter in 0..num_iter {
        let (_, i, j) = pq.pop().unwrap().0;

        union_find.connect(i, j);
        processed_nodes[i] = true;
        processed_nodes[j] = true;

        if n_iter == num_iter - 1 {
            last_conn = (i, j);
        }
    }

    (union_find, last_conn)
}

fn get_three_largest_groups(u: &mut UnionFind) -> (usize, usize, usize) {
    let mut groups = u.disjoint_groups();
    groups.sort_by_key(|&(size, _)| Reverse(size));

    (groups[0].0, groups[1].0, groups[2].0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_part_1() {
        let junctions = vec![
            XYZCoord::new(162, 817, 812),
            XYZCoord::new(57, 618, 57),
            XYZCoord::new(906, 360, 560),
            XYZCoord::new(592, 479, 940),
            XYZCoord::new(352, 342, 300),
            XYZCoord::new(466, 668, 158),
            XYZCoord::new(542, 29, 236),
            XYZCoord::new(431, 825, 988),
            XYZCoord::new(739, 650, 466),
            XYZCoord::new(52, 470, 668),
            XYZCoord::new(216, 146, 977),
            XYZCoord::new(819, 987, 18),
            XYZCoord::new(117, 168, 530),
            XYZCoord::new(805, 96, 715),
            XYZCoord::new(346, 949, 466),
            XYZCoord::new(970, 615, 88),
            XYZCoord::new(941, 993, 340),
            XYZCoord::new(862, 61, 35),
            XYZCoord::new(984, 92, 344),
            XYZCoord::new(425, 690, 689),
        ];

        let (mut res, _) = connect_junctions(&junctions, 10);
        let (f, s, t) = get_three_largest_groups(&mut res);

        assert_eq!(40, f * s * t);
    }
}
