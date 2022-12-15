use crate::day12::grid::Grid;

mod grid;

pub struct Day12;

impl Day12 {
    pub fn part_one() -> usize {
        let grid = Grid::parse("src/day12/input");
        let start = grid.first_position('S').unwrap();
        let end = grid.first_position('E').unwrap();
        let distances = grid.djikstra(&end);

        distances[&start]
    }

    pub fn part_two() -> usize {
        let grid = Grid::parse("src/day12/input");
        let end = grid.first_position('E').unwrap();
        let distances = grid.djikstra(&end);
        let mut result = usize::MAX;

        distances.into_iter().for_each(|(position, distance)| {
            if grid.char(position) == 'a' {
                result = result.min(distance);
            }
        });

        result
    }
}
