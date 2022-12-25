use crate::day17::direction::Direction;
use std::str::FromStr;

pub(crate) struct Commands {
    commands: Vec<Direction>,
    cursor: usize,
}

impl FromStr for Commands {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut commands = vec![];

        s.chars().for_each(|c| commands.push(c.into()));

        Ok(Self {
            commands,
            cursor: 0,
        })
    }
}

impl Commands {
    pub(crate) fn next(&mut self) -> &Direction {
        let result = &self.commands[self.cursor];
        self.cursor = (self.cursor + 1) % self.commands.len();
        result
    }
}
