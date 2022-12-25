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

pub(crate) enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub(crate) fn rotate(self, rotation: Rotation) -> Self {
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
