use crate::day18::grid::{Coordinate, Grid};
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod grid;

pub struct Day18;

impl Day18 {
    pub fn part_one() -> usize {
        let mut grid = Grid::default();
        let buffer_reader = BufReader::new(File::open("src/day18/input").unwrap());

        buffer_reader.lines().for_each(|line| {
            if let Ok((x, y, z)) = scan_fmt!(line.as_ref().unwrap(), "{d},{d},{d}", i8, i8, i8) {
                grid.add(Coordinate(x, y, z));
            }
        });

        grid.surface()
    }

    pub fn part_two() -> usize {
        let mut grid = Grid::default();
        let buffer_reader = BufReader::new(File::open("src/day18/input").unwrap());
        let mut water_regions = HashSet::new();

        buffer_reader.lines().for_each(|line| {
            if let Ok((x, y, z)) = scan_fmt!(line.as_ref().unwrap(), "{d},{d},{d}", i8, i8, i8) {
                grid.add(Coordinate(x, y, z));
            }
        });

        grid.bot_left = Coordinate(
            grid.bot_left.0 - 1,
            grid.bot_left.1 - 1,
            grid.bot_left.2 - 1,
        );
        grid.top_right = Coordinate(
            grid.top_right.0 + 1,
            grid.top_right.1 + 1,
            grid.top_right.2 + 1,
        );

        grid.flood(grid.top_right.clone(), &mut water_regions);

        grid.cubes
            .into_iter()
            .map(|coordinate| {
                [
                    water_regions.contains(&Coordinate(
                        coordinate.0 - 1,
                        coordinate.1,
                        coordinate.2,
                    )) as usize,
                    water_regions.contains(&Coordinate(
                        coordinate.0 + 1,
                        coordinate.1,
                        coordinate.2,
                    )) as usize,
                    water_regions.contains(&Coordinate(
                        coordinate.0,
                        coordinate.1 - 1,
                        coordinate.2,
                    )) as usize,
                    water_regions.contains(&Coordinate(
                        coordinate.0,
                        coordinate.1 + 1,
                        coordinate.2,
                    )) as usize,
                    water_regions.contains(&Coordinate(
                        coordinate.0,
                        coordinate.1,
                        coordinate.2 - 1,
                    )) as usize,
                    water_regions.contains(&Coordinate(
                        coordinate.0,
                        coordinate.1,
                        coordinate.2 + 1,
                    )) as usize,
                ]
                .iter()
                .sum::<usize>()
            })
            .sum()
    }
}
