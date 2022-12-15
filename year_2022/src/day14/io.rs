use crate::day14::position::Position;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn parse_line(line: &str) -> Vec<Position> {
    line.split(' ')
        .into_iter()
        .filter_map(|token| {
            let position = Position::from_str(token);

            match position.is_ok() {
                false => None,
                true => Some(position.unwrap()),
            }
        })
        .collect()
}

pub(crate) fn read_lines(path: &str) -> Vec<Vec<Position>> {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|line| parse_line(line.as_ref().unwrap()))
        .collect()
}
