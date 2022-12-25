use crate::day21::jungle::Jungle;

mod expression;
mod jungle;

pub struct Day21;

impl Day21 {
    pub fn part_one() -> i64 {
        let jungle = Jungle::parse("src/day21/input");
        jungle.eval("root")
    }

    pub fn part_two() -> i64 {
        let jungle = Jungle::parse("src/day21/input");
        jungle.solve("root")
    }
}
