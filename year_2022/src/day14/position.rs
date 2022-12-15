use std::str::FromStr;
use strum_macros::EnumIter;

/// Directions are sorted in the order of priorities.
/// When we insert sand, wego as down as possible,
/// then as down left as possible then as down right as possible
#[derive(Debug, Eq, PartialEq, EnumIter)]
pub(crate) enum Direction {
    Down,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Position(pub(crate) i16, pub(crate) i16);

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(',').collect::<Vec<&str>>();

        if tokens.len() != 2 {
            return Err(format!("Cannot parse {}", s));
        }

        Ok(Self(
            tokens[0].parse::<i16>().unwrap(),
            tokens[1].parse::<i16>().unwrap(),
        ))
    }
}

impl Position {
    /// Takes a list of positions and shifts to the left as much as possible
    ///
    /// # Examples
    ///
    /// [Position(5, 6), Position(6, 6)] -> [Position(0, 6), Position(1, 6)]
    pub(crate) fn shift_left(positions: &mut Vec<Vec<Self>>) -> i16 {
        let min_x = positions
            .iter()
            .map(|positions| positions.iter().map(|position| position.0).min().unwrap())
            .min()
            .unwrap();
        positions.iter_mut().for_each(|positions| {
            positions
                .iter_mut()
                .for_each(|position| position.0 -= min_x);
        });

        min_x
    }

    pub(crate) fn translate(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::DownLeft => Self(self.0 - 1, self.1 + 1),
            Direction::DownRight => Self(self.0 + 1, self.1 + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day14::position::Direction;
    use strum::IntoEnumIterator;

    #[test]
    fn direction_iteration() {
        assert_eq!(
            Direction::iter().collect::<Vec<_>>(),
            vec![Direction::Down, Direction::DownLeft, Direction::DownRight]
        );
    }
}
