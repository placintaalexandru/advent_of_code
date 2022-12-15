use crate::day11::game::Game;
use crate::day11::item::StartingItems;
use crate::day11::monkey::Monkey;
use crate::day11::operation::Play;
use crate::day11::test::{Action, Test};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

mod game;
mod item;
mod monkey;
mod operation;
mod test;

pub struct Day11;

impl Day11 {
    fn parse(file: &str, n: usize) -> Game {
        let mut monkeys = vec![];
        let mut buffer_reader = BufReader::new(File::open(file).unwrap());
        let mut s = String::default();

        loop {
            if monkeys.len() == n {
                break;
            }

            let content = buffer_reader.read_line(&mut s);

            if s.len() == 1 {
                s.clear();
                continue;
            }

            // End of file
            if content.is_err() {
                break;
            }

            // Don't need the first line Monkey number since they come ordered
            s.clear();

            buffer_reader.read_line(&mut s).unwrap();
            let trimmed = s.trim_end();

            let start_items = StartingItems::from_str(trimmed).unwrap();
            s.clear();

            buffer_reader.read_line(&mut s).unwrap();
            let trimmed = s.trim_end();
            let play = Play::from_str(trimmed).unwrap();
            s.clear();

            buffer_reader.read_line(&mut s).unwrap();
            let trimmed = s.trim_end();
            let test = Test::from_str(trimmed).unwrap();
            s.clear();

            buffer_reader.read_line(&mut s).unwrap();
            let trimmed = s.trim_end();
            let on_true_destination = trimmed.split(' ').last().unwrap().parse::<usize>().unwrap();
            s.clear();

            buffer_reader.read_line(&mut s).unwrap();
            let trimmed = s.trim_end();
            let on_false_destination = trimmed.split(' ').last().unwrap().parse::<usize>().unwrap();
            s.clear();

            let action = Action(test, on_true_destination, on_false_destination);

            monkeys.push(Monkey::new(start_items, play, action));
        }

        Game::new(monkeys)
    }

    pub fn part_one() -> usize {
        let mut game = Self::parse("src/day11/input", 8);

        (0..20).for_each(|_| {
            game.round_step(3, 9_699_690);
        });

        let stats = game.most_active_monkeys(2);

        stats[0] * stats[1]
    }

    pub fn part_two() -> u128 {
        let mut game = Self::parse("src/day11/input", 8);

        (0..10_000).for_each(|_| {
            game.round_step(1, 9_699_690);
        });

        let stats = game.most_active_monkeys(2);

        stats[0] as u128 * stats[1] as u128
    }
}
