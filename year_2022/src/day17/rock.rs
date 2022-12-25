use crate::day17::direction::{Direction, Position};
use crate::day17::pixel::Pixel;
use crate::day17::screen::Screen;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter, Hash, Eq, PartialEq)]
pub(crate) enum RockType {
    // Keep track of the left most point
    Horizontal,

    // Keep track of the lowest mid point
    Cross,

    // Keep track of the right corner point
    LReverted,

    // Keep track of the base point
    Vertical,

    // Keep track of the left down corner
    Square,
}

pub(crate) struct Rock {
    pub(crate) r#type: RockType,
    pub(crate) length: usize,
    pub(crate) position: Position,
}

impl Rock {
    pub(crate) fn new(r#type: RockType, length: usize, position: Position) -> Self {
        Self {
            r#type,
            length,
            position,
        }
    }

    pub(crate) fn required_height(&self) -> usize {
        match self.r#type {
            RockType::Horizontal => self.position.0 as usize + 1,
            RockType::Cross => self.position.0 as usize + 2 * self.length + 1,
            RockType::LReverted | RockType::Vertical | RockType::Square => {
                self.position.0 as usize + self.length
            }
        }
    }

    pub(crate) fn can_move(&self, screen: &Screen, direction: &Direction) -> bool {
        match self.r#type {
            RockType::Horizontal => match *direction {
                Direction::Left => {
                    self.position.1 > 0
                        && screen.rows[self.position.0 as usize][self.position.1 as usize - 1]
                            == Pixel::Empty
                }
                Direction::Right => {
                    self.position.1 as usize + self.length < screen.width
                        && screen.rows[self.position.0 as usize]
                            [self.position.1 as usize + self.length]
                            == Pixel::Empty
                }
                Direction::Down => {
                    self.position.0 > 0
                        && (self.position.1 as usize..self.position.1 as usize + self.length)
                            .all(|x| screen.rows[self.position.0 as usize - 1][x] == Pixel::Empty)
                }
                _ => unreachable!(),
            },
            RockType::Cross => {
                match *direction {
                    Direction::Left => {
                        self.position.1 as usize > self.length
                            && screen.rows[self.position.0 as usize + self.length]
                                [self.position.1 as usize - self.length - 1]
                                == Pixel::Empty
                            && (self.position.0 as usize..self.position.0 as usize + self.length)
                                .all(|y| {
                                    screen.rows[y][self.position.1 as usize - 1] == Pixel::Empty
                                        && screen.rows[y + 1 + self.length]
                                            [self.position.1 as usize - 1]
                                            == Pixel::Empty
                                })
                    }
                    Direction::Right => {
                        self.position.1 as usize + self.length + 1 < screen.width
                            && screen.rows[self.position.0 as usize + self.length]
                                [self.position.1 as usize + self.length + 1]
                                == Pixel::Empty
                            && (self.position.0 as usize..self.position.0 as usize + self.length)
                                .all(|y| {
                                    screen.rows[y][self.position.1 as usize + 1] == Pixel::Empty
                                        && screen.rows[y + 1 + self.length]
                                            [self.position.1 as usize + 1]
                                            == Pixel::Empty
                                })
                    }
                    Direction::Down => {
                        self.position.0 > 0
                            && screen.rows[self.position.0 as usize - 1][self.position.1 as usize]
                                == Pixel::Empty
                            && (self.position.1 as usize - self.length..self.position.1 as usize)
                                .all(|x| {
                                    screen.rows[self.position.0 as usize + self.length - 1][x]
                                        == Pixel::Empty
                                })
                            && (self.position.1 as usize + 1
                                ..self.position.1 as usize + self.length + 1)
                                .all(|x| {
                                    screen.rows[self.position.0 as usize + (self.length - 1)][x]
                                        == Pixel::Empty
                                })
                    }
                    _ => unreachable!(),
                }
            }
            RockType::LReverted => match *direction {
                Direction::Left => {
                    self.position.1 as usize >= self.length
                        && screen.rows[self.position.0 as usize]
                            [self.position.1 as usize - self.length]
                            == Pixel::Empty
                        && (self.position.0 as usize + 1..self.position.0 as usize + self.length)
                            .all(|y| screen.rows[y][self.position.1 as usize - 1] == Pixel::Empty)
                }
                Direction::Right => {
                    self.position.1 as usize + 1 < screen.width
                        && (self.position.0 as usize..self.position.0 as usize + self.length)
                            .all(|y| screen.rows[y][self.position.1 as usize + 1] == Pixel::Empty)
                }
                Direction::Down => {
                    self.position.0 > 0
                        && (self.position.1 as usize - (self.length - 1)
                            ..self.position.1 as usize + 1)
                            .all(|x| screen.rows[self.position.0 as usize - 1][x] == Pixel::Empty)
                }
                _ => unreachable!(),
            },
            RockType::Vertical => match *direction {
                Direction::Left => {
                    self.position.1 > 0
                        && (self.position.0 as usize..self.position.0 as usize + self.length)
                            .all(|y| screen.rows[y][self.position.1 as usize - 1] == Pixel::Empty)
                }
                Direction::Right => {
                    self.position.1 as usize + 1 < screen.width
                        && (self.position.0 as usize..self.position.0 as usize + self.length)
                            .all(|y| screen.rows[y][self.position.1 as usize + 1] == Pixel::Empty)
                }
                Direction::Down => {
                    self.position.0 > 0
                        && screen.rows[self.position.0 as usize - 1][self.position.1 as usize]
                            == Pixel::Empty
                }
                _ => unreachable!(),
            },
            RockType::Square => match *direction {
                Direction::Left => {
                    self.position.1 > 0
                        && (self.position.0 as usize..self.position.0 as usize + self.length)
                            .all(|y| screen.rows[y][self.position.1 as usize - 1] == Pixel::Empty)
                }
                Direction::Right => {
                    self.position.1 as usize + self.length < screen.width
                        && (self.position.0 as usize..self.position.0 as usize + self.length).all(
                            |y| {
                                screen.rows[y][self.position.1 as usize + self.length]
                                    == Pixel::Empty
                            },
                        )
                }
                Direction::Down => {
                    self.position.0 > 0
                        && (self.position.1 as usize..self.position.1 as usize + self.length)
                            .all(|x| screen.rows[self.position.0 as usize - 1][x] == Pixel::Empty)
                }
                _ => unreachable!(),
            },
        }
    }

    pub(crate) fn fix(&self, screen: &mut Screen) {
        match self.r#type {
            RockType::Horizontal => {
                (self.position.1 as usize..self.position.1 as usize + self.length).for_each(
                    |part_position| {
                        screen.rows[self.position.0 as usize][part_position] = Pixel::Rock;
                    },
                );
            }
            RockType::Cross => {
                (self.position.1 as usize - self.length
                    ..self.position.1 as usize + self.length + 1)
                    .for_each(|part_position| {
                        screen.rows[self.position.0 as usize + self.length][part_position] =
                            Pixel::Rock
                    });
                (self.position.0 as usize..self.position.0 as usize + 1 + 2 * self.length)
                    .for_each(|part_position| {
                        screen.rows[part_position][self.position.1 as usize] = Pixel::Rock;
                    });
            }
            RockType::LReverted => {
                (self.position.1 as usize - (self.length - 1)..self.position.1 as usize + 1)
                    .for_each(|part_position| {
                        screen.rows[self.position.0 as usize][part_position] = Pixel::Rock;
                    });
                (self.position.0 as usize + 1..self.position.0 as usize + self.length).for_each(
                    |part_position| {
                        screen.rows[part_position][self.position.1 as usize] = Pixel::Rock;
                    },
                );
            }
            RockType::Vertical => {
                (self.position.0 as usize..self.position.0 as usize + self.length).for_each(
                    |part_position| {
                        screen.rows[part_position][self.position.1 as usize] = Pixel::Rock;
                    },
                );
            }
            RockType::Square => {
                (self.position.0 as usize..self.position.0 as usize + self.length).for_each(
                    |row| {
                        (self.position.1 as usize..self.position.1 as usize + self.length)
                            .for_each(|column| {
                                screen.rows[row][column] = Pixel::Rock;
                            });
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day17::direction::Position;

    #[test]
    fn simple_render() {
        let screen = Screen::new(7);
        assert_eq!(screen.render(), "");
    }

    #[test]
    fn simple_render_horizontal() {
        let mut screen = Screen::new(7);
        (0..1).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        Rock::new(RockType::Horizontal, 4, Position(0, 2)).fix(&mut screen);
        assert_eq!(screen.render(), "..####.");
    }

    #[test]
    fn complex_render_horizontal() {
        let mut screen = Screen::new(7);
        (0..4).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        Rock::new(RockType::Horizontal, 4, Position(3, 1)).fix(&mut screen);
        assert_eq!(screen.render(), ".####..\n.......\n.......\n.......");
    }

    #[test]
    fn simple_render_cross() {
        let mut screen = Screen::new(7);
        (0..3).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        Rock::new(RockType::Cross, 1, Position(0, 1)).fix(&mut screen);
        assert_eq!(screen.render(), ".#.....\n###....\n.#.....");
    }

    #[test]
    fn simple_render_l_reverted() {
        let mut screen = Screen::new(7);
        (0..4).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        Rock::new(RockType::LReverted, 3, Position(1, 3)).fix(&mut screen);
        assert_eq!(screen.render(), "...#...\n...#...\n.###...\n.......");
    }

    #[test]
    fn simple_render_vertical() {
        let mut screen = Screen::new(7);
        (0..5).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        Rock::new(RockType::Vertical, 3, Position(2, 3)).fix(&mut screen);
        assert_eq!(
            screen.render(),
            "...#...\n...#...\n...#...\n.......\n......."
        );
    }

    #[test]
    fn simple_render_bulk() {
        let mut screen = Screen::new(7);
        (0..5).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        Rock::new(RockType::Square, 3, Position(2, 3)).fix(&mut screen);
        assert_eq!(
            screen.render(),
            "...###.\n...###.\n...###.\n.......\n......."
        );
    }

    #[test]
    fn move_horizontal_piece() {
        let mut screen = Screen::new(7);
        (0..5).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        assert!(
            Rock::new(RockType::Horizontal, 3, Position(2, 3)).can_move(&screen, &Direction::Right)
        );
        assert!(!Rock::new(RockType::Horizontal, 3, Position(2, 4))
            .can_move(&screen, &Direction::Right));
        assert!(
            Rock::new(RockType::Horizontal, 3, Position(2, 3)).can_move(&screen, &Direction::Left)
        );
        assert!(
            !Rock::new(RockType::Horizontal, 3, Position(2, 0)).can_move(&screen, &Direction::Left)
        );
        assert!(
            Rock::new(RockType::Horizontal, 3, Position(2, 3)).can_move(&screen, &Direction::Down)
        );
        screen.rows[1][3] = Pixel::Rock;
        assert!(
            !Rock::new(RockType::Horizontal, 3, Position(2, 3)).can_move(&screen, &Direction::Down)
        );
    }

    #[test]
    fn move_vertical_piece() {
        let mut screen = Screen::new(7);
        (0..5).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));
        assert!(
            Rock::new(RockType::Vertical, 3, Position(2, 3)).can_move(&screen, &Direction::Right)
        );
        assert!(
            !Rock::new(RockType::Vertical, 3, Position(2, 6)).can_move(&screen, &Direction::Right)
        );
        assert!(
            Rock::new(RockType::Vertical, 3, Position(2, 3)).can_move(&screen, &Direction::Left)
        );
        assert!(
            !Rock::new(RockType::Vertical, 3, Position(2, 0)).can_move(&screen, &Direction::Left)
        );
        assert!(
            Rock::new(RockType::Vertical, 3, Position(2, 3)).can_move(&screen, &Direction::Down)
        );
        screen.rows[1][3] = Pixel::Rock;
        assert!(
            !Rock::new(RockType::Vertical, 3, Position(2, 3)).can_move(&screen, &Direction::Down)
        );
    }

    #[test]
    fn cross_piece_moves_down() {
        let mut screen = Screen::new(7);
        (0..6).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));

        screen.rows[0][1] = Pixel::Rock;

        assert!(!Rock::new(RockType::Cross, 2, Position(1, 1)).can_move(&screen, &Direction::Down));

        screen.rows[1][1] = Pixel::Rock;
        screen.rows[2][1] = Pixel::Rock;

        assert!(!Rock::new(RockType::Cross, 2, Position(1, 2)).can_move(&screen, &Direction::Down));
        assert!(!Rock::new(RockType::Cross, 1, Position(2, 2)).can_move(&screen, &Direction::Down));
    }

    #[test]
    fn cross_piece_moves_left() {
        let mut screen = Screen::new(7);
        (0..6).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));

        assert!(Rock::new(RockType::Cross, 2, Position(0, 3)).can_move(&screen, &Direction::Left));

        screen.rows[0][2] = Pixel::Rock;
        assert!(!Rock::new(RockType::Cross, 2, Position(0, 3)).can_move(&screen, &Direction::Left));

        screen.rows[0][2] = Pixel::Empty;
        screen.rows[3][2] = Pixel::Rock;

        assert!(!Rock::new(RockType::Cross, 2, Position(0, 3)).can_move(&screen, &Direction::Left));
    }

    #[test]
    fn cross_piece_moves_right() {
        let mut screen = Screen::new(7);
        (0..6).for_each(|_| screen.rows.push(Screen::empty_line(screen.width)));

        assert!(Rock::new(RockType::Cross, 2, Position(0, 2)).can_move(&screen, &Direction::Right));

        screen.rows[0][3] = Pixel::Rock;
        assert!(!Rock::new(RockType::Cross, 2, Position(0, 2)).can_move(&screen, &Direction::Right));

        screen.rows[0][3] = Pixel::Empty;
        screen.rows[3][3] = Pixel::Rock;

        assert!(!Rock::new(RockType::Cross, 2, Position(0, 2)).can_move(&screen, &Direction::Right));
    }
}
