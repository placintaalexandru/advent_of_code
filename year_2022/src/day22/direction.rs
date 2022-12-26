use std::ops::Neg;

#[derive(Debug)]
pub(crate) enum Rotation {
    Left,
    Right,
}

impl From<char> for Rotation {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Neg for &Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl From<&Direction> for usize {
    fn from(direction: &Direction) -> Self {
        match *direction {
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Up => 3,
            Direction::Down => 1,
        }
    }
}

impl Direction {
    pub(crate) fn rotate(&self, rotation: Rotation) -> Self {
        match self {
            Direction::Left => match rotation {
                Rotation::Right => Self::Up,
                Rotation::Left => Self::Down,
            },
            Direction::Right => match rotation {
                Rotation::Left => Self::Up,
                Rotation::Right => Self::Down,
            },
            Direction::Up => match rotation {
                Rotation::Left => Self::Left,
                Rotation::Right => Self::Right,
            },
            Direction::Down => match rotation {
                Rotation::Left => Self::Right,
                Rotation::Right => Self::Left,
            },
        }
    }
}
