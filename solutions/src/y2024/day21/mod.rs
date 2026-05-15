use crate::{AdventError, AdventProblem};
use std::collections::HashMap;

pub struct Day21;

impl AdventProblem for Day21 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut total = 0;
        for line in &lines {
            let dirs = robot_dirs(line);

            let num = line.replace("A", "").parse::<u32>().expect("numeric value");
            total += num * (dirs.len() as u32);
        }

        Ok(total)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut total = 0;
        for line in &lines {
            let dirs = cached_robot_dirs(line);

            let num = line.replace("A", "").parse::<u32>().expect("numeric value");
            total += num * (dirs.len() as u32);
        }

        Ok(total)
    }
}

fn robot_dirs(seq: &str) -> String {
    let keypad = keypad();
    let controller = controls();

    let dirs = directions(seq, &keypad, (0, 0));
    println!("dirs={}", dirs);
    let controller_dirs = directions(&dirs, &controller, (1, 0));
    println!("controller_dirs={}", controller_dirs);
    let robot_dirs = directions(&controller_dirs, &controller, (1, 0));
    println!("robot_dirs={}", robot_dirs);
    robot_dirs
}

fn cached_robot_dirs(seq: &str) -> String {
    let keypad = keypad();
    let controller = controls();

    let mut dirs = directions(seq, &keypad, (0, 0));

    let mut cache: HashMap<(char, char), String> = HashMap::new();

    for _ in 0..26 {
        dirs = directions_with_cache(&dirs, &controller, (1, 0), &mut cache);
    }
    println!("robot_dirs={}", dirs);
    dirs
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn push_horizontal(s: &mut String, start: (usize, usize), end: (usize, usize)) {
    if end.1 > start.1 {
        for _ in 0..(end.1 - start.1) {
            s.push_str(">");
        }
    } else if start.1 > end.1 {
        for _ in 0..(start.1 - end.1) {
            s.push_str("<");
        }
    }
}

fn push_vertical(s: &mut String, start: (usize, usize), end: (usize, usize)) {
    if end.0 > start.0 {
        for _ in 0..(end.0 - start.0) {
            s.push_str("^");
        }
    } else if start.0 > end.0 {
        for _ in 0..(start.0 - end.0) {
            s.push_str("v");
        }
    }
}

fn directions(seq: &str, keys: &HashMap<char, (usize, usize)>, empty: (usize, usize)) -> String {
    let mut pos = keys[&'A'];
    let mut directions = String::new();

    for c in seq.chars() {
        let next = keys[&c];
        let next_dirs = append_directions(pos, next, empty);
        directions.push_str(&next_dirs);
        directions.push_str("A");
        pos = next;
    }

    directions
}

fn directions_with_cache(
    seq: &str,
    keys: &HashMap<char, (usize, usize)>,
    empty: (usize, usize),
    cache: &mut HashMap<(char, char), String>,
) -> String {
    let mut curr_ch = 'A';
    let mut pos = keys[&'A'];
    let mut directions = String::new();

    for next_ch in seq.chars() {
        let next = keys[&next_ch];

        match cache.get(&(curr_ch, next_ch)) {
            Some(s) => {
                directions.push_str(s);
            }
            None => {
                let dirs = append_directions(pos, next, empty);
                directions.push_str(&dirs);
                cache.insert((curr_ch, next_ch), dirs);
            }
        }
        directions.push_str("A");
        pos = next;
        curr_ch = next_ch;
    }

    directions
}

fn append_directions(pos: (usize, usize), next: (usize, usize), empty: (usize, usize)) -> String {
    let mut directions = String::new();
    if pos.1 > next.1 {
        if pos.0 == empty.0 && next.1 == empty.1 {
            push_vertical(&mut directions, pos, next);
            push_horizontal(&mut directions, pos, next);
        } else {
            push_horizontal(&mut directions, pos, next);
            push_vertical(&mut directions, pos, next);
        }
        return directions;
    }

    // moving up
    if pos.0 < next.0 {
        if pos.1 == empty.1 && next.0 == empty.0 {
            push_horizontal(&mut directions, pos, next);
            push_vertical(&mut directions, pos, next);
        } else {
            push_vertical(&mut directions, pos, next);
            push_horizontal(&mut directions, pos, next);
        }
        return directions;
    }

    // moving down
    if pos.0 > next.0 {
        if pos.1 == empty.1 && next.0 == empty.0 {
            push_horizontal(&mut directions, pos, next);
            push_vertical(&mut directions, pos, next);
        } else {
            push_vertical(&mut directions, pos, next);
            push_horizontal(&mut directions, pos, next);
        }
        return directions;
    }

    if pos.1 < next.1 {
        push_horizontal(&mut directions, pos, next);
        push_vertical(&mut directions, pos, next);
    }
    return directions;
}

/**
 * <^A
 * v<<A>^A>A
 * v<A<AA>>^AvA<^A>AvA^A 21
 *
 * ^<A
 * <Av>A>>^A
 * v<<A>>^Av<A>A^AvAA<^A>A 23
 *
 */

fn up_directions(seq: &str, keys: &HashMap<char, (usize, usize)>) -> String {
    let mut pos = keys[&'A'];
    let mut directions = String::new();

    for c in seq.chars() {
        let next = keys[&c];
        if next.0 > pos.0 {
            for _ in 0..(next.0 - pos.0) {
                directions.push_str("^");
            }
        } else if pos.0 > next.0 {
            for _ in 0..(pos.0 - next.0) {
                directions.push_str("v");
            }
        }

        if next.1 > pos.1 {
            for _ in 0..(next.1 - pos.1) {
                directions.push_str(">");
            }
        } else if pos.1 > next.1 {
            for _ in 0..(pos.1 - next.1) {
                directions.push_str("<");
            }
        }

        directions.push_str("A");
        pos = next;
    }

    directions
}

fn generate_directions(start: (usize, usize), end: (usize, usize)) -> Vec<String> {
    let mut directions = Vec::new();
    let mut row_first = String::new();
    if end.0 > start.0 {
        for _ in 0..(start.0 - end.0) {
            row_first.push_str("^");
        }
        directions.push(row_first);
    } else if start.0 > end.0 {
        if end.1 != 0 {
            for _ in 0..(end.0 - start.0) {
                row_first.push_str("v");
            }
            directions.push(row_first);
        }
    }

    let mut col_first = String::new();
    if end.1 > start.1 {
        for _ in 0..(start.1 - end.1) {
            col_first.push_str("^");
        }
        directions.push(col_first);
    } else if start.1 > end.1 {
        if end.0 != 0 {
            for _ in 0..(end.1 - start.1) {
                col_first.push_str("v");
            }
            directions.push(col_first);
        }
    }
    directions
}

fn left_directions(seq: &str, keys: &HashMap<char, (usize, usize)>) -> String {
    let mut pos = keys[&'A'];
    let mut directions = String::new();

    for c in seq.chars() {
        let next = keys[&c];
        if next.1 > pos.1 {
            for _ in 0..(next.1 - pos.1) {
                directions.push_str(">");
            }
        } else if pos.1 > next.1 {
            for _ in 0..(pos.1 - next.1) {
                directions.push_str("<");
            }
        }

        if next.0 > pos.0 {
            for _ in 0..(next.0 - pos.0) {
                directions.push_str("^");
            }
        } else if pos.0 > next.0 {
            for _ in 0..(pos.0 - next.0) {
                directions.push_str("v");
            }
        }

        directions.push_str("A");
        pos = next;
    }

    directions
}

fn keypad() -> HashMap<char, (usize, usize)> {
    HashMap::from([
        ('0', (0, 1)),
        ('1', (1, 0)),
        ('2', (1, 1)),
        ('3', (1, 2)),
        ('4', (2, 0)),
        ('5', (2, 1)),
        ('6', (2, 2)),
        ('7', (3, 0)),
        ('8', (3, 1)),
        ('9', (3, 2)),
        ('A', (0, 2)),
    ])
}

fn controls() -> HashMap<char, (usize, usize)> {
    HashMap::from([
        ('^', (1, 1)),
        ('<', (0, 0)),
        ('v', (0, 1)),
        ('>', (0, 2)),
        ('A', (1, 2)),
    ])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(68, robot_dirs("029A").len());
        assert_eq!(60, robot_dirs("980A").len());
        assert_eq!(68, robot_dirs("179A").len());
        assert_eq!(64, robot_dirs("456A").len());
        assert_eq!(64, robot_dirs("379A").len());
    }
}
