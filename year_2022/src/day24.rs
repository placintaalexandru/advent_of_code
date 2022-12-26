use crate::day24::blizzard::Blizzard;
use crate::day24::direction::Direction;
use crate::day24::grid::Grid;
use crate::day24::pixel::Pixel;
use crate::day24::position::Position;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod blizzard;
mod direction;
mod grid;
mod pixel;
mod position;

pub struct Day24;

impl Day24 {
    fn parse(path: &str) -> Grid {
        let mut grid_rows = vec![];
        let mut num_columns = 0;

        BufReader::new(File::open(path).unwrap())
            .lines()
            .enumerate()
            .for_each(|(i, line)| {
                let content = line.unwrap();
                num_columns = num_columns.max(content.len());

                if i == 0 {
                    grid_rows.push(vec![Pixel::Rock.clone(); num_columns]);
                }

                grid_rows.push(
                    content
                        .chars()
                        .enumerate()
                        .map(|(j, c)| match c {
                            '.' => Pixel::Land,
                            '#' => Pixel::Rock,
                            _ => Pixel::Blizzard(Blizzard::new(
                                Position(i as i32 + 1, j as i32),
                                Direction::from(c),
                            )),
                        })
                        .collect(),
                );
            });

        grid_rows.push(vec![Pixel::Rock.clone(); num_columns]);
        Grid::new(num_columns, grid_rows)
    }

    pub fn part_one() -> usize {
        let grid = Self::parse("src/day24/input");

        let (distance, _) = grid
            .a_star(
                Position(1, 1),
                Position(grid.pixels.len() as i32 - 2, grid.width as i32 - 2),
                &|a, b| a.manhattan_distance(b),
            )
            .unwrap();

        distance
    }

    pub fn part_two() -> usize {
        let grid = Self::parse("src/day24/input");

        let (distance1, weather_map1) = grid
            .a_star(
                Position(1, 1),
                Position(grid.pixels.len() as i32 - 2, grid.width as i32 - 2),
                &|a, b| a.manhattan_distance(b),
            )
            .unwrap();

        let (distance2, weather_map2) = weather_map1
            .a_star(
                Position(grid.pixels.len() as i32 - 2, grid.width as i32 - 2),
                Position(1, 1),
                &|a, b| a.manhattan_distance(b),
            )
            .unwrap();

        let (distance3, _) = weather_map2
            .a_star(
                Position(1, 1),
                Position(grid.pixels.len() as i32 - 2, grid.width as i32 - 2),
                &|a, b| a.manhattan_distance(b),
            )
            .unwrap();

        distance1 + distance2 + distance3
    }
}
