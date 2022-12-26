use crate::day22::direction::Rotation;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum Action {
    Rotation(Rotation),
    Run(usize),
}

pub(crate) struct Actions(pub(crate) Vec<Action>);

impl FromStr for Actions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut actions = vec![];
        let mut buffer = String::default();

        s.chars().for_each(|c| {
            if c.is_digit(10) {
                buffer.push(c);
            } else {
                actions.push(Action::Run(buffer.parse::<usize>().unwrap()));
                actions.push(Action::Rotation(c.into()));
                buffer.clear();
            }
        });

        if !buffer.is_empty() {
            actions.push(Action::Run(buffer.parse::<usize>().unwrap()));
        }

        Ok(Self(actions))
    }
}
