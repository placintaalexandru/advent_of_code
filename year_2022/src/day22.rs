use crate::day22::action::Actions;
use crate::day22::character::Character;
use crate::day22::direction::Direction;
use crate::day22::labyrinth::{Labyrinth, Pixel, Square};
use crate::day22::position::Position;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod action;
mod character;
mod direction;
mod labyrinth;
mod position;

pub struct Day22;

impl Day22 {
    fn lines(path: &str) -> Vec<String> {
        BufReader::new(File::open(path).unwrap())
            .lines()
            .filter_map(|line| {
                let content = line.unwrap();
                (!content.is_empty()).then_some(content)
            })
            .collect()
    }

    fn labyrinth(lines: &[String]) -> Labyrinth {
        let row_size = lines[0].len();
        let mut rows = HashMap::new();

        lines.iter().enumerate().for_each(|(i, line)| {
            let mut row = HashSet::new();

            for (j, c) in (*line).chars().enumerate() {
                let pixel = c.into();

                if pixel == Pixel::Void {
                    continue;
                }

                row.insert(Square(j as i32, pixel));
            }

            rows.insert(i as i32, row);
        });

        Labyrinth::new(row_size, rows)
    }

    pub fn part_one() -> usize {
        let lines = Self::lines("src/day22/input");
        let labyrinth = Self::labyrinth(&lines[0..lines.len() - 1]);
        let commands = Actions::from_str(&lines[lines.len() - 1]).unwrap();
        let mut character = Character::new(Direction::Right, Position(0, labyrinth.min_column(0)));

        commands.0.into_iter().for_each(|action| {
            character.act(action, |position, direction| {
                labyrinth.move2d(position, direction)
            });
        });

        (character.position.0 + 1) as usize * 1000
            + (character.position.1 + 1) as usize * 4
            + usize::from(&character.direction)
    }

    pub fn part_two() -> usize {
        let lines = Self::lines("src/day22/input");
        let labyrinth = Self::labyrinth(&lines[0..lines.len() - 1]);
        let commands = Actions::from_str(&lines[lines.len() - 1]).unwrap();
        let mut character = Character::new(Direction::Right, Position(0, labyrinth.min_column(0)));

        commands.0.into_iter().for_each(|action| {
            character.act(action, |position, direction| {
                labyrinth.move3d(position, direction)
            });
        });

        (character.position.0 + 1) as usize * 1000
            + (character.position.1 + 1) as usize * 4
            + usize::from(&character.direction)
    }
}
