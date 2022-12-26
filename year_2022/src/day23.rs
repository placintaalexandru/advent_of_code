use crate::day23::direction::Direction;
use crate::day23::elf::{Elf, Pixel};
use crate::day23::position::Position;
use itertools::Itertools;
use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;

mod direction;
mod elf;
mod position;

pub struct Day23;

impl Day23 {
    fn read_elves(path: &str) -> HashSet<Elf> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .filter_map(|(j, c)| match Pixel::from(c) {
                        Pixel::Land => None,
                        Pixel::Elf => Some(Elf::new(Position(i as i32, j as i32))),
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn simulate(rounds: usize, others: &mut HashSet<Elf>) -> usize {
        let mut directions = LinkedList::from([
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]);

        for i in 0..rounds {
            let mut destinations = HashMap::new();
            let mut proposals = HashMap::new();

            for elf in others.iter().filter(|elf| (*elf).can_move(others)) {
                for direction_now in directions.iter() {
                    if let Some(destination) = elf.destination(direction_now, others) {
                        destinations
                            .entry(destination)
                            .or_insert_with(Vec::new)
                            .push(elf.clone());
                        proposals
                            .entry(elf.clone())
                            .or_insert_with(Vec::new)
                            .push(destination);
                        break;
                    }
                }
            }

            if proposals.is_empty() {
                return i + 1;
            }

            proposals.into_iter().for_each(|(elf, elf_proposals)| {
                for proposal in &elf_proposals {
                    if destinations[&proposal].len() == 1 {
                        others.remove(&elf);
                        others.insert(Elf::new(*proposal));
                        break;
                    }
                }
            });

            let direction = directions.pop_front().unwrap();
            directions.push_back(direction);
        }

        return rounds;
    }

    pub fn part_one() -> usize {
        let mut elves = Self::read_elves("src/day23/input");
        Self::simulate(10, &mut elves);
        let mut bot_left_corner = Position(i32::MAX, i32::MAX);
        let mut top_right_corner = Position(i32::MIN, i32::MIN);

        elves.iter().for_each(|elf| {
            bot_left_corner.0 = bot_left_corner.0.min(elf.position.0);
            bot_left_corner.1 = bot_left_corner.1.min(elf.position.1);

            top_right_corner.0 = top_right_corner.0.max(elf.position.0);
            top_right_corner.1 = top_right_corner.1.max(elf.position.1);
        });

        ((top_right_corner.1 - bot_left_corner.1 + 1)
            * (top_right_corner.0 - bot_left_corner.0 + 1)) as usize
            - elves.len()
    }

    pub fn part_two() -> usize {
        let mut elves = Self::read_elves("src/day23/input");
        Self::simulate(usize::MAX, &mut elves)
    }
}
