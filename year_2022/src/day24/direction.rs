use strum_macros::EnumIter;

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, Hash)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'v' => Self::Down,
            '^' => Self::Up,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl From<&Direction> for char {
    fn from(direction: &Direction) -> Self {
        match *direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}
