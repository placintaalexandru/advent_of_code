use crate::day24::direction::Direction;
use crate::day24::grid::Grid;
use crate::day24::position::Position;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Blizzard {
    pub(crate) position: Position,
    pub(crate) direction: Direction,
}

impl Blizzard {
    pub(crate) fn new(position: Position, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    pub(crate) fn change_position(&mut self, grid: &Grid) {
        self.position += Position::from(&self.direction);

        match self.direction {
            Direction::Up | Direction::Down => {
                self.position.0 = (self.position.0 - 2).rem_euclid(grid.pixels.len() as i32 - 4) + 2
            }
            Direction::Left | Direction::Right => {
                self.position.1 = (self.position.1 - 1).rem_euclid(grid.width as i32 - 2) + 1
            }
        }
    }
}
