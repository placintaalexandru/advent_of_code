use crate::day24::direction::Direction;
use crate::day24::pixel::Pixel;
use crate::day24::position::Position;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

pub(crate) type Heuristic = dyn Fn(&Position, &Position) -> i32;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Grid {
    pub(crate) width: usize,
    pub(crate) pixels: Vec<Vec<Pixel>>,
}

impl Grid {
    pub(crate) fn new(width: usize, pixels: Vec<Vec<Pixel>>) -> Self {
        Self { width, pixels }
    }

    pub(crate) fn step(&self) -> Self {
        let mut new_grid = vec![vec![Pixel::Land; self.width].clone(); self.pixels.len()];
        let mut blizzard_map = HashMap::new();

        self.pixels.iter().enumerate().for_each(|(i, pixels)| {
            pixels
                .iter()
                .enumerate()
                .for_each(|(j, pixel)| match pixel {
                    Pixel::Blizzard(blizzard) => {
                        let mut blizzard = blizzard.clone();
                        blizzard.change_position(self);

                        blizzard_map
                            .entry(blizzard.position)
                            .or_insert_with(Vec::new)
                            .push(blizzard);
                    }
                    Pixel::Blizzards(blizzards) => {
                        blizzards.iter().for_each(|blizzard| {
                            let mut blizzard = blizzard.clone();
                            blizzard.change_position(self);

                            blizzard_map
                                .entry(blizzard.position)
                                .or_insert_with(Vec::new)
                                .push(blizzard);
                        });
                    }
                    Pixel::Rock => {
                        *new_grid.get_mut(i).unwrap().get_mut(j).unwrap() = Pixel::Rock;
                    }
                    Pixel::Land => {}
                });
        });

        blizzard_map
            .into_iter()
            .for_each(|(position, mut blizzards)| {
                if blizzards.len() == 1 {
                    *new_grid
                        .get_mut(position.0 as usize)
                        .unwrap()
                        .get_mut(position.1 as usize)
                        .unwrap() = Pixel::Blizzard(blizzards.pop().unwrap());
                } else if blizzards.len() > 1 {
                    *new_grid
                        .get_mut(position.0 as usize)
                        .unwrap()
                        .get_mut(position.1 as usize)
                        .unwrap() = Pixel::Blizzards(blizzards);
                }
            });

        Self::new(self.width, new_grid)
    }

    fn neighbors(
        &self,
        position: &Position,
        destination: &Position,
        h: &Heuristic,
    ) -> PriorityQueue<Position, i32> {
        let next_state = self.step();
        let mut result = PriorityQueue::new();

        Direction::iter().for_each(|direction| {
            let neighbor = *position + Position::from(&direction);

            // Check if any storm will come at destination
            if next_state.pixels[neighbor.0 as usize][neighbor.1 as usize] == Pixel::Land {
                let heuristic = h(&neighbor, destination);
                result.push(neighbor, heuristic);
            }
        });

        result
    }

    pub(crate) fn a_star(
        &self,
        start_position: Position,
        destination: Position,
        heuristic: &Heuristic,
    ) -> Option<(usize, Self)> {
        let mut open: PriorityQueue<(Position, Grid, usize), Reverse<usize>> =
            PriorityQueue::from(vec![(
                (start_position, self.clone(), 1),
                Reverse(heuristic(&start_position, &destination) as usize),
            )]);

        while !open.is_empty() {
            let ((position, weather_map, moment), time_taken) = open.pop().unwrap();

            // Return distance and state of the map for part 2
            if position == destination {
                return Some((time_taken.0, weather_map));
            }

            let mut neighbors = weather_map.neighbors(&position, &destination, heuristic);
            let next_state = weather_map.step();

            // Take into account staying in the same position, but add 1 because of waiting
            if next_state.pixels[position.0 as usize][position.1 as usize] == Pixel::Land {
                open.push(
                    (position, next_state.clone(), moment + 1),
                    Reverse(time_taken.0 + 1),
                );
            }

            while !neighbors.is_empty() {
                let (neighbor, heuristic_estimation) = neighbors.pop().unwrap();
                open.push(
                    (neighbor, next_state.clone(), moment + 1),
                    Reverse(moment + heuristic_estimation as usize),
                );
            }
        }

        None
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|pixel| char::from(pixel))
                        .collect::<String>()
                })
                .intersperse("\n".to_owned())
                .collect::<String>()
        )
    }
}
