use crate::day19::bag::Bag;
use crate::day19::blueprint::{BluePrint, Cost};
use crate::day19::resource::Resource;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub(crate) type RobotInventory = Cost;

#[derive(Debug)]
pub(crate) struct RobotFactory {
    pub(crate) robots: RobotInventory,
    pub(crate) stock: Bag,
}

impl Default for RobotFactory {
    fn default() -> Self {
        let mut inventory = HashMap::from_iter(Resource::iter().map(|resource| (resource, 0)));
        *inventory.get_mut(&Resource::Ore).unwrap() = 1;

        Self {
            robots: inventory,
            stock: Default::default(),
        }
    }
}
impl RobotFactory {
    pub(crate) fn can_build(&self, blue_print: &BluePrint, robot_type: &Resource) -> bool {
        blue_print.recipes[robot_type]
            .iter()
            .all(|(resource, amount_required)| self.stock.resources[resource] >= *amount_required)
    }

    pub(crate) fn build(&mut self, blue_print: &BluePrint, robot_type: &Resource) {
        blue_print.recipes[robot_type]
            .iter()
            .for_each(|(resource, amount_required)| {
                (*self.stock.resources.get_mut(resource).unwrap()) -= *amount_required;
            });

        (*self.robots.get_mut(robot_type).unwrap()) += 1;
    }

    pub(crate) fn reverse_build(&mut self, blue_print: &BluePrint, robot_type: &Resource) {
        blue_print.recipes[robot_type]
            .iter()
            .for_each(|(resource, amount_required)| {
                (*self.stock.resources.get_mut(resource).unwrap()) += *amount_required;
            });

        (*self.robots.get_mut(robot_type).unwrap()) -= 1;
    }

    pub(crate) fn blue_print_value(
        &mut self,
        moment: u8,
        limit: u8,
        blueprint: &BluePrint,
        acc: usize,
        result: &mut usize,
    ) {
        if moment == limit {
            *result = (*result).max(acc);
            return;
        }

        if acc
            + self.robots[&Resource::Geode] as usize
            + (limit - moment) as usize * (limit + moment) as usize / 2
            < *result
        {
            return;
        }

        let buildable_robots = Resource::iter()
            .rev()
            .filter(|robot_type| {
                self.can_build(blueprint, robot_type) && {
                    match *robot_type {
                        Resource::Geode => true,
                        _ => self.stock.resources[robot_type] <= blueprint.max_costs[robot_type],
                    }
                }
            })
            .collect::<Vec<_>>();

        let harvested_resources = self
            .robots
            .iter()
            .map(|(resource, num_robots)| (resource.clone(), *num_robots))
            .collect::<HashMap<Resource, u32>>();

        harvested_resources
            .iter()
            .for_each(|(resource, num_robots)| {
                (*self.stock.resources.get_mut(&resource).unwrap()) += num_robots;
            });

        self.blue_print_value(
            moment + 1,
            limit,
            blueprint,
            acc + self.robots[&Resource::Geode] as usize,
            result,
        );

        buildable_robots.into_iter().for_each(|robot_type| {
            self.build(blueprint, &robot_type);

            self.blue_print_value(
                moment + 1,
                limit,
                blueprint,
                acc + self.robots[&Resource::Geode] as usize
                    - (robot_type == Resource::Geode) as usize,
                result,
            );

            // Try building anything else
            self.reverse_build(blueprint, &robot_type);
        });

        harvested_resources.iter().for_each(|(k, v)| {
            (*self.stock.resources.get_mut(&k).unwrap()) -= v;
        });
    }
}
