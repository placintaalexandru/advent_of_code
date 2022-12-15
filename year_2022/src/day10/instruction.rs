use std::str::FromStr;

pub(crate) enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(" ").collect::<Vec<&str>>();

        match tokens[0] {
            "noop" => Ok(Self::Noop),
            "addx" => match tokens.len() {
                2 => Ok(Self::AddX(tokens[1].parse::<i32>().unwrap())),
                _ => Err(format!("Cannot parse Addx from {}", s)),
            },
            _ => Err(format!("Cannot anything from {}", s)),
        }
    }
}

impl Instruction {
    pub(crate) fn register_increment(&self, register: i32) -> i32 {
        match self {
            Self::Noop => register,
            Self::AddX(val) => register + *val,
        }
    }

    pub(crate) fn cycle_increment(&self, cycle: usize) -> usize {
        match self {
            Self::Noop => cycle + 1,
            Self::AddX(_) => cycle + 2,
        }
    }
}
