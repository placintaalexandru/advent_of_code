use crate::day22::action::Action;
use crate::day22::direction::{Direction, Rotation};
use crate::day22::position::Position;

pub(crate) struct Character {
    pub(crate) direction: Direction,
    pub(crate) position: Position,
}

impl Character {
    pub(crate) fn new(direction: Direction, position: Position) -> Self {
        Self {
            direction,
            position,
        }
    }

    pub(crate) fn rotate(&mut self, rotation: Rotation) {
        self.direction = self.direction.rotate(rotation);
    }

    pub(crate) fn act<P>(&mut self, action: Action, predicate: P)
    where
        P: Fn(Position, &Direction) -> Option<(Position, Direction)>,
    {
        match action {
            Action::Rotation(rotation) => self.rotate(rotation),
            Action::Run(mut tiles) => {
                while tiles > 0 {
                    if let Some((new_position, direction)) =
                        predicate(self.position, &self.direction)
                    {
                        self.position = new_position;
                        self.direction = direction;
                        tiles -= 1;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}
