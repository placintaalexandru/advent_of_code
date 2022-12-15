use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub(crate) struct Position(pub(crate) i16, pub(crate) i16);

#[derive(Debug)]
pub(crate) struct Grid {
    heights: Vec<Vec<char>>,
}

impl Grid {
    pub(crate) fn parse(path: &str) -> Self {
        let buffer_reader = BufReader::new(File::open(path).unwrap());
        let mut lines = vec![];

        buffer_reader.lines().for_each(|line| {
            lines.push(line.unwrap().chars().collect());
        });

        Self { heights: lines }
    }

    fn elevation(&self, position: &Position) -> char {
        if self.is_start(position) {
            return 'a';
        }

        if self.is_end(position) {
            return 'z';
        }

        self.heights[position.0 as usize][position.1 as usize]
    }

    pub(crate) fn first_position(&self, c: char) -> Option<Position> {
        for (i, line) in self.heights.iter().enumerate() {
            for (j, height) in line.iter().enumerate() {
                if *height == c {
                    return Some(Position(i as i16, j as i16));
                }
            }
        }

        None
    }

    fn is_char(&self, c: char, position: &Position) -> bool {
        self.heights[position.0 as usize][position.1 as usize] == c
    }

    fn is_start(&self, position: &Position) -> bool {
        self.is_char('S', position)
    }

    fn is_end(&self, position: &Position) -> bool {
        self.is_char('E', position)
    }

    fn valid_neighbor(&self, source: &Position, destination: &Position) -> bool {
        if destination.0 >= self.heights.len() as i16
            || destination.0 < 0
            || destination.1 >= self.heights[0].len() as i16
            || destination.1 < 0
        {
            return false;
        }

        // Take the condition from problem statement and translate it so the djikstra
        // can be performed from destination to source
        self.elevation(destination) as i16 >= self.elevation(source) as i16 - 1
    }

    fn neighbors(&self, position: &Position) -> Option<Vec<Position>> {
        let (row, column) = (position.0, position.1);
        let result = vec![
            Position(row - 1, column),
            Position(row + 1, column),
            Position(row, column - 1),
            Position(row, column + 1),
        ];

        let result = result
            .into_iter()
            .filter(|neighbor| self.valid_neighbor(position, neighbor))
            .collect::<Vec<_>>();

        (!result.is_empty()).then_some(result)
    }

    pub(crate) fn djikstra(&self, source: &Position) -> HashMap<Position, usize> {
        let mut distances = HashMap::new();
        let mut queue: PriorityQueue<Position, Reverse<usize>> = PriorityQueue::new();

        distances.insert(source.clone(), 0);
        queue.push(source.clone(), Reverse(0));

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            let new_distance = distances[&node.0] + 1;

            if let Some(neighbors) = self.neighbors(&node.0) {
                neighbors.into_iter().for_each(|neighbor| {
                    if let Some(current_distance) = distances.get(&neighbor) {
                        if *current_distance > new_distance {
                            // Better path found up to `neighbor`
                            distances.insert(neighbor.clone(), new_distance);
                            queue.push(neighbor.clone(), Reverse(new_distance));
                        }
                    } else {
                        // First encounter of `neighbor`
                        distances.insert(neighbor.clone(), new_distance);
                        queue.push(neighbor.clone(), Reverse(new_distance));
                    }
                });
            }
        }

        distances
    }

    pub(crate) fn char(&self, position: Position) -> char {
        self.heights[position.0 as usize][position.1 as usize]
    }
}
