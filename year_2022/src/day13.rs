use crate::day13::packet::Packet;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod packet;

pub struct Day13;

impl Day13 {
    pub fn part_one() -> usize {
        let buffer_reader = BufReader::new(File::open("src/day13/input").unwrap());

        buffer_reader
            .lines()
            .filter_map(|line| {
                (line.is_ok() && !line.as_ref().unwrap().is_empty()).then_some(line.unwrap())
            })
            .collect::<Vec<_>>()
            .chunks(2)
            .enumerate()
            .filter_map(|(i, chunk)| {
                (Packet::from_str(&chunk[0])
                    .unwrap()
                    .partial_cmp(&Packet::from_str(&chunk[1]).unwrap())
                    == Some(Ordering::Less))
                .then_some(i + 1)
            })
            .sum()
    }

    pub fn part_two() -> usize {
        let buffer_reader = BufReader::new(File::open("src/day13/input").unwrap());
        let mut raw_signal = buffer_reader
            .lines()
            .filter_map(|line| match Packet::from_str(line.as_ref().unwrap()) {
                Ok(packet) => Some(packet),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        let divider_packets = vec![
            Packet::from_str("[[2]]").unwrap(),
            Packet::from_str("[[6]]").unwrap(),
        ];

        raw_signal.extend(divider_packets.clone());
        raw_signal.sort();

        raw_signal
            .into_iter()
            .enumerate()
            .filter_map(|(i, signal)| (divider_packets.contains(&signal)).then_some(i + 1))
            .product()
    }
}
