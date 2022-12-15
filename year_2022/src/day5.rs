use std::fs::File;
use std::io;
use std::io::BufRead;

pub struct Day5;

impl Day5 {
    fn stacks() -> Vec<Vec<char>> {
        vec![
            vec!['H', 'R', 'B', 'D', 'Z', 'F', 'L', 'S'],
            vec!['T', 'B', 'M', 'Z', 'R'],
            vec!['Z', 'L', 'C', 'H', 'N', 'S'],
            vec!['S', 'C', 'F', 'J'],
            vec!['P', 'G', 'H', 'W', 'R', 'Z', 'B'],
            vec!['V', 'J', 'Z', 'G', 'D', 'N', 'M', 'T'],
            vec!['G', 'L', 'N', 'W', 'F', 'S', 'P', 'Q'],
            vec!['M', 'Z', 'R'],
            vec!['M', 'C', 'L', 'G', 'V', 'R', 'T'],
        ]
    }

    fn apply_move(stacks: &mut Vec<Vec<char>>, src: usize, dst: usize, amount: usize) {
        for _ in 0..amount {
            if stacks[src].is_empty() {
                break;
            }

            let c = stacks[src].pop().unwrap();
            stacks[dst].push(c);
        }
    }

    fn apply_move_multiple(stacks: &mut Vec<Vec<char>>, src: usize, dst: usize, amount: usize) {
        let mut intermediate = Vec::with_capacity(amount);

        for _ in 0..amount {
            if stacks[src].is_empty() {
                break;
            }

            intermediate.insert(0, stacks[src].pop().unwrap());
        }

        stacks[dst].extend(intermediate);
    }

    pub fn part_one() -> String {
        let file = File::open("src/day5/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut stacks = Self::stacks();

        for line in buffer_reader.lines() {
            let content = line.unwrap();
            let tokens = content.split(" ").collect::<Vec<&str>>();
            let (amount, src, dst) = (
                tokens[1].parse::<usize>().unwrap(),
                tokens[3].parse::<usize>().unwrap(),
                tokens[5].parse::<usize>().unwrap(),
            );

            Self::apply_move(&mut stacks, src - 1, dst - 1, amount);
        }

        stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap_or(&'-'))
            .collect()
    }

    pub fn part_two() -> String {
        let file = File::open("src/day5/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut stacks = Self::stacks();

        for line in buffer_reader.lines() {
            let content = line.unwrap();
            let tokens = content.split(" ").collect::<Vec<&str>>();
            let (amount, src, dst) = (
                tokens[1].parse::<usize>().unwrap(),
                tokens[3].parse::<usize>().unwrap(),
                tokens[5].parse::<usize>().unwrap(),
            );

            Self::apply_move_multiple(&mut stacks, src - 1, dst - 1, amount);
        }

        stacks
            .into_iter()
            .map(|stack| *stack.last().unwrap_or(&'-'))
            .collect()
    }
}
