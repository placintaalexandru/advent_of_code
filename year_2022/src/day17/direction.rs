use std::ops::{Add, AddAssign, Neg};

/// Former element is line and the latter is the column
#[derive(Debug)]
pub(crate) struct Position(pub(crate) i64, pub(crate) i32);

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
            Direction::Down => Self(-1, 0),
            Direction::Up => Self(1, 0),
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

#[derive(Debug)]
pub(crate) enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unknown char `{}`", c),
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Up => Self::Down,
        }
    }
}
