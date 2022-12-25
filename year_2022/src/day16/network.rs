use itertools::Itertools;
use priority_queue::PriorityQueue;
use scan_fmt::scan_fmt;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct Node {
    name: String,
    flow_rate: u32,
    neighbors: Vec<String>,
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((name, flow_rate, neighbors)) = scan_fmt!(
            s,
            "Valve {} has flow rate={d}; tunnels lead to valves {/((([A-Z]{2}, )+)*([A-Z]{2}))/}",
            String,
            u32,
            String
        ) {
            return Ok(Self {
                name,
                flow_rate,
                neighbors: neighbors
                    .split(',')
                    .map(|token| token.trim().to_owned())
                    .collect(),
            });
        }

        Err(s.to_owned())
    }
}

impl Node {
    pub(crate) fn released_pressure(&self, open_moment: u32, until: u32) -> usize {
        if open_moment > until {
            return 0;
        }

        self.flow_rate as usize * (until - open_moment) as usize
    }
}

#[derive(Debug)]
pub(crate) struct Network {
    pub(crate) graph: HashMap<String, Node>,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            graph: Default::default(),
        }
    }
}

impl Network {
    pub(crate) fn parse(path: &str) -> Self {
        let mut graph = Self::default();
        let buffer_reader = BufReader::new(File::open(path).unwrap());

        buffer_reader.lines().for_each(|line| {
            let node = Node::from_str(line.as_ref().unwrap()).unwrap();
            graph.add_node(node)
        });

        let flow_rates = <HashMap<String, u32>>::from_iter(
            graph
                .graph
                .iter()
                .map(|(name, node)| (name.clone(), node.flow_rate)),
        );

        graph.graph.iter_mut().for_each(|(_name, node)| {
            node.neighbors
                .sort_by(|a, b| flow_rates[a].cmp(&flow_rates[b]).reverse())
        });

        graph
    }

    fn djikstra(&self, source: &Node) -> HashMap<&String, u32> {
        let mut queue: PriorityQueue<(&String, u32), Reverse<u32>> = PriorityQueue::new();
        let mut result = HashMap::new();
        queue.push((&source.name, 0), Reverse(0));

        while !queue.is_empty() {
            let ((node, distance), _) = queue.pop().unwrap();

            for neighbor in &self.graph[node].neighbors {
                if !result.contains_key(&neighbor) {
                    result.insert(neighbor, distance + 1);
                    queue.push((neighbor, distance + 1), Reverse(distance + 1));
                    continue;
                }

                if distance + 1 > result[&neighbor] {
                    continue;
                }

                result.insert(neighbor, distance + 1);
                queue.push((neighbor, distance + 1), Reverse(distance + 1));
            }
        }

        for (name, node) in &self.graph {
            if *name != source.name && node.flow_rate == 0 {
                result.remove(name);
            }
        }

        result
    }

    fn distance_between_all_valves(&self) -> HashMap<&String, HashMap<&String, u32>> {
        let mut result = HashMap::new();

        for (name, node) in &self.graph {
            result.insert(name, self.djikstra(node));
        }

        result
    }

    fn dfs<'a>(
        &self,
        graph: &HashMap<&'a String, HashMap<&'a String, u32>>,
        opened_valves: &mut HashSet<&'a String>,
        current_node: &String,
        current_time: u32,
        limit_time: u32,
        result: &mut usize,
        acc: usize,
        node_subset: &HashSet<String>,
    ) {
        if current_time > limit_time {
            return;
        }

        if !node_subset.contains(current_node) {
            return;
        }

        if !graph.contains_key(&current_node) {
            return;
        }

        let neighbors = &graph[&current_node];

        for (neighbor, distance) in neighbors {
            if opened_valves.contains(neighbor) {
                continue;
            }

            let released_pressure =
                self.graph[*neighbor].released_pressure(current_time + distance, limit_time);

            if released_pressure > 0 {
                opened_valves.insert(*neighbor);
                self.dfs(
                    graph,
                    opened_valves,
                    *neighbor,
                    current_time + distance + 1,
                    limit_time,
                    result,
                    acc + released_pressure,
                    node_subset,
                );
                opened_valves.remove(*neighbor);
            }
        }

        *result = (*result).max(acc);
    }

    pub(crate) fn prioritize_valves(
        &self,
        start_node: &String,
        start_time: u32,
        until: u32,
        node_subset: &HashSet<String>,
    ) -> usize {
        let distance_between_all_valves = self
            .distance_between_all_valves()
            .into_iter()
            .filter(|(name, _distances)| {
                if **name == *start_node {
                    true
                } else {
                    self.graph[*name].flow_rate > 0
                }
            })
            .collect();

        let mut result = 0;
        self.dfs(
            &distance_between_all_valves,
            &mut HashSet::new(),
            start_node,
            start_time,
            until,
            &mut result,
            0,
            node_subset,
        );

        result
    }

    pub(crate) fn disjoint_search(
        &self,
        exclude: HashSet<String>,
        source: &String,
        start_time: u32,
        until: u32,
    ) -> usize {
        let mut result = 0;
        let scores = self
            .graph
            .keys()
            .filter(|key| !exclude.contains(*key) && self.graph[*key].flow_rate > 0)
            .cloned()
            .powerset()
            .filter(|subset| !subset.is_empty())
            .map(|subset| {
                let mut new_subset = HashSet::from_iter(subset.clone());
                new_subset.insert(source.clone());
                (
                    HashSet::from_iter(subset.into_iter()),
                    self.prioritize_valves(source, start_time, until, &new_subset),
                )
            })
            .collect::<Vec<_>>();

        for i in 0..scores.len() - 1 {
            let (my_set, my_score): &(HashSet<String>, usize) = &scores[i];

            for j in i + 1..scores.len() {
                let (elephant_set, elephant_score) = &scores[j];

                if my_set.intersection(elephant_set).count() > 0 {
                    continue;
                }

                result = result.max(*my_score + *elephant_score);
            }
        }

        result
    }

    fn add_node(&mut self, node: Node) {
        self.graph.insert(node.name.clone(), node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn released_pressure() {
        let node = Node {
            name: "".to_string(),
            flow_rate: 20,
            neighbors: vec![],
        };

        assert_eq!(node.released_pressure(2, 30), 20 * 28);
    }
}
