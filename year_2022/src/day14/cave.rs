use crate::day14::io::read_lines;
use crate::day14::position::{Direction, Position};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;

#[derive(Clone, Eq, PartialEq)]
enum Element {
    Nothing,
    Rock,
    Sand,
    RoofCrack,
}

#[derive(Eq, PartialEq)]
pub(crate) enum Outcome {
    OutsideTheGrid,
    CellEmpty,
    CellBusy,
}

impl From<Element> for char {
    fn from(element: Element) -> Self {
        match element {
            Element::Rock => '#',
            Element::Nothing => '.',
            Element::Sand => 'o',
            Element::RoofCrack => '+',
        }
    }
}

pub(crate) struct Cave {
    grid: Vec<Vec<Element>>,
    sand_source: Position,
}

impl Cave {
    fn draw_line(&mut self, pos1: &Position, pos2: &Position) {
        // Same row
        if pos1.1 == pos2.1 {
            let y = pos1.1 as usize;
            (pos1.0.min(pos2.0)..pos1.0.max(pos2.0) + 1).for_each(|x| {
                self.grid[y][x as usize] = Element::Rock;
            });
        } else if pos1.0 == pos2.0 {
            let x = pos1.0 as usize;
            (pos1.1.min(pos2.1)..pos1.1.max(pos2.1) + 1).for_each(|y| {
                self.grid[y as usize][x] = Element::Rock;
            });
        }
    }

    pub(crate) fn parse(path: &str) -> Self {
        // Read original lines
        let mut positions = read_lines(path);

        // Shift left all positions
        let min_x = Position::shift_left(&mut positions);

        let max_y = positions
            .iter()
            .map(|positions| positions.iter().map(|position| position.1).max().unwrap())
            .max()
            .unwrap();
        let max_x = positions
            .iter()
            .map(|positions| positions.iter().map(|position| position.0).max().unwrap())
            .max()
            .unwrap();
        let mut grid = vec![vec![Element::Nothing; (max_x + 1) as usize]; (max_y + 1) as usize];

        grid[0][(500 - min_x) as usize] = Element::RoofCrack;

        let mut cave = Self {
            grid,
            sand_source: Position(500 - min_x, 0),
        };

        positions.into_iter().for_each(|positions| {
            for i in 0..positions.len() - 1 {
                cave.draw_line(&positions[i], &positions[i + 1]);
            }
        });

        cave
    }

    /// Checks the outcome of trying to fill the position with sand
    fn outcome(&self, position: &Position) -> Outcome {
        if position.0 < 0
            || position.0 as usize >= self.grid[0].len()
            || position.1 as usize >= self.grid.len()
        {
            return Outcome::OutsideTheGrid;
        }

        let element = &self.grid[position.1 as usize][position.0 as usize];

        if *element == Element::Rock || *element == Element::Sand {
            return Outcome::CellBusy;
        }

        Outcome::CellEmpty
    }

    fn push(&mut self, position: &Position, element: Element) {
        self.grid[position.1 as usize][position.0 as usize] = element
    }

    fn sand_fall(&mut self, current_position: Position) -> Result<Position, Outcome> {
        match self.outcome(&current_position) {
            // Outside the map -> we stop
            Outcome::OutsideTheGrid => Err(Outcome::OutsideTheGrid),

            // Current cell is empty -> check if we can follow
            Outcome::CellEmpty => {
                for new_direction in Direction::iter() {
                    let fall_result = self.sand_fall(current_position.translate(&new_direction));

                    if matches!(fall_result, Err(Outcome::OutsideTheGrid)) {
                        return fall_result;
                    }

                    if fall_result.is_ok() {
                        return fall_result;
                    }
                }

                self.push(&current_position, Element::Sand);

                Ok(current_position)
            }

            // Current cell is busy, we stop
            Outcome::CellBusy => Err(Outcome::CellBusy),
        }
    }

    fn border_left(&mut self) {
        let limit = self.grid.len() - 1;

        // Add one column to the left, moves the sand source to the right
        self.sand_source.0 += 1;

        for (idx, row) in self.grid.iter_mut().enumerate(){
            if idx == limit {
                row.insert(0, Element::Rock);
            } else {
                row.insert(0, Element::Nothing)
            }
        }
    }

    fn border_right(&mut self) {
        let limit = self.grid.len() - 1;

        for (idx, row) in self.grid.iter_mut().enumerate() {
            if idx == limit {
                row.push(Element::Rock);
            } else {
                row.push(Element::Nothing);
            }
        }
    }

    fn sand_fall2(&mut self, current_position: Position) -> Result<Position, Outcome> {
        if current_position.0 < 0 {
            assert_eq!(current_position.0, -1);
            self.border_left();

            // Try again, but now -1 becomes 0
            return self.sand_fall2(Position(current_position.0 + 1, current_position.1));
        }

        if current_position.0 as usize >= self.grid[0].len() {
            self.border_right();

            // Try again
            return self.sand_fall2(Position(current_position.0, current_position.1));
        }

        let cave_element = &self.grid[current_position.1 as usize][current_position.0 as usize];

        if current_position.1 as usize == self.grid.len() - 2 {
            return if matches!(cave_element, Element::Sand | Element::Rock) {
                Err(Outcome::CellBusy)
            } else {
                self.grid[current_position.1 as usize][current_position.0 as usize] = Element::Sand;
                Ok(current_position.clone())
            }

        }

        if matches!(cave_element, Element::Sand | Element::Rock) {
            return Err(Outcome::CellBusy);
        }

        for new_direction in Direction::iter() {
            let fall_result = self.sand_fall2(current_position.translate(&new_direction));

            if fall_result.is_ok() {
                return fall_result;
            }
        }

        // Nothing was inserted on lower levels and here it's space -> insert sand here
        self.grid[current_position.1 as usize][current_position.0 as usize] = Element::Sand;

        Ok(current_position.clone())
    }

    pub(crate) fn pour_sand(&mut self) -> Result<Position, Outcome> {
        self.sand_fall(self.sand_source.clone())
    }

    pub(crate) fn pour_sand2(&mut self) -> Result<Position, Outcome> {
        self.sand_fall2(self.sand_source.clone())
    }

    fn push_level(&mut self, floor: Vec<Element>) {
        self.grid.push(floor)
    }

    pub(crate) fn push_air(&mut self) {
        self.push_level(vec![Element::Nothing; self.grid[0].len()])
    }

    pub(crate) fn push_floor(&mut self) {
        self.push_level(vec![Element::Rock; self.grid[0].len()])
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let grid_string = self
            .grid
            .iter()
            .map(|line| {
                line.iter()
                    .map(|element| char::from(element.clone()))
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", grid_string)
    }
}
