use crate::day24::direction::Direction;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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
            Direction::Up => Self(-1, 0),
            Direction::Down => Self(1, 0),
            Direction::Right => Self(0, 1),
            Direction::Left => Self(0, -1),
        }
    }
}

impl Position {
    pub(crate) fn manhattan_distance(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}
