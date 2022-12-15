use crate::day11::monkey::Monkey;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub(crate) struct Destination {
    destination: usize,
    stress_level: usize,
}

impl Destination {
    pub(crate) fn new(destination: usize, stress_level: usize) -> Self {
        Self {
            destination,
            stress_level,
        }
    }
}

pub(crate) struct Game {
    monkeys: Vec<Monkey>,
    stats: Vec<usize>,
}

impl Game {
    pub(crate) fn new(monkeys: Vec<Monkey>) -> Self {
        let stats = vec![0; monkeys.len()];

        Self { monkeys, stats }
    }

    /// `modulo_reducer` is used to keep numbers small
    /// is the smallest common multiple of all monkeys' divisible number
    pub(crate) fn round_step(&mut self, worry_level: usize, modulo_reducer: usize) {
        for i in 0..self.monkeys.len() {
            if let Some(destinations) = self.monkeys[i].round_step(worry_level, modulo_reducer) {
                self.stats[i] += destinations.len();

                destinations.into_iter().for_each(|destination| {
                    let monkey = &mut self.monkeys[destination.destination];
                    monkey.push_back(destination.stress_level);
                });
            }
        }
    }

    pub(crate) fn most_active_monkeys(&self, n: usize) -> Vec<usize> {
        let mut queue: PriorityQueue<usize, Reverse<usize>> = PriorityQueue::new();
        self.stats.iter().for_each(|n_processed_items| {
            queue.push(*n_processed_items, Reverse(*n_processed_items));

            if queue.len() > n {
                queue.pop();
            }
        });

        queue.into_iter().map(|(e, _priority)| e).collect()
    }
}
