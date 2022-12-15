use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Day1;

impl Day1 {
    pub fn run() -> usize {
        let file = File::open("src/day1/input").unwrap();
        let buffer_reader = io::BufReader::new(file);
        let mut queue: PriorityQueue<usize, Reverse<usize>> = PriorityQueue::new();
        let mut crt_calories = 0_usize;

        for line in buffer_reader.lines() {
            let content = line.unwrap();

            if content.is_empty() {
                queue.push(crt_calories, Reverse(crt_calories));
                crt_calories = 0;

                if queue.len() > 3 {
                    queue.pop();
                }

                continue;
            }

            crt_calories += content.parse::<usize>().unwrap();
        }

        queue.push(crt_calories, Reverse(crt_calories));

        if queue.len() > 3 {
            queue.pop();
        }

        queue.into_iter().fold(0, |acc, (e, _)| acc + e)
    }
}
