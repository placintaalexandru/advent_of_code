use crate::day9::game::GameType;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub(crate) enum Direction {
    Up(GameType),
    Down(GameType),
    Left(GameType),
    Right(GameType),
}

impl Direction {
    pub(crate) fn decompose(self) -> Option<Vec<Self>> {
        match self {
            Self::Up(val) => (val > 0).then_some(vec![Self::Up(1); val as usize]),
            Self::Down(val) => (val > 0).then_some(vec![Self::Down(1); val as usize]),
            Self::Left(val) => (val > 0).then_some(vec![Self::Left(1); val as usize]),
            Self::Right(val) => (val > 0).then_some(vec![Self::Right(1); val as usize]),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(" ").collect::<Vec<&str>>();

        if tokens.len() != 2 {
            return Err(s.to_owned());
        }

        match tokens[0] {
            "U" => Ok(Self::Up(tokens[1].parse::<GameType>().unwrap())),
            "L" => Ok(Self::Left(tokens[1].parse::<GameType>().unwrap())),
            "R" => Ok(Self::Right(tokens[1].parse::<GameType>().unwrap())),
            "D" => Ok(Self::Down(tokens[1].parse::<GameType>().unwrap())),
            _ => Err(s.to_owned()),
        }
    }
}
