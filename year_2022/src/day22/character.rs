use crate::day15::Position;
use crate::day22::direction::{Direction, Rotation};
use crate::day22::labyrinth::Labyrinth;

pub(crate) struct Character {
    pub(crate) direction: Direction,
    pub(crate) position: Position,
}

pub(crate) enum Action {
    Rotation(Rotation),
    Run(usize),
}

impl Character {
    pub(crate) fn rotate(&mut self, rotation: Rotation) {
        self.direction = self.direction.rotate(rotation);
    }

    pub(crate) fn act(&mut self, labyrinth: Labyrinth, action: Action) {
        match action {
            Action::Rotation(rotation) => self.rotate(rotation),
            Action::Run(tiles) => {}
        }
    }
}
