use crate::day14::cave::{Cave, Outcome};

mod cave;
mod io;
mod position;

pub struct Day14;

impl Day14 {
    pub fn part_one() -> usize {
        let mut round = 0;
        let mut cave = Cave::parse("src/day14/input");

        loop {
            if cave.pour_sand().is_err() {
                break;
            }

            round += 1;
        }

        round
    }

    pub fn part_two() -> usize {
        let mut round = 0;
        let mut cave = Cave::parse("src/day14/input");

        // Add the 2 extra layers
        cave.push_air();
        cave.push_floor();

        loop {
            if let Err(Outcome::CellBusy) = cave.pour_sand2() {
                break;
            }

            round += 1;
        }

        round
    }
}
