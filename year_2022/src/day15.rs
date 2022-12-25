use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Position(i32, i32);

impl Position {
    pub(crate) fn manhattan_distance(&self, other: &Self) -> usize {
        ((self.0 - other.0).abs() + (self.1 - other.1).abs()) as usize
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Interval(i32, i32);

impl Interval {
    fn overlap(&self, other: &Self) -> bool {
        if other.0 < self.0 {
            return other.overlap(self);
        }

        self.1 >= other.0 - 1
    }

    fn merge(self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.max(other.1))
    }

    fn merge_intervals<P: Ord + Debug>(signals: PriorityQueue<Interval, P>) -> Vec<Self> {
        let mut intervals = vec![];

        signals.into_sorted_iter().for_each(|(signal, _)| {
            if intervals.is_empty() {
                intervals.push(signal);
            } else {
                let last = intervals.pop().unwrap();

                if !last.overlap(&signal) {
                    intervals.push(last);
                    intervals.push(signal);
                } else {
                    intervals.push(last.merge(signal));
                }
            }
        });

        intervals
    }
}

#[derive(Debug)]
pub(crate) struct Scene {
    sensors: HashMap<Position, Position>,
}

impl Scene {
    fn parse(path: &str) -> Self {
        let buffer_reader = BufReader::new(File::open(path).unwrap());
        let mut scene = Self {
            sensors: Default::default(),
        };

        buffer_reader.lines().for_each(|line| {
            let content = line.unwrap();
            let tokens = content.split(' ').collect::<Vec<&str>>();
            let sensor = Position(
                (&tokens[2][2..tokens[2].len() - 1]).parse::<i32>().unwrap(),
                (&tokens[3][2..tokens[3].len() - 1]).parse::<i32>().unwrap(),
            );
            let beacon = Position(
                (&tokens[8][2..tokens[8].len() - 1]).parse::<i32>().unwrap(),
                (&tokens[9][2..tokens[9].len()]).parse::<i32>().unwrap(),
            );

            scene.sensors.insert(sensor, beacon);
        });

        scene
    }

    fn signals<F>(&self, line: i32, predicate: F) -> PriorityQueue<Interval, Reverse<i32>>
    where
        F: Fn(Interval) -> Interval,
    {
        self.sensors
            .iter()
            .flat_map(|(sensor, beacon)| {
                // Compute manhattan distance between sensor and beacon .It means that on the line
                // the sensor lies, the signal will look like this: `# * distance` S `# * distance`
                // This distance decreases by 2 with every line further to the line where the sensor is
                let distance = sensor.manhattan_distance(beacon);
                let distance_between_lines = (sensor.1 - line).abs() as usize;

                if distance_between_lines > distance {
                    None
                } else {
                    let interval = predicate(Interval(
                        sensor.0 - (distance - distance_between_lines) as i32,
                        sensor.0 + (distance - distance_between_lines) as i32,
                    ));
                    let priority = interval.0;

                    Some((interval, Reverse(priority)))
                }
            })
            .collect()
    }
}

pub struct Day15;

impl Day15 {
    pub fn part_one() -> usize {
        let scene = Scene::parse("src/day15/input");
        Interval::merge_intervals(scene.signals(2000000, |interval| interval))
            .into_iter()
            .fold(0, |acc, interval| acc + (interval.1 - interval.0) as usize)
    }

    fn restrained_search<T>(
        scene: &Scene,
        min_row: i32,
        max_row: i32,
        min_col: i32,
        max_col: i32,
        tuner: T,
    ) -> usize
    where
        T: Fn(usize, usize) -> usize,
    {
        for row in min_row..max_row {
            let signals = scene.signals(row, |interval| {
                Interval(
                    interval.0.clamp(min_col, max_col),
                    interval.1.clamp(min_col, max_col),
                )
            });
            let merged_intervals = Interval::merge_intervals(signals);

            if merged_intervals.len() == 1 {
                // All line, except last element is covered
                if merged_intervals[0].1 < max_col {
                    return tuner(row as usize, merged_intervals[0].1 as usize + 1);
                }

                // All line except last element is covered
                if merged_intervals[0].0 > min_col {
                    return tuner(row as usize, merged_intervals[0].0 as usize - 1);
                }
            } else {
                // There are more intervals [a; b], [c; d] and so on where
                // c - b >= 2 so we take column b + 1 (first column not covered by signal)
                return tuner(row as usize, merged_intervals[0].1 as usize + 1);
            }
        }

        unreachable!()
    }

    pub fn part_two() -> usize {
        let scene = Scene::parse("src/day15/input");
        Self::restrained_search(&scene, 0, 4000001, 0, 4000000, |row, col| {
            row + 4000000 * col
        })
    }
}
