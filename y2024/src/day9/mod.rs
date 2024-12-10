use crate::{AdventError, AdventProblem};

pub struct Day9;

impl AdventProblem for Day9 {
    fn run_part_1(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut d = DiskMap::from(lines[0].as_str());
        d.move_blocks();

        println!("checksum={:?}", d.data);
        Ok(0)
    }

    fn run_part_2(&self, lines: Vec<String>) -> Result<u32, AdventError> {
        let mut d = CompressedDiskMap::from(lines[0].as_str());
        d.move_blocks();
        println!("checksum={:?}", d.checksum());
        println!("final={:?}", d.data);

        Ok(0)
    }
}

struct DiskMap {
    data: Vec<Option<u32>>,
}

impl DiskMap {
    fn move_blocks(&mut self) {
        let n = self.data.len();
        let (mut i, mut j) = (0, n - 1);

        while i < n && self.data[i].is_some() {
            i += 1;
        }

        while i < j {
            if self.data[j].is_some() {
                self.data[i] = self.data[j];
                self.data[j] = None;
                i += 1;
            }
            j -= 1;

            while i < n && self.data[i].is_some() {
                i += 1;
            }
        }
    }

    fn checksum(&self) -> u64 {
        let mut total = 0;
        for (i, v) in self.data.iter().enumerate() {
            if let Some(v) = v {
                total += (i as u64) * (*v as u64);
            } else {
                break;
            }
        }
        total
    }
}

impl From<&str> for DiskMap {
    fn from(value: &str) -> Self {
        let mut id = 0;
        let mut data = Vec::new();

        for (i, c) in value.chars().enumerate() {
            let part = if i % 2 == 0 { Some(id) } else { None };
            let n = c.to_digit(10).expect("non-numeric bit");
            for _ in 0..n {
                data.push(part);
            }
            if i % 2 == 0 {
                id += 1;
            }
        }

        Self { data }
    }
}

struct CompressedDiskMap {
    data: Vec<(Option<u32>, u32)>,
}

impl CompressedDiskMap {
    fn move_blocks(&mut self) {
        let n = self.data.len();
        let mut i = n - 1;

        while i > 1 {
            let (data, n) = self.data[i];
            if data.is_some() {
                for j in 0..i {
                    let (space, size) = self.data[j];
                    if space.is_none() && n <= size {
                        self.data[j] = self.data[i];
                        self.data[i] = (None, n);

                        if n < size {
                            if self.data[j + 1].0.is_none() {
                                self.data[j + 1].1 += size - n;
                            } else {
                                self.data.insert(j + 1, (None, size - n));
                                i += 1;
                            }
                        }
                        break;
                    }
                }
            }
            i -= 1;
        }
    }

    fn checksum(&self) -> u64 {
        let mut total = 0;
        let mut i = 0;
        for (v, n) in self.data.iter() {
            if let Some(v) = v {
                for j in i..(i + n) {
                    total += (j as u64) * (*v as u64);
                }
            }
            i += n;
        }
        total
    }
}

impl From<&str> for CompressedDiskMap {
    fn from(value: &str) -> Self {
        let mut data = Vec::new();

        for (i, c) in value.chars().enumerate() {
            let part = if i % 2 == 0 {
                Some((i / 2) as u32)
            } else {
                None
            };
            let n = c.to_digit(10).expect("non-numeric bit");
            data.push((part, n));
        }

        Self { data }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let s = "2333133121414131402";
        let _ = DiskMap::from(s);
    }

    #[test]
    fn move_data() {
        let s = "2333133121414131402";
        let mut d = DiskMap::from(s);
        d.move_blocks();
        assert_eq!(1928, d.checksum());
    }

    #[test]
    fn parse_data_compressed() {
        let s = "2333133121414131402";
        let _ = CompressedDiskMap::from(s);
    }

    #[test]
    fn move_data_compressed() {
        let s = "2333133121414131402";
        let mut d = CompressedDiskMap::from(s);
        println!("{:?}", d.data);
        d.move_blocks();
        assert_eq!(2858, d.checksum());
        println!("{:?}", d.data);
    }
}
