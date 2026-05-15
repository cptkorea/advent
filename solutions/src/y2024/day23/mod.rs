use crate::{AdventError, AdventProblem};
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl AdventProblem for Day23 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let neighbors = build_neighbors(&lines);
        let mut triplets = HashSet::new();

        for &n1 in neighbors.keys() {
            for &n2 in &neighbors[&n1] {
                for &n3 in &neighbors[&n2] {
                    for &n4 in &neighbors[&n3] {
                        if n4 == n1 {
                            let mut triplet = vec![n1, n2, n3];
                            triplet.sort();
                            triplets.insert((triplet[0], triplet[1], triplet[2]));
                        }
                    }
                }
            }
        }

        let mut total = 0;
        for (n1, n2, n3) in triplets {
            if n1.starts_with("t") || n2.starts_with("t") || n3.starts_with("t") {
                total += 1;
            }
        }

        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let adjacency = build_neighbors(&lines);

        let mut biggest = Vec::new();

        for (&node, neighbors) in &adjacency {
            let mut group = vec![node];
            for &nei in neighbors {
                if group.iter().all(|&n| adjacency[n].contains(nei)) {
                    group.push(nei);
                }
            }

            if group.len() > biggest.len() {
                biggest = group;
            }
        }

        biggest.sort();

        let password = biggest.join(",");

        println!("password={}", password);
        Ok(0)
    }
}

fn build_neighbors(lines: &Vec<String>) -> HashMap<&str, HashSet<&str>> {
    let mut neighbors: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in lines {
        let mut conn = line.split("-");
        let first = conn.next().unwrap();
        let second = conn.next().unwrap();

        neighbors.entry(first).or_default().insert(second);
        neighbors.entry(second).or_default().insert(first);
    }
    neighbors
}
