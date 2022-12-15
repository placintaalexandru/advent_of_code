use crate::day9::direction::Direction;
use num_traits::Signed;

#[derive(Eq, PartialEq, Clone)]
pub(crate) struct Knot<T: Signed + Eq + PartialEq> {
    x: T,
    y: T,
}

pub(crate) struct Rope<T: Signed + Eq + PartialEq> {
    knots: Vec<Knot<T>>,
}

impl<T: Signed + Eq + PartialEq> Rope<T> {
    pub(crate) fn len(&self) -> usize {
        self.knots.len()
    }
}

pub(crate) type GameType = i32;
pub(crate) type Game = Rope<GameType>;

impl Knot<GameType> {
    fn on_same_row(&self, other: &Self) -> bool {
        self.y == other.y
    }

    fn on_same_column(&self, other: &Self) -> bool {
        self.x == other.x
    }

    fn touch(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn moves(&self, other: &Self) -> Option<Vec<Direction>> {
        // Overlap
        if *self == *other {
            return None;
        }

        if self.on_same_row(other) {
            if self.x + 2 == other.x {
                return Some(vec![Direction::Right(1)]);
            } else if other.x + 2 == self.x {
                return Some(vec![Direction::Left(1)]);
            }
        }

        if self.on_same_column(other) {
            if self.y + 2 == other.y {
                return Some(vec![Direction::Down(1)]);
            } else if other.y + 2 == self.y {
                return Some(vec![Direction::Up(1)]);
            }
        }

        // Compose the diagonal moves
        if !self.touch(other) {
            let mut moves = vec![];

            if self.x < other.x {
                moves.push(Direction::Right(1));
            } else {
                moves.push(Direction::Left(1));
            }

            if self.y < other.y {
                moves.push(Direction::Down(1));
            } else {
                moves.push(Direction::Up(1));
            }

            return Some(moves);
        }

        None
    }

    fn apply_move(&mut self, change: Direction) {
        match change {
            Direction::Up(val) => self.y -= val,
            Direction::Down(val) => self.y += val,
            Direction::Left(val) => self.x -= val,
            Direction::Right(val) => self.x += val,
        }
    }
}

impl Rope<GameType> {
    pub(crate) fn new(n_knots: usize) -> Self {
        Self {
            knots: vec![Knot { x: 0, y: 0 }.clone(); n_knots],
        }
    }
}

impl Rope<GameType> {
    pub(crate) fn apply_move(&mut self, change: Direction) -> (GameType, GameType) {
        let n_knots = self.len();

        // Move head
        self.knots[0].apply_move(change);

        for knot_index in 1..n_knots {
            if let Some(knot_moves) = self.knots[knot_index].moves(&self.knots[knot_index - 1]) {
                knot_moves.into_iter().for_each(|knot_move| {
                    self.knots[knot_index].apply_move(knot_move);
                });
            } else {
                // If knot k does not move -> all knots [k + 1; N] won't move
                break;
            }
        }

        (self.knots[n_knots - 1].y, self.knots[n_knots - 1].x)
    }
}
