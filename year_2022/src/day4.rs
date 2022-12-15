use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

pub struct Day4;

struct Interval {
    left: usize,
    right: usize,
}

impl Interval {
    fn contains(&self, other: &Self) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    fn overlap(&self, other: &Self) -> bool {
        !(other.right < self.left
            || self.left > other.right
            || self.right < other.left
            || self.right < other.left)
    }
}

impl FromStr for Interval {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split("-").collect::<Vec<&str>>();
        Ok(Self {
            left: (&tokens[0]).parse::<usize>().unwrap(),
            right: (&tokens[1]).parse::<usize>().unwrap(),
        })
    }
}

impl Day4 {
    pub fn part_one() -> usize {
        let file = File::open("src/day4/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut result = 0_usize;

        for line in buffer_reader.lines() {
            let content = line.unwrap();
            let tokens = content.split(",").collect::<Vec<&str>>();
            let (i1, i2) = (
                Interval::from_str(&tokens[0]).unwrap(),
                Interval::from_str(&tokens[1]).unwrap(),
            );

            result += (i1.contains(&i2) || i2.contains(&i1)) as usize;
        }

        result
    }

    pub fn part_two() -> usize {
        let file = File::open("src/day4/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut result = 0_usize;

        for line in buffer_reader.lines() {
            let content = line.unwrap();
            let tokens = content.split(",").collect::<Vec<&str>>();
            let (i1, i2) = (
                Interval::from_str(&tokens[0]).unwrap(),
                Interval::from_str(&tokens[1]).unwrap(),
            );

            result += i1.overlap(&i2) as usize;
        }

        result
    }
}
