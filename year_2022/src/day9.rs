mod direction;
mod game;

use crate::day9::direction::Direction;
use crate::day9::game::Game;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Day9;

impl Day9 {
    fn play_part(n_knots: usize) -> usize {
        let mut result = HashSet::new();
        let mut game = Game::new(n_knots);
        let file = File::open("src/day9/input").unwrap();
        let buffer_reader = BufReader::new(file);

        buffer_reader.lines().for_each(|line| {
            let direction = Direction::from_str(line.as_ref().unwrap()).unwrap();

            // Convert, for example, R(4) to [R(1), R(1), R(1), R(1)]
            if let Some(one_step_directions) = direction.decompose() {
                one_step_directions
                    .into_iter()
                    .for_each(|one_step_direction| {
                        result.insert(game.apply_move(one_step_direction));
                    });
            }
        });

        result.len()
    }

    pub fn part_one() -> usize {
        Self::play_part(2)
    }

    pub fn part_two() -> usize {
        Self::play_part(10)
    }
}
