use strum_macros::EnumIter;

#[derive(Debug, Eq, PartialEq, EnumIter)]
pub(crate) enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub(crate) fn dependencies(&self) -> [Self; 3] {
        match *self {
            Self::North => [Self::North, Self::NorthEast, Self::NorthWest],
            Self::South => [Self::South, Self::SouthEast, Self::SouthWest],
            Self::West => [Self::West, Self::NorthWest, Self::SouthWest],
            Self::East => [Self::East, Self::NorthEast, Self::SouthEast],
            _ => unreachable!(),
        }
    }
}
