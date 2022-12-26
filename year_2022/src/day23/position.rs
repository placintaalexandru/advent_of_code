use crate::day23::direction::Direction;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub(crate) struct Position(pub(crate) i32, pub(crate) i32);

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        match *direction {
            Direction::North => Self(-1, 0),
            Direction::South => Self(1, 0),
            Direction::East => Self(0, 1),
            Direction::West => Self(0, -1),
            Direction::NorthEast => Self(-1, 1),
            Direction::NorthWest => Self(-1, -1),
            Direction::SouthEast => Self(1, 1),
            Direction::SouthWest => Self(1, -1),
        }
    }
}
