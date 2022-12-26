use crate::day23::direction::Direction;
use crate::day23::position::Position;
use std::collections::HashSet;
use std::fmt::Display;
use strum::IntoEnumIterator;

pub(crate) enum Pixel {
    Elf,
    Land,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Elf,
            '.' => Self::Land,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Elf {
    pub(crate) position: Position,
}

impl Elf {
    pub(crate) fn new(position: Position) -> Self {
        Self { position }
    }

    pub(crate) fn can_move(&self, elves: &HashSet<Elf>) -> bool {
        Direction::iter()
            .any(|direction| elves.contains(&Elf::new(self.position + Position::from(&direction))))
    }

    pub(crate) fn destination(
        &self,
        direction: &Direction,
        elves: &HashSet<Self>,
    ) -> Option<Position> {
        direction
            .dependencies()
            .iter()
            .all(|dependency| {
                !elves.contains(&Elf::new(self.position + Position::from(dependency)))
            })
            .then_some(self.position + Position::from(direction))
    }
}
