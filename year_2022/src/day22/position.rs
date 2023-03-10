use crate::day22::direction::Direction;
use std::ops::{Add, AddAssign};

/// Keeps row and column
#[derive(Debug, Clone, Copy)]
pub(crate) struct Position(pub(crate) i32, pub(crate) i32);

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        match *direction {
            Direction::Left => Self(0, -1),
            Direction::Right => Self(0, 1),
            Direction::Up => Self(-1, 0),
            Direction::Down => Self(1, 0),
        }
    }
}
