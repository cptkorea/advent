use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Binary,
};

use crate::{AdventError, AdventProblem};

pub struct Day5;

impl AdventProblem for Day5 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (rules, i) = parse_ordering_rules(&lines);
        let requests = parse_print_requests(&lines, i + 1);

        let total = requests
            .iter()
            .filter(|r| r.is_valid(&rules))
            .map(|r| r.middle())
            .sum();

        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let (rules, i) = parse_ordering_rules(&lines);
        let mut requests = parse_print_requests(&lines, i + 1);

        let total = requests
            .iter_mut()
            .filter(|r| !r.is_valid(&rules))
            .map(|r| {
                r.fix(&rules);
                r.middle()
            })
            .sum();

        Ok(total)
    }
}

pub struct Request {
    pages: Vec<u32>,
}

#[derive(Eq, PartialEq)]
pub struct Page {
    num: u32,
    dependencies: HashSet<u32>,
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dependencies.len().cmp(&other.dependencies.len())
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Request {
    fn middle(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    // For each page, check whether there is a subsequent print request
    // that has the current page as a dependency. If so, the request is invalid.
    fn is_valid(&self, rules: &HashMap<u32, HashSet<u32>>) -> bool {
        let n = self.pages.len();
        for i in 0..n {
            let page = self.pages[i];
            if let Some(dependencies) = rules.get(&page) {
                for j in (i + 1)..n {
                    if dependencies.contains(&self.pages[j]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    // 1. Retrieve all dependencies for each page, this should form a directed-acyclic graph
    // 2. Traverse the graph in topological order removing each completed dependency
    // 3. The order of the traversal will be the new order of the pages
    fn fix(&mut self, rules: &HashMap<u32, HashSet<u32>>) -> bool {
        let n = self.pages.len();
        let mut i = 0;
        while i < n {
            let page = self.pages[i];
            let mut redo = false;
            if let Some(dependencies) = rules.get(&page) {
                for j in (i + 1)..n {
                    if dependencies.contains(&self.pages[j]) {
                        let tmp = self.pages[i];
                        self.pages[i] = self.pages[j];
                        self.pages[j] = tmp;
                        redo = true;
                        break;
                    }
                }
            }
            if !redo {
                i += 1;
            }
        }
        true
    }
}

/**
 * Given ordering rule 47|53, map 53 -> 47 indicating that it must be printed after
 */
fn parse_ordering_rules(lines: &Vec<String>) -> (HashMap<u32, HashSet<u32>>, usize) {
    let mut i = 0;
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    while i < lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            break;
        }

        let mut tokens = line.split("|");
        let first = tokens
            .next()
            .expect("print rule first page")
            .parse::<u32>()
            .expect("non-numeric page in print rule");

        let second = tokens
            .next()
            .expect("print rule second page")
            .parse::<u32>()
            .expect("non-numeric page in print rule");

        rules.entry(second).or_default().insert(first);
        i += 1;
    }

    (rules, i)
}

/**
 * Given ordering rule 47|53, map 53 -> 47 indicating that it must be printed after
 */
fn parse_print_requests(lines: &Vec<String>, start: usize) -> Vec<Request> {
    let mut i = start;
    let mut requests = Vec::new();

    while i < lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            break;
        }

        let pages = line
            .split(",")
            .into_iter()
            .map(|s| s.parse::<u32>().expect("non-numeric page in request"))
            .collect::<Vec<_>>();

        requests.push(Request { pages });
        i += 1;
    }

    requests
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_request() {
        let request = Request {
            pages: vec![75, 47, 61, 53, 29],
        };

        let rules = HashMap::from_iter([
            (47, HashSet::from_iter([75, 97])),
            (53, HashSet::from_iter([13, 47, 61, 75, 97])),
            (61, HashSet::from_iter([47, 75, 97])),
            (29, HashSet::from_iter([47, 61, 75, 97, 29])),
            (97, HashSet::from_iter([75])),
        ]);

        assert!(request.is_valid(&rules));
    }

    #[test]
    fn invalid_request() {
        let request = Request {
            pages: vec![75, 97, 47, 61, 53],
        };

        let rules = HashMap::from_iter([
            (47, HashSet::from_iter([75, 97])),
            (53, HashSet::from_iter([13, 47, 61, 75, 97])),
            (61, HashSet::from_iter([47, 75, 97])),
            (29, HashSet::from_iter([47, 61, 75, 97, 29])),
            (75, HashSet::from_iter([97])),
        ]);

        assert!(!request.is_valid(&rules));
    }

    #[test]
    fn swap() {
        let mut request = Request {
            pages: vec![75, 97, 47, 61, 53],
        };

        let rules = HashMap::from_iter([
            (47, HashSet::from_iter([75, 97])),
            (53, HashSet::from_iter([13, 47, 61, 75, 97])),
            (61, HashSet::from_iter([47, 75, 97])),
            (29, HashSet::from_iter([47, 61, 75, 97, 29])),
            (75, HashSet::from_iter([97])),
        ]);

        request.fix(&rules);
        assert_eq!(vec![97, 75, 47, 61, 53], request.pages);
    }
}
