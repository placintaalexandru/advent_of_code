use crate::day11::game::Destination;
use crate::day11::item::StartingItems;
use crate::day11::operation::Play;
use crate::day11::test::Action;

pub(crate) struct Monkey {
    starting_items: StartingItems,
    play: Play,
    action: Action,
}

impl Monkey {
    pub(crate) fn new(starting_items: StartingItems, play: Play, action: Action) -> Self {
        Self {
            starting_items,
            play,
            action,
        }
    }

    fn play(&self, item: usize) -> usize {
        self.play.apply(item)
    }

    fn get_bored(val: usize, worry_level: usize) -> usize {
        val / worry_level
    }

    pub(crate) fn round_step(
        &mut self,
        worry_level: usize,
        modulo_reducer: usize,
    ) -> Option<Vec<Destination>> {
        let mut result = vec![];

        while !self.starting_items.is_empty() {
            let stress_level = self.starting_items.pop_front().unwrap();
            let new_stress_level =
                Self::get_bored(self.play(stress_level), worry_level) % modulo_reducer;

            if (self.action)(new_stress_level) {
                result.push(Destination::new(self.action.1, new_stress_level));
            } else {
                result.push(Destination::new(self.action.2, new_stress_level));
            }
        }

        (!result.is_empty()).then_some(result)
    }

    pub(crate) fn push_back(&mut self, val: usize) {
        self.starting_items.push_back(val);
    }
}
