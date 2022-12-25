use crate::day17::commands::Commands;
use crate::day17::direction::Direction;
use crate::day17::direction::Position;
use crate::day17::pixel::Pixel;
use crate::day17::rock::Rock;
use itertools::Itertools;

pub(crate) struct Screen {
    pub(crate) rows: Vec<Vec<Pixel>>,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl Screen {
    pub(crate) fn new(width: usize) -> Self {
        Self {
            rows: vec![],
            height: 0,
            width,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn render(&self) -> String {
        self.rows
            .iter()
            .rev()
            .map(|row| {
                row.iter()
                    .map(|pixel| char::from(pixel))
                    .collect::<String>()
            })
            .intersperse("\n".to_owned())
            .collect()
    }

    pub(crate) fn empty_line(width: usize) -> Vec<Pixel> {
        vec![Pixel::Empty.clone(); width]
    }

    pub(crate) fn fall(&mut self, rock: &mut Rock, sequence: &mut Commands) {
        let required_height = rock.required_height();

        while self.rows.len() < required_height {
            self.rows.push(Self::empty_line(self.width))
        }

        let direction = sequence.next();
        // println!("{}", self.rows.len());
        // println!("1 - {:?} {:?}", direction, rock.position);
        if rock.can_move(self, direction) {
            rock.position += direction.into();
        }
        //println!("1 - {:?} {:?}", direction, rock.position);

        loop {
            //println!("1 - {:?} {:?}", &Direction::Down, rock.position);
            if !rock.can_move(self, &Direction::Down) {
                break;
            }

            rock.position += Position::from(&Direction::Down);
            //println!("2 - {:?} {:?}", &Direction::Down, rock.position);

            let direction = sequence.next();

            //println!("1 - {:?} {:?}", direction, rock.position);
            if rock.can_move(self, direction) {
                rock.position += Position::from(direction);
            }
            //println!("1 - {:?} {:?}", direction, rock.position);
        }

        rock.fix(self);
        self.height = self.height.max(rock.required_height());
        // println!("{}", self.render());
        // println!("================");
    }
}
