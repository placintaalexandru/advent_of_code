use crate::day20::mixer::Mixer;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod mixer;

pub struct Day20;

impl Day20 {
    fn mix_with_key(key: i64, rounds: usize) -> i64 {
        let mut shifter = Mixer::new(
            BufReader::new(File::open("src/day20/input").unwrap())
                .lines()
                .map(|line| line.unwrap().parse::<i64>().unwrap() * key)
                .collect(),
        );
        shifter.shift_rounds(rounds);

        let zero_element = shifter
            .positions
            .iter()
            .find(|position| (*position).1 == 0)
            .unwrap();

        [1_000, 2_000, 3_000]
            .into_iter()
            .map(|offset| {
                shifter
                    .positions
                    .iter()
                    .find(|position| {
                        (*position).0 as i64
                            == (zero_element.0 as i64 + offset)
                                .rem_euclid(shifter.positions.len() as i64)
                    })
                    .unwrap()
                    .1
            })
            .sum()
    }

    pub fn part_one() -> i64 {
        Self::mix_with_key(1, 1)
    }

    pub fn part_two() -> i64 {
        Self::mix_with_key(811_589_153, 10)
    }
}
