use crate::day19::resource::Resource;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::str::FromStr;
use strum::IntoEnumIterator;

pub(crate) type Cost = HashMap<Resource, u32>;

#[derive(Debug)]
pub(crate) struct BluePrint {
    pub(crate) recipes: HashMap<Resource, Cost>,
    pub(crate) max_costs: Cost,
}

impl FromStr for BluePrint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((_, ore_robot_ore_cost, clay_robot_ore_cost, obsidian_robot_ore_cost, obsidian_robot_clay_cost, geode_robot_ore_cost, geode_robot_obsidian_cost)) = scan_fmt!(s, "Blueprint {d}: Each ore robot costs {d} ore. Each clay robot costs {d} ore. Each obsidian robot costs {d} ore and {d} clay. Each geode robot costs {d} ore and {d} obsidian.", u32, u32, u32, u32, u32, u32, u32) {
            let recipes = HashMap::from([(Resource::Ore, HashMap::from([(Resource::Ore, ore_robot_ore_cost)])), (Resource::Clay, HashMap::from([(Resource::Ore, clay_robot_ore_cost)])), (Resource::Obsidian, HashMap::from([(Resource::Ore, obsidian_robot_ore_cost), (Resource::Clay, obsidian_robot_clay_cost)])), (Resource::Geode, HashMap::from([(Resource::Ore, geode_robot_ore_cost), (Resource::Obsidian, geode_robot_obsidian_cost)]))]);
            let max_costs = Resource::iter().map(|resource| {
                let max_cost_resource = recipes.values().map(|robot_costs| *robot_costs.get(&resource).unwrap_or(&0)).max().unwrap();
                (resource, max_cost_resource)
            }).collect();
            return Ok(Self {
                recipes, max_costs
            });
        }

        Err(s.to_owned())
    }
}
