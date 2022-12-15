mod instruction;

pub struct Day10;

use crate::day10::instruction::Instruction;
use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const LINE_LEN: usize = 40;

struct Sprite {
    register: i32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self { register: 1 }
    }
}

impl Sprite {
    fn increment_register(&mut self, instruction: Instruction) {
        self.register = instruction.register_increment(self.register);
    }

    fn char(&self, position: usize) -> char {
        let position = (position % LINE_LEN) as i32;

        if position < self.register - 1 || position > self.register + 1 {
            return '.';
        }

        '#'
    }
}

impl Day10 {
    pub fn part_one() -> i32 {
        let mut cycles_of_interest = LinkedList::from([20, 60, 100, 140, 180, 220]);
        let mut cycle_now = 0;
        let mut register_now = 1;
        let mut result = 0;
        let buf_reader = BufReader::new(File::open("src/day10/input").unwrap());

        for line in buf_reader.lines() {
            if cycles_of_interest.is_empty() {
                break;
            }

            let cycle_of_interest = *cycles_of_interest.front().unwrap();
            let register_before = register_now;

            let instruction = Instruction::from_str(line.as_ref().unwrap()).unwrap();

            cycle_now = instruction.cycle_increment(cycle_now);
            register_now = instruction.register_increment(register_now);

            // After applying the instruction I end after the cycle
            if cycle_now >= cycle_of_interest {
                result += cycle_of_interest as i32 * register_before;
                cycles_of_interest.pop_front();
            }
        }

        result
    }

    pub fn part_two() -> String {
        let buf_reader = BufReader::new(File::open("src/day10/input").unwrap());
        let mut chars = vec![];
        let mut result = String::default();
        let mut sprite = Sprite::default();
        let mut cycle_now = 0;

        buf_reader.lines().for_each(|line| {
            let instruction = Instruction::from_str(line.as_ref().unwrap()).unwrap();

            let cycle_future = instruction.cycle_increment(cycle_now);

            (cycle_now..cycle_future).for_each(|cycle| {
                chars.push(sprite.char(cycle));
            });

            sprite.increment_register(instruction);
            cycle_now = cycle_future;
        });

        chars.chunks(LINE_LEN).for_each(|chunk| {
            result.extend(chunk);
            result.push('\n');
        });

        result
    }
}
