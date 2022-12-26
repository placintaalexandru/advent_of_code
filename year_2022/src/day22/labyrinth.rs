use crate::day22::direction::Direction;
use crate::day22::position::Position;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::process::exit;

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) enum Pixel {
    Void,
    Land,
    Rock,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            ' ' => Self::Void,
            '.' => Self::Land,
            '#' => Self::Rock,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct Square(pub(crate) i32, pub(crate) Pixel);

pub(crate) struct Labyrinth {
    row_size: usize,
    pub(crate) rows: HashMap<i32, HashSet<Square>>,
}

impl Labyrinth {
    pub(crate) fn new(row_size: usize, rows: HashMap<i32, HashSet<Square>>) -> Self {
        Self { row_size, rows }
    }

    pub(crate) fn min_column(&self, row: i32) -> i32 {
        self.rows[&row].iter().map(|square| square.0).min().unwrap()
    }

    fn is_void(&self, position: &Position) -> bool {
        // Outside the grid on the vertical axis
        if !self.rows.contains_key(&position.0) {
            return true;
        }

        !self.rows[&position.0].contains(&Square(position.1, Pixel::Rock))
            && !self.rows[&position.0].contains(&Square(position.1, Pixel::Land))
    }

    pub(crate) fn move2d(
        &self,
        source: Position,
        direction: &Direction,
    ) -> Option<(Position, Direction)> {
        let new_position = source + Position::from(direction);

        if self.rows.contains_key(&new_position.0) {
            if (&self.rows[&new_position.0]).contains(&Square(new_position.1, Pixel::Land)) {
                return Some((new_position, direction.clone()));
            }

            if (&self.rows[&new_position.0]).contains(&Square(new_position.1, Pixel::Rock)) {
                return None;
            }
        }

        // Void => get to the other side of the wall
        let opposite_direction = -direction;
        let mut previous_position = source;

        loop {
            let destination = previous_position + Position::from(&opposite_direction);

            if self.is_void(&destination) {
                return self.rows[&previous_position.0]
                    .contains(&Square(previous_position.1, Pixel::Land))
                    .then_some((previous_position, direction.clone()));
            }

            previous_position = destination;
        }

        unreachable!()
    }

    pub(crate) fn move3d(
        &self,
        source: Position,
        direction: &Direction,
    ) -> Option<(Position, Direction)> {
        let (new_position, new_direction) = match *direction {
            Direction::Left => {
                if source.1 == 50 && source.0 >= 0 && source.0 < 50 {
                    (Position(149 - source.0, 0), Direction::Right)
                } else if source.1 == 50 && source.0 >= 50 && source.0 < 100 {
                    (Position(100, source.0 - 50), Direction::Down)
                } else if source.1 == 0 && source.0 >= 100 && source.0 < 150 {
                    (Position(149 - source.0, 50), Direction::Right)
                } else if source.1 == 0 && source.0 >= 150 && source.0 < 200 {
                    (Position(0, source.0 - 100), Direction::Down)
                } else {
                    (Position(source.0, source.1 - 1), Direction::Left)
                }
            }
            Direction::Right => {
                if source.1 == 149 && source.0 >= 0 && source.0 < 50 {
                    (Position(149 - source.0, 99), Direction::Left)
                } else if source.1 == 99 && source.0 >= 50 && source.0 < 100 {
                    (Position(49, 50 + source.0), Direction::Up)
                } else if source.1 == 99 && source.0 >= 100 && source.0 < 150 {
                    (Position(149 - source.0, 149), Direction::Left)
                } else if source.1 == 49 && source.0 >= 150 && source.0 < 200 {
                    (Position(149, source.0 - 100), Direction::Up)
                } else {
                    (Position(source.0, source.1 + 1), Direction::Right)
                }
            }
            Direction::Up => {
                if source.0 == 100 && source.1 >= 0 && source.1 < 50 {
                    (Position(50 + source.1, 50), Direction::Right)
                } else if source.0 == 0 && source.1 >= 50 && source.1 < 100 {
                    (Position(source.1 + 100, 0), Direction::Right)
                } else if source.0 == 0 && source.1 >= 100 && source.1 < 150 {
                    (Position(199, source.1 - 100), Direction::Up)
                } else {
                    (Position(source.0 - 1, source.1), Direction::Up)
                }
            }
            Direction::Down => {
                if source.0 == 49 && source.1 >= 100 && source.1 < 150 {
                    (Position(source.1 - 50, 99), Direction::Left)
                } else if source.0 == 149 && source.1 >= 50 && source.1 < 100 {
                    (Position(source.1 + 100, 49), Direction::Left)
                } else if source.0 == 199 && source.1 >= 0 && source.1 < 50 {
                    (Position(0, source.1 + 100), Direction::Down)
                } else {
                    (Position(source.0 + 1, source.1), Direction::Down)
                }
            }
        };

        (self.rows[&new_position.0].contains(&Square(new_position.1, Pixel::Land)))
            .then_some((new_position, new_direction))
    }
}
