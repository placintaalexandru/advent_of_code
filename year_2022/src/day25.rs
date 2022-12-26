use crate::day25::converter::Converter;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod converter;

pub struct Day25;

impl Day25 {
    pub fn part_one() -> String {
        let n = BufReader::new(File::open("src/day25/input").unwrap())
            .lines()
            .map(|line| Converter::snafu_to_base10(line.as_ref().unwrap(), 5))
            .sum::<i64>();

        Converter::base10_to_snafu(n, 5)
    }
}
