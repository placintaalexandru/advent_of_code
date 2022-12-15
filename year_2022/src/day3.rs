use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Day3;

impl Day3 {
    fn priority(c: char) -> u8 {
        match c.is_lowercase() {
            true => 1 + (c as u8 - 'a' as u8),
            false => 27 + (c as u8 - 'A' as u8),
        }
    }

    fn analyze_line(s: &str) -> usize {
        let mut first_half_set = HashSet::new();
        let mut result = 0;

        for (i, c) in s.chars().enumerate() {
            if i < s.len() / 2 {
                first_half_set.insert(c);
            } else {
                if first_half_set.contains(&c) {
                    result += Self::priority(c) as usize;
                    first_half_set.remove(&c);
                }
            }
        }

        result
    }

    fn find_badge(groups: &Vec<String>) -> char {
        let sets = groups
            .iter()
            .map(|group| group.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        *(&sets[0])
            .iter()
            .find(|c| sets.iter().all(|set| set.contains(*c)))
            .unwrap()
    }

    pub fn part_one() -> usize {
        let file = File::open("src/day3/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut result = 0_usize;

        for line in buffer_reader.lines() {
            result += Self::analyze_line(line.as_ref().unwrap());
        }

        result
    }

    pub fn part_two() -> usize {
        let file = File::open("src/day3/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut group = vec![];
        let mut result = 0_usize;

        for line in buffer_reader.lines() {
            if group.len() == 3 {
                result += Self::priority(Self::find_badge(&group)) as usize;
                group.clear();
            }

            group.push(line.unwrap());
        }

        if group.len() == 3 {
            result += Self::priority(Self::find_badge(&group)) as usize;
        }

        result
    }
}
