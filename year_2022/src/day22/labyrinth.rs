use std::collections::HashMap;

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

pub(crate) struct Square(usize, Pixel);

pub(crate) struct Labyrinth {
    row_size: usize,
    rows: HashMap<i32, Square>,
}
