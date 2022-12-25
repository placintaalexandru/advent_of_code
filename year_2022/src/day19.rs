use crate::day19::blueprint::BluePrint;
use crate::day19::robot_factory::RobotFactory;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod bag;
mod blueprint;
mod resource;
mod robot_factory;

pub struct Day19;

impl Day19 {
    fn parse(path: &str) -> Vec<BluePrint> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|line| BluePrint::from_str(line.as_ref().unwrap()).unwrap())
            .collect()
    }

    pub fn part_one() -> usize {
        let mut robot_factory = RobotFactory::default();

        Self::parse("src/day19/input")
            .into_iter()
            .enumerate()
            .map(|(idx, blueprint)| {
                let mut blueprint_value = 0;
                robot_factory.blue_print_value(1, 25, &blueprint, 0, &mut blueprint_value);
                (idx + 1) * blueprint_value
            })
            .sum()
    }

    pub fn part_two() -> usize {
        let mut robot_factory = RobotFactory::default();
        let blueprints = Self::parse("src/day19/input");

        (0..3)
            .map(|idx| {
                let mut blueprint_value = 0;
                robot_factory.blue_print_value(1, 33, &blueprints[idx], 0, &mut blueprint_value);
                blueprint_value
            })
            .product()
    }
}
