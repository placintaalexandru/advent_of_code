use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Day6;

impl Day6 {
    fn valid_signale(signal: &str, start: usize, length: usize) -> bool {
        if start + length >= signal.len() {
            return false;
        }

        let mut s = HashSet::new();
        let bytes = signal.as_bytes();

        for pos in start..start + length {
            s.insert(bytes[pos] as char);
        }

        return s.len() == length;
    }

    fn find_amount(length: usize) -> usize {
        let file = File::open("src/day6/input").unwrap();
        let mut buffer_reader = io::BufReader::new(file);
        let mut content = String::default();
        buffer_reader.read_line(&mut content).unwrap();

        for i in 0..content.len() - length {
            if Self::valid_signale(&content, i, length) {
                return i + length;
            }
        }

        0
    }

    pub fn part_one() -> usize {
        Self::find_amount(4)
    }

    pub fn part_two() -> usize {
        Self::find_amount(14)
    }
}
