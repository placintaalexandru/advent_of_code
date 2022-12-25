use crate::day16::network::Network;
use std::collections::HashSet;

mod network;

pub struct Day16;

impl Day16 {
    pub fn part_one() -> usize {
        let network = Network::parse("src/day16/input");
        let subset = network.graph.keys().cloned().collect();
        network.prioritize_valves(&"AA".to_owned(), 1, 30, &subset)
    }

    pub fn part_two() -> usize {
        let network = Network::parse("src/day16/input");
        network.disjoint_search(
            HashSet::from_iter(["AA".to_owned()]),
            &"AA".to_owned(),
            1,
            26,
        )
    }
}
