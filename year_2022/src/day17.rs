use crate::day17::commands::Commands;
use crate::day17::direction::Position;
use crate::day17::rock::{Rock, RockType};
use crate::day17::screen::Screen;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use strum::IntoEnumIterator;

mod commands;
mod direction;
mod pixel;
mod rock;
mod screen;

pub struct Day17;

impl Day17 {
    fn simulate(steps: usize) -> usize {
        let mut screen = Screen::new(7);
        let mut buffer_reader = BufReader::new(File::open("src/day17/input").unwrap());
        let mut buffer = String::default();
        buffer_reader.read_line(&mut buffer);
        let rock_sequence = RockType::iter().collect::<Vec<RockType>>();
        let mut commands = Commands::from_str(buffer.trim()).unwrap();

        (0..steps).for_each(|step| {
            let rock_type = &rock_sequence[step % rock_sequence.len()];
            let x = match rock_type {
                RockType::Horizontal | RockType::Vertical | RockType::Square => 2,
                RockType::Cross => 3,
                RockType::LReverted => 4,
            };
            let length = match rock_type {
                RockType::Horizontal | RockType::Vertical => 4,
                RockType::Cross => 1,
                RockType::LReverted => 3,
                RockType::Square => 2,
            };
            let mut rock = Rock::new(
                rock_type.clone(),
                length,
                Position(screen.height as i64 + 3, x),
            );

            screen.fall(&mut rock, &mut commands);
        });

        screen.height
    }

    pub fn part_one() -> usize {
        Self::simulate(2022)
    }

    pub fn part_two() -> usize {
        // let mut result = 0;
        // let mut buffer_reader = BufReader::new(File::open("src/day17/input").unwrap());
        // let mut buffer = String::default();
        // buffer_reader.read_line(&mut buffer);
        // let mut occurrence_moments = HashMap::new();
        // let l = 50;
        //
        // (0..buffer.trim().len() - l).for_each(|i| {
        //     (0..l).for_each(|offset| {
        //         (*occurrence_moments
        //             .entry(&buffer[i..i + offset])
        //             .or_insert(0)) += 1;
        //     })
        // });
        //
        // result
        0
    }
}
