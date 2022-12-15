use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Eq, PartialEq)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn weak_against(&self) -> Self {
        match *self {
            Self::Scissors => Self::Rock,
            Self::Paper => Self::Scissors,
            Self::Rock => Self::Paper,
        }
    }

    fn strong_against(&self) -> Self {
        match *self {
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
            Self::Rock => Self::Scissors,
        }
    }

    fn versus(&self, other: &Self) -> WinLoss {
        if *self == *other {
            return WinLoss::Draw;
        }

        if *other == self.strong_against() {
            return WinLoss::Win;
        }

        WinLoss::Loss
    }
}

impl From<RockPaperScissors> for usize {
    fn from(rps: RockPaperScissors) -> Self {
        rps as Self + 1
    }
}

#[derive(EnumString)]
enum Code {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

impl From<Code> for RockPaperScissors {
    fn from(code: Code) -> Self {
        match code {
            Code::A | Code::X => Self::Rock,
            Code::B | Code::Y => Self::Paper,
            Code::C | Code::Z => Self::Scissors,
        }
    }
}

#[derive(Debug)]
enum WinLoss {
    Win,
    Loss,
    Draw,
}

impl From<WinLoss> for usize {
    fn from(win_loss: WinLoss) -> Self {
        match win_loss {
            WinLoss::Win => 6,
            WinLoss::Loss => 0,
            WinLoss::Draw => 3,
        }
    }
}

pub struct Day2;

impl Day2 {
    pub fn part_one() -> usize {
        let file = File::open("src/day2/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut result = 0_usize;

        for line in buffer_reader.lines() {
            let content = line.unwrap();

            if content.is_empty() {
                continue;
            }

            let tokens = content.split(" ").collect::<Vec<&str>>();
            let choice1 = RockPaperScissors::from(Code::from_str(tokens[0]).unwrap());
            let choice2 = RockPaperScissors::from(Code::from_str(tokens[1]).unwrap());

            result += usize::from(choice2.versus(&choice1)) + usize::from(choice2);
        }

        result
    }

    pub fn part_two() -> usize {
        let file = File::open("src/day2/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut result = 0_usize;

        for line in buffer_reader.lines() {
            let content = line.unwrap();

            if content.is_empty() {
                continue;
            }

            let tokens = content.split(" ").collect::<Vec<&str>>();
            let choice1 = RockPaperScissors::from(Code::from_str(tokens[0]).unwrap());
            let choice2 = Code::from_str(tokens[1]).unwrap();

            result += match choice2 {
                Code::X => usize::from(choice1.strong_against()) + usize::from(WinLoss::Loss),
                Code::Y => usize::from(choice1) + usize::from(WinLoss::Draw),
                Code::Z => usize::from(choice1.weak_against()) + usize::from(WinLoss::Win),
                _ => unreachable!(),
            };
        }

        result
    }
}
